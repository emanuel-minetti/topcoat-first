mod components;
mod imprint;

use crate::components::navigation_menu::{
    navigation_menu, navigation_menu_item, navigation_menu_link, NavigationMenuLinkVariant,
};
use topcoat::asset::{asset, Asset, AssetBundle, RouterBuilderAssetExt};
use topcoat::context::Cx;
use topcoat::font::fontsource::fontsource_font;
use topcoat::font::Font;
use topcoat::l10n::{accept_language, locale, negotiate, Locale};
use topcoat::router::response::Response;
use topcoat::router::{href, layer, layout, Body, Next, RouterBuilderDiscoverExt, Slot};
use topcoat::view::attributes;
use topcoat::{
    router::{module_router, page},
    tailwind,
    view::{component, view, View},
    Result,
};
use topcoat::l10n::fluent::messages;

const LOCALES: [Locale; 2] = [locale!("de"), locale!("en")];
messages!("l10n/{locale}.ftl");


#[tokio::main]
async fn main() {
    topcoat::start(
        module_router!()
            .discover()
            .assets(AssetBundle::load().unwrap())
            .build(),
    )
    .await
    .unwrap();
}

#[layer]
async fn l10n_layer(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let locale = negotiate(accept_language(cx), &LOCALES)
        .cloned()
        .unwrap_or(LOCALES[0].clone());
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
                topcoat::dev::script()
            </head>
            <body>
                navigation_menu(
                    navigation_menu_item(
                        navigation_menu_link(
                            variant: NavigationMenuLinkVariant::Trigger,
                            active: href!(home).is_current(cx),
                            attrs: attributes! { href=(href!(home)) },
                            "Home"
                        )
                    )
                    navigation_menu_item(
                        navigation_menu_link(
                            variant: NavigationMenuLinkVariant::Trigger,
                            active: href!(imprint::imprint).is_current(cx),
                            attrs: attributes! { href=(href!(imprint::imprint)) },
                            (t!(imprint))
                        )
                    )
                )
                <main>(slot)</main>
            </body>
        </html>
    })
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        hello(name: "World")
        <p>"The page is in: " (locale(cx))</p>
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
