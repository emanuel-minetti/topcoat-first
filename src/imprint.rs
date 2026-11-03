use topcoat::router::page;
use topcoat::view::{view, View};

#[page]
pub async fn imprint() -> topcoat::Result<impl View> {
    Ok(view! { <h1>"Imprint"</h1> })
}