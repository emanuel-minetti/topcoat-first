use crate::app::_pages::current_user;
use topcoat::context::Cx;
use topcoat::l10n::locale;
use topcoat::router::error::RouterErrorExt;
use topcoat::router::{href, page};
use topcoat::view::{View, component, view};

#[page]
pub async fn page(cx: &Cx) -> topcoat::Result<impl View> {
    let user = current_user(cx)
        .await?
        .ok_or_redirect(href!(crate::app::_pages::login::page).resolve(cx))?;

    Ok(view! {
        hello(name: "Emu")
        <p>
            "The page is in: "
            <i>(locale(cx).to_string())</i>
        </p>
        <p>
            "The user is: "
            <i>(user.name)</i>
        </p>
    })
}

#[component]
async fn hello(cx: &Cx, name: &str) -> topcoat::Result<impl View> {
    Ok(view! {
        <h1>
            "Hello, "
            (name)
            "!"
        </h1>
    })
}
