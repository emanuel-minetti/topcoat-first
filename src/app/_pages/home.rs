use topcoat::context::Cx;
use topcoat::l10n::locale;
use topcoat::router::page;
use topcoat::view::{component, view, View};

#[page]
pub async fn home(cx: &Cx) -> topcoat::Result<impl View> {
    Ok(view! {
        hello(name: "Emu")
        <p>
            "The page is in: "
            <i>(locale(cx).to_string())</i>
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