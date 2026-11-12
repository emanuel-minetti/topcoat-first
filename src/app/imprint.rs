use topcoat::context::Cx;
use topcoat::router::page;
use topcoat::view::{View, view};

#[page]
pub async fn imprint(cx: &Cx) -> topcoat::Result<impl View> {
    Ok(view! { <h1>"Imprint"</h1> })
}
