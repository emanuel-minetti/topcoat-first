use crate::components::navigation_menu::{
    NavigationMenuLinkVariant, navigation_menu, navigation_menu_item, navigation_menu_link,
};
use crate::components::select::select;
use crate::imprint::imprint;
use crate::{home, t};
use topcoat::asset::{Asset, asset};
use topcoat::context::Cx;
use topcoat::l10n::locale;
use topcoat::router::href;
use topcoat::view::{View, attributes, class, component, view};

const LOGO: Asset = asset!("assets/logo.svg");

#[component]
pub async fn header(cx: &Cx) -> topcoat::Result<impl View> {
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
                attrs: attributes! { class=(class!("")) },
                select(
                    attrs: attributes!(
                        {
                            name="language"
                        }
                    ),
                    <option value="en" slected=(locale(cx).to_string() == "en")>
                        (t!(english))
                    </option>
                    <option value="de">(t!(german))</option>
                )
            )
        )
    })
}
