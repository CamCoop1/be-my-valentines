mod guide_rsx;
mod guide_state;
use dioxus::prelude::*;
use guide_state::App;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

// #[component]
// fn App() -> Element {
//     rsx! {
//         "Hi Buns"
//     }
// }
