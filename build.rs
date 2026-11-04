fn main() {
    topcoat::icon::iconify::BuildConfig::new()
        .icon_set("skill-icons")
        .icon_set("lucide")
        .stage()
        .unwrap();

    topcoat::tailwind::BuildConfig::new()
        .input("styles.css")
        .render()
        .unwrap();
}
