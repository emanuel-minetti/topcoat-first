mod imprint;
mod components;

use topcoat::{Result, router::{module_router, page}, view::{View, component, view}, tailwind};
use topcoat::asset::{asset, Asset, AssetBundle, RouterBuilderAssetExt};
use topcoat::context::Cx;
use topcoat::font::Font;
use topcoat::font::fontsource::fontsource_font;
use topcoat::router::{href, layout, RouterBuilderDiscoverExt, Slot};
use topcoat::view::attributes;
use crate::components::navigation_menu::{navigation_menu, navigation_menu_item, navigation_menu_link, NavigationMenuLinkVariant};

#[tokio::main]
async fn main() {
    topcoat::start(module_router!()
        .discover()
        .assets(AssetBundle::load().unwrap())
        .build()).await.unwrap();
}

#[layout]
async fn root_layout(cx: &Cx,slot: Slot<'_>) -> Result<impl View> {
    const LOGO: Asset = asset!("assets/logo.svg");
    const GEIST: Font = fontsource_font!(GEIST, host: Asset);

    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Topcoat First"</title>
                <link rel="icon" href=(LOGO) type="image/svg+xml">
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                topcoat::font::link(font: GEIST)
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
                            "Imprint"
                        )
                    )
                )
                <main>(slot)</main>
            </body>
        </html>
    })
}

#[page]
async fn home() -> topcoat::Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        hello(name: "World")
    })
}

#[component]
async fn hello(name: &str) -> topcoat::Result<impl View> {
    Ok(view! {
        <h1>
            "Hello, "
            (name)
            "!"
        </h1>
    })
}
