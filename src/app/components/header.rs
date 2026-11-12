use crate::app::components::navigation_menu::{navigation_menu, navigation_menu_item, navigation_menu_link, NavigationMenuLinkVariant};
use crate::app::components::select::select;
use crate::app::home;
use crate::app::imprint::imprint;
use crate::app::t;
use std::convert::Into;
use topcoat::asset::{asset, Asset};
use topcoat::context::Cx;
use topcoat::cookie::{cookies, Cookies};
use topcoat::icon::{icon, iconify, IconData};
use topcoat::l10n::locale;
use topcoat::router::error::redirect;
use topcoat::router::href;
use topcoat::runtime::{shard, signal, Event};
use topcoat::view::{attributes, class, component, view, View};
use topcoat::Result;

const LOGO: Asset = asset!("assets/logo.svg");
const GERMAN_FLAG: IconData = iconify::iconify_icon!("openmoji:flag-germany");
const ENGLISH_FLAG: IconData = iconify::iconify_icon!("openmoji:flag-united-kingdom");

#[component]
pub async fn header(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        navigation_menu(
            attrs: attributes! { class="list-none bg-(--sidebar) pt-3 pb-7 mb-13" },
            navigation_menu_item(
                navigation_menu_link(
                    variant: NavigationMenuLinkVariant::Trigger,
                    active: href!(home).is_current(cx),
                    attrs: attributes! { href=(href!(home)) class="mr-10" },
                    <img class=(class!("h-9")) src=(LOGO) alt="the logo">
                    "Topcoat First"
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
            )
        )
    })
}

#[component]
async fn lang_select(cx: &Cx) -> Result<impl View> {
    let locale = locale(cx);
    let locale_str = locale.to_string();
    let locale_icon = if locale_str == "de" {
        GERMAN_FLAG
    } else {
        ENGLISH_FLAG
    };
    let show_lang_select = signal(cx, || false);
    let lang_signal = signal(cx, String::new);

    Ok(view!(
        <span
            class="mr-15 text-2xl align-middle inline-block"
            :hidden=$(show_lang_select.get())
            @click=$(|_e| {
                show_lang_select.set(!show_lang_select.get());
            })
        >
            icon(data: locale_icon, label: "language selector")
        </span>
        <span :hidden=$(!show_lang_select.get())>
            select(
                attrs: attributes! {
                    @change=$(|e: Event| {
                        lang_signal.set(e.target.value);
                    })
                },
                <option
                    value="en"
                    selected=(locale.to_string() == "en")
                    @click=$(|_e: Event| {
                        show_lang_select.set(!show_lang_select.get());
                    })
                >
                    (t!(english))
                </option>
                <option
                    value="de"
                    selected=(locale.to_string() == "de")
                    @click=$(|_e: Event| {
                        show_lang_select.set(!show_lang_select.get());
                    })
                >
                    (t!(german))
                </option>
            )
        </span>
        set_lang_cookie(lang: lang_signal.get())
    ))
}

#[shard]
async fn set_lang_cookie(cx: &Cx, lang: String) -> Result<impl View> {
    if lang != topcoat::l10n::locale(cx).to_string()
        && ["en", "de"].contains(&lang.to_lowercase().as_str())
    {
        cookies(cx).add(("lang", lang.to_owned()));
        return Err::<(), topcoat::Error>(redirect("/").into());
    }
    Ok(())
}
