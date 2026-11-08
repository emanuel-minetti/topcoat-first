use std::convert::Into;
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
use topcoat::icon::{icon, iconify, IconData};

const LOGO: Asset = asset!("assets/logo.svg");
const GERMAN_FLAG: IconData = iconify::iconify_icon!("openmoji:flag-germany");
const ENGLISH_FLAG: IconData = iconify::iconify_icon!("openmoji:flag-united-kingdom");

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
                lang_select()
                // select(
                //     attrs: attributes! {
                //             @change=$(|e: Event| {
                //                 lang.set(e.target.value);
                //         })
                //         }
                //     ,
                //     <option value="en" selected=(locale.to_string() == "en")>
                //         (t!(english))
                //         icon(data: ENGLISH_FLAG, label: "English Flag")
                //     </option>
                //     <option value="de" selected=(locale.to_string() == "de")>
                //         (t!(german))
                //     </option>
                // )
                set_lang_cookie(lang: lang.get())
            )
        )
    })
}

#[shard]
async fn set_lang_cookie(cx: &Cx, lang: String) -> Result<impl View> {
    if lang != topcoat::l10n::locale(cx).to_string() && [ "en", "de" ].contains(&lang.to_lowercase().as_str()) {
        let jar = cookies(cx);
        jar.add(cookie!("lang" = lang.to_owned()));
        return Err::<(), topcoat::Error>(redirect("/").into());
    }
    Ok(())
}

#[component]
async fn lang_select(cx: &Cx) -> Result<impl View> {
    let lang = locale(cx);
    let lang_str = lang.to_string();
    let lang_icon = if lang_str == "de" {
        GERMAN_FLAG} else { ENGLISH_FLAG  };

    Ok(view!(<span class="mr-15 text-2xl align-middle inline-block">icon(data: lang_icon, label: "language selector")</span>))
}