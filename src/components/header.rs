use crate::components::navigation_menu::{
    NavigationMenuLinkVariant, navigation_menu, navigation_menu_item, navigation_menu_link,
};
use crate::imprint::imprint;
use crate::{home, t};
use topcoat::context::Cx;
use topcoat::router::href;
use topcoat::view::{View, attributes, component, view};

#[component]
pub async fn header(cx: &Cx) -> topcoat::Result<impl View> {
    Ok(view! {
        navigation_menu(
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
        )
    })
}
