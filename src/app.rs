pub (crate) mod components;
pub mod models;
pub mod _pages;

use topcoat::asset::{AssetBundle, RouterBuilderAssetExt};
use topcoat::context::{BaseUrl, Cx};
use topcoat::cookie::{cookies, CookieLayer, Cookies};
use topcoat::l10n::fluent::messages;
use topcoat::l10n::{accept_language, locale, negotiate, Locale};
use topcoat::router::response::Response;
use topcoat::router::{layer, module_router, page, Body, Compression, Next, OriginPolicy, RouterBuilderDiscoverExt};
use topcoat::runtime::{PrefetchMode, RuntimeLayer};
use topcoat::view::{SuspenseMode, View};
use topcoat::Result;
use topcoat::router::error::redirect;

const LOCALES: [Locale; 2] = [locale!("de"), locale!("en")];

messages!("l10n/{locale}.ftl");

pub fn router() -> topcoat::router::RouterBuilder {
    module_router!()
        .with(Compression::default().brotli(false))
        .with(OriginPolicy::default())
        .with(BaseUrl::new("http://localhost:3000"))
        .with(PrefetchMode::default())
        .with(SuspenseMode::default())
        .discover()
        .assets(AssetBundle::load().unwrap())
        .layer(CookieLayer::new())
        .layer(RuntimeLayer::new())
}

#[layer]
async fn l10n_layer(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let mut locale = negotiate(accept_language(cx), &LOCALES)
        .cloned()
        .unwrap_or(LOCALES[0].clone());
    let jar = cookies(cx);
    let cookie_locale_option = jar.get("lang");
    if (cookie_locale_option.is_some())
        && (cookie_locale_option.clone().unwrap().value() != locale.to_string()
            && ["de", "en"].contains(&cookie_locale_option.clone().unwrap().value()))
    {
        locale = cookie_locale_option.unwrap().value().parse::<Locale>()?;
    }
    next.run(&cx.with(locale), body).await
}



#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    Err::<(), topcoat::Error>(redirect("/home").into())
}
