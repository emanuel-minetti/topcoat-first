mod components;
mod imprint;
mod models;

use crate::components::header::header;
use topcoat::asset::{Asset, AssetBundle, RouterBuilderAssetExt, asset};
use topcoat::context::Cx;
use topcoat::font::Font;
use topcoat::font::fontsource::fontsource_font;
use topcoat::l10n::fluent::messages;
use topcoat::l10n::{Locale, accept_language, locale, negotiate};
use topcoat::router::response::Response;
use topcoat::router::{Body, Next, RouterBuilderDiscoverExt, Slot, layer, layout};
use topcoat::{
    Result,
    router::{module_router, page},
    tailwind,
    view::{View, component, view},
};
use topcoat::cookie::{cookies, Cookies, RouterBuilderCookieExt};
use topcoat::runtime::RouterBuilderRuntimeExt;

const LOCALES: [Locale; 2] = [locale!("de"), locale!("en")];
messages!("l10n/{locale}.ftl");

#[tokio::main]
async fn main() {
    topcoat::start(
        module_router!()
            .discover()
            .assets(AssetBundle::load().unwrap())
            .cookies()
            .runtime()
            .build(),
    )
    .await
    .unwrap();
}

#[layer]
async fn l10n_layer(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let mut locale = negotiate(accept_language(cx), &LOCALES)
        .cloned()
        .unwrap_or(LOCALES[0].clone());
    let jar = cookies(cx);
    let cookie_locale_option = jar.get("lang");
    if (cookie_locale_option.is_some()) && (cookie_locale_option.clone().unwrap().value() != locale.to_string()) {
        locale = cookie_locale_option.unwrap().value().parse::<Locale>()?;
    }
    next.run(&cx.with(locale), body).await
}

#[layout]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    const LOGO: Asset = asset!("assets/logo.svg");
    const GEIST: Font = fontsource_font!(GEIST, host: Asset);

    Ok(view! {
        <!DOCTYPE html>
        <html lang=(locale(cx).id()) dir=(locale(cx).id().direction())>
            <head>
                <title>"Topcoat First"</title>
                <link rel="icon" href=(LOGO) type="image/svg+xml">
                topcoat::font::link(font: GEIST)
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                topcoat::runtime::script()
                topcoat::dev::script()
            </head>
            <body>
                header()
                <main>(slot)</main>
            </body>
        </html>
    })
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        hello(name: "World")
        <p>
            "The page is in: "
            <i>(locale(cx).to_string())</i>
        </p>
    })
}

#[component]
async fn hello(name: &str) -> Result<impl View> {
    Ok(view! {
        <h1>
            "Hello, "
            (name)
            "!"
        </h1>
    })
}
