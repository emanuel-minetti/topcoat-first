pub(crate) mod imprint;
pub mod home;

use crate::app::components::header::header;
use topcoat::asset::{asset, Asset};
use topcoat::context::Cx;
use topcoat::font::fontsource::fontsource_font;
use topcoat::font::Font;
use topcoat::l10n::locale;
use topcoat::router::{layout, Slot};
use topcoat::tailwind;
use topcoat::view::{view, View};

#[layout]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> topcoat::Result<impl View> {
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
