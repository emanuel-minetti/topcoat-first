pub (crate) mod components;
pub (crate) mod imprint;
pub mod models;

use crate::app::components::header::header;
use topcoat::asset::{asset, Asset, AssetBundle, RouterBuilderAssetExt};
use topcoat::context::{BaseUrl, Cx};
use topcoat::cookie::{cookies, CookieLayer, Cookies};
use topcoat::font::fontsource::fontsource_font;
use topcoat::font::Font;
use topcoat::l10n::fluent::messages;
use topcoat::l10n::{accept_language, locale, negotiate, Locale};
use topcoat::router::response::Response;
use topcoat::router::{layer, layout, module_router, page, Body, Compression, Next, OriginPolicy, RouterBuilderDiscoverExt, Slot};
use topcoat::runtime::{PrefetchMode, RuntimeLayer};
use topcoat::tailwind;
use topcoat::view::{component, view, SuspenseMode, View};
use topcoat::Result;

const LOCALES: [Locale; 2] = [locale!("de"), locale!("en")];

messages!("l10n/{locale}.ftl");

pub fn router() -> topcoat::router::RouterBuilder {
    module_router!()
        .with(Compression::default().brotli(false))
        .with(OriginPolicy::default())
        .with(BaseUrl::new("http://localhost:3000"))
        .with(PrefetchMode::default())
        .with(SuspenseMode::default())
        .discover()
        .assets(AssetBundle::load().unwrap())
        .layer(CookieLayer::new())
        .layer(RuntimeLayer::new())
}

#[layer]
async fn l10n_layer(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let mut locale = negotiate(accept_language(cx), &LOCALES)
        .cloned()
        .unwrap_or(LOCALES[0].clone());
    let jar = cookies(cx);
    let cookie_locale_option = jar.get("lang");
    if (cookie_locale_option.is_some())
        && (cookie_locale_option.clone().unwrap().value() != locale.to_string()
            && ["de", "en"].contains(&cookie_locale_option.clone().unwrap().value()))
    {
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
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                topcoat::font::link(font: GEIST)
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
        hello(name: "Emu")
        <p>
            "The page is in: "
            <i>(locale(cx).to_string())</i>
        </p>
    })
}

#[component]
async fn hello(cx: &Cx, name: &str) -> Result<impl View> {
    Ok(view! {
        <h1>
            "Hello, "
            (name)
            "!"
        </h1>
    })
}
