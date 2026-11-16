pub mod home;
pub(crate) mod imprint;
pub mod login;

use crate::app::components::header::header;
use crate::app::models::session::Session;
use crate::app::models::user::User;
use crate::db::db;
use topcoat::Result;
use topcoat::asset::{Asset, asset};
use topcoat::context::Cx;
use topcoat::font::Font;
use topcoat::font::fontsource::fontsource_font;
use topcoat::l10n::locale;
use topcoat::router::{Slot, layout};
use topcoat::session::TokenHash;
use topcoat::view::{View, view};
use topcoat::{session, tailwind};

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
                <main><div class="container mx-auto px-4">(slot)</div></main>
            </body>
        </html>
    })
}

pub async fn current_user(cx: &Cx) -> Result<Option<User>> {
    let Some(hash) = session::token_hash(cx).await? else {
        return Ok(None);
    };
    load_session_user(cx, &hash).await
}

async fn load_session_user(cx: &Cx, token_hash: &TokenHash) -> Result<Option<User>> {
    let session: Session = Session::get_by_token_hash(&mut db(cx), token_hash.to_vec()).await?;
    let now = jiff::Timestamp::now();
    if session.expires_at < now {
        return Ok(None);
    }
    if session
        .revoked_at
        .is_some_and(|revoked_at| revoked_at < now)
    {
        return Ok(None);
    }

    let user = User::get_by_id(&mut db(cx), session.user_id).await?;

    Ok(Some(user.clone()))
}
