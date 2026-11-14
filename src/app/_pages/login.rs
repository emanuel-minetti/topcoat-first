use crate::app::t;

use topcoat::router::{href, page, route};
use topcoat::Result;
use topcoat::view::{View, view};
use topcoat::context::Cx;
use topcoat::router::error::{see_other, service_unavailable, unauthorized, SeeOther};
use topcoat::session;
use crate::app::models::user::User;

#[page]
pub async fn page(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <h1>"Login"</h1>
        (t!(login_intro))
        ":"
        <form
            id="login-form"
            class="bg-white text-gray-500 max-w-[340px] mx-4 p-6 text-left text-sm rounded-lg border border-gray-300/60"
            method="POST"
            action=(href!(login))
        >
            <label class="font-medium" for="username">(t!(login_username))</label>
            <input
                id="username"
                class="w-full border mt-1.5 mb-4 border-gray-500/30 outline-none rounded py-2.5 px-3"
                type="string"
                placeholder=(t!(login_username))
                required="true"
            >
            <label class="font-medium" for="password">(t!(login_password))</label>
            <input
                id="password"
                class="w-full border mt-1.5 mb-4 border-gray-500/30 outline-none rounded py-2.5 px-3"
                type="password"
                placeholder=(t!(login_password))
                required="true"
            >
            <button
                type="submit"
                class="my-3 bg-primary py-2 px-5 rounded text-white font-medium"
            >
                (t!(login_login))
            </button>
        </form>
    })
}

#[route(POST)]
async fn login(cx: &Cx ) -> Result<SeeOther> {
    //let user_input =
    let user = verify_credentials(cx).await?;
    let session = session::start(cx).await?;
    persist_session(cx, user, session).await?;

    Ok(see_other(href!(crate::app::home).resolve(cx)))
}

async fn verify_credentials(_cx: &Cx) -> Result<User> {
    Err(unauthorized().into())
}

async fn persist_session(_cx: &Cx, _user: User, _session: session::Session) -> Result<()> {
    Err(service_unavailable(10).into())
}