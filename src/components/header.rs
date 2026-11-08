use crate::components::navigation_menu::{
    NavigationMenuLinkVariant, navigation_menu, navigation_menu_item, navigation_menu_link,
};
use crate::components::select::select;
use crate::imprint::imprint;
use crate::{home, t};
use topcoat::asset::{Asset, asset};
use topcoat::context::Cx;
use topcoat::cookie::{cookie, cookies, Cookies};
use topcoat::l10n::{locale};
use topcoat::router::{href};
use topcoat::router::error::{redirect};
use topcoat::runtime::{shard, signal, Event};
use topcoat::view::{View, attributes, class, view, component};
use topcoat::{Result};

const LOGO: Asset = asset!("assets/logo.svg");

#[shard]
async fn set_lang_cookie(cx: &Cx, lang: String) -> Result<impl View> {
    if lang != locale(cx).to_string() && [ "en", "de" ].contains(&lang.to_lowercase().as_str()) {
            let jar = cookies(cx);
            jar.add(cookie!("lang" = lang.to_owned()));
            return Err::<(), topcoat::Error>(redirect("/").into());
    }
    Ok(())
}

#[component]
pub async fn header(cx: &Cx) -> Result<impl View> {
    let locale = locale(cx);
    let lang = signal(cx, String::new);

    Ok(view! {
        navigation_menu(
            attrs: attributes! { class="list-none bg-(--sidebar) pt-3 pb-7 mb-13" },
            navigation_menu_item(
                navigation_menu_link(
                    variant: NavigationMenuLinkVariant::Trigger,
                    active: href!(home).is_current(cx),
                    attrs: attributes! { href=(href!(home)) class="mr-10" },
                    <img class=(class!("h-9")) src=(LOGO) alt="the logo">
                    "Topcaot First"
                )
            )
            navigation_menu_item(
                navigation_menu_link(
                    variant: NavigationMenuLinkVariant::Trigger,
                    active: href!(home).is_current(cx),
                    attrs: attributes! { href=(href!(home)) },
                    (t!(home))
                )
            )
            navigation_menu_item(
                navigation_menu_link(
                    variant: NavigationMenuLinkVariant::Trigger,
                    active: href!(imprint).is_current(cx),
                    attrs: attributes! { href=(href!(imprint)) },
                    (t!(imprint))
                )
            )
            navigation_menu_item(
                attrs: attributes! { class=(class!("ml-auto")) },
                select(
                    attrs: attributes! {
                            @change=$(|e: Event| {
                                lang.set(e.target.value);
                        })
                        }
                    ,
                    <option value="en" selected=(locale.to_string() == "en")>
                        (t!(english))
                    </option>
                    <option value="de" selected=(locale.to_string() == "de")>
                        (t!(german))
                    </option>
                )
                set_lang_cookie(lang: lang.get())
            )
        )
    })
}
