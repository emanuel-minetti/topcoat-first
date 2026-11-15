pub mod home;
pub(crate) mod imprint;
pub mod login;

use crate::app::components::header::header;
use topcoat::asset::{Asset, asset};
use topcoat::context::Cx;
use topcoat::font::Font;
use topcoat::font::fontsource::fontsource_font;
use topcoat::l10n::locale;
use topcoat::router::{Slot, layout};
use topcoat::tailwind;
use topcoat::view::{View, view};

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
                <main><div class="container mx-auto px-4">(slot)</div></main>
            </body>
        </html>
    })
}
