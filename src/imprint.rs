use topcoat::router::page;
use topcoat::view::{view, View};

#[page]
async fn home() -> topcoat::Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Imprint"</title>
                topcoat::dev::script()
            </head>
            <body>
                <h1>"Imprint"</h1>
            </body>
        </html>
    })
}