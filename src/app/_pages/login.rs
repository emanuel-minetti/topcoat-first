use jiff::Timestamp;
use crate::app::t;

use crate::app::models::user::User;
use topcoat::Result;
use topcoat::context::{try_context, Cx};
use topcoat::internal::serde::Deserialize;
use topcoat::l10n::Translation;
use topcoat::router::error::{SeeOther, see_other, internal_server_error, rewrite};
use topcoat::router::{href, page, route, Body, Method};
use topcoat::router::content::Form;
use topcoat::session;
use topcoat::view::{View, element_id, view, attributes};
use crate::app::models::session::Session;
use crate::components::field::*;
use crate::components::input::input;
use crate::db::db;

struct LoginFailed {
    username: String,
    username_error: Option<LoginError>,
    password_error: Option<LoginError>,
}

#[derive(Clone, Copy)]
enum LoginError {
    UsernameMissing,
    PasswordMissing,
    InvalidCredentials,
}

impl LoginError {
    fn message(self) -> Translation<'static> {
        match self {
            LoginError::UsernameMissing => t!(login_error_username_missing),
            LoginError::PasswordMissing => t!(login_error_password_missing),
            LoginError::InvalidCredentials => t!(login_error_invalid),
        }
    }
}

#[page]
pub async fn page(cx: &Cx) -> Result<impl View> {
    let failed = try_context::<LoginFailed>(cx);
    let username = failed.map(|failed| failed.username.as_str());
    let username_error = failed.and_then(|failed| failed.username_error);
    let password_error = failed.and_then(|failed| failed.password_error);

    let form_id = element_id(cx);
    let username_id = element_id(cx);
    let username_error_id = element_id(cx);
    let password_id = element_id(cx);
    let password_error_id = element_id(cx);

    Ok(view! {
        <h1>"Login"</h1>
        (t!(login_intro))
        ":"
        <form
            id=(&form_id)
            class="bg-white text-gray-500 max-w-[340px] mx-4 p-6 text-left text-sm rounded-lg border border-gray-300/60"
            method="post"
            action=(href!(submit))
            novalidate=""
        >
            field_group(
                field(
                    field_label(
                        attrs: attributes! { for=(&username_id) },
                        (t!(login_username))
                    )
                    input(
                        attrs: attributes! {
                            id=(&username_id)
                            class="w-full border mt-1.5 mb-4 border-gray-500/30 outline-none rounded py-2.5 px-3"
                            type="string"
                            name="username"
                            autocomplete="username"
                            value=(username)
                            placeholder=(t!(login_username))
                            aria-invalid=(username_error.is_some().then_some("true"))
                            aria-descibedby=(username_error.is_some().then_some(&username_error_id))
                        }
                    )
                    if let Some(error) = username_error {
                        field_error(
                            attrs: attributes! { id=(&username_error_id) },
                            (error.message())
                        )
                    }
                )
                field(
                    field_label(
                        attrs: attributes! { for=(&password_id) },
                        (t!(login_password))
                    )
                    input(
                        attrs: attributes! {
                            id=(&password_id)
                            class="w-full border mt-1.5 mb-4 border-gray-500/30 outline-none rounded py-2.5 px-3"
                            type="password"
                            name="password"
                            autocomplete="current-password"
                            placeholder=(t!(login_password))
                            aria-invalid=(password_error.is_some().then_some("true"))
                            aria-descibedby=(password_error.is_some().then_some(&password_error_id))
                        }
                    )
                    if let Some(error) = password_error {
                        field_error(
                            attrs: attributes! { id=(&password_error_id) },
                            (error.message())
                        )
                    }
                )
            )
            <button
                type="submit"
                class="my-3 bg-primary py-2 px-5 rounded text-white font-medium"
            >
                (t!(login_login))
            </button>
        </form>
    })
}

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[route(POST)]
async fn submit(cx: &Cx, Form(credentials): Form<Credentials>) -> Result<SeeOther> {
    let Credentials { username, password } = credentials;
    let mut username_error = username.trim().is_empty().then_some(LoginError::UsernameMissing);
    let password_error = password.trim().is_empty().then_some(LoginError::PasswordMissing);

    if username_error.is_none() && password_error.is_none() {
        if let Some(user) = User::authenticate(cx, &username, password).await? {
            persist_session(cx, &user).await?;
            return Ok(see_other(href!(crate::app::home).resolve(cx)));
        }
        else {
            username_error = Some(LoginError::InvalidCredentials);
        }
    }

    // Rewrite the POST to keep the submitted email and errors in the request context.
    Err(rewrite(href!(page).resolve(cx), Body::empty())
            .method(Method::GET)
            .with(LoginFailed {
                username,
                username_error,
                password_error,
            })
            .into())
}


async fn persist_session(cx: &Cx, user: &User) -> Result<()> {
    let session = session::start(cx).await?;
    let expires_at = Timestamp::try_from(session.expires_at).map_err(internal_server_error)?;

    toasty::create!(Session {
        token_hash: session.token_hash.to_vec(),
        user_id: user.id,
        expires_at,
    })
        .exec(&mut db(cx))
        .await.expect("Failed to persist session");

    Ok(())
}
