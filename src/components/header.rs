use topcoat::asset::{asset, Asset};
use topcoat::view::{component, view, View};

#[component]
pub async fn header() -> topcoat::Result<impl View> {
    const LOGO: Asset = asset!("assets/logo.svg");

    Ok(view! {
        <header
            class="flex items-center justify-between px-6 md:px-16 lg:px-24 xl:px-32 py-4 border-b border-gray-300 bg-white relative transition-all"
        >
            <a href="/">
                <img class="h-9" src=(LOGO) alt="the logo">
                "Topcoat First"
            </a>
            <button aria-label="Menu" id="menu-toggle" class="sm:hidden">
                <svg
                    width="21"
                    height="15"
                    viewBox="0 0 21 15"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <rect width="21" height="1.5" rx=".75" fill="#426287" />
                    <rect x="8" y="6" width="13" height="1.5" rx=".75" fill="#426287" />
                    <rect
                        x="6"
                        y="13"
                        width="15"
                        height="1.5"
                        rx=".75"
                        fill="#426287"
                    />
                </svg>
            </button>
        </header>
    })
}