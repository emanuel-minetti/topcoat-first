use topcoat::router::page;
use topcoat::view::{View, view};

#[page]
pub async fn imprint() -> topcoat::Result<impl View> {
    Ok(view! { <h1>"Imprint"</h1> })
}
