use std::ops::{Mul, Sub};

use dioxus::{logger, prelude::*};

static CSS: Asset = asset!("/assets/main.css");
static WILLOW: Asset = asset!("/assets/willow2.jpg", AssetOptions::image().with_avif());
static CELEBRATION: Asset = asset!("/assets/cat.gif");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "Will you be my valentines for 2026? ♥️" }
        }
    }
}

// #[component]
// fn DogView() -> Element {
//     let skip = move |evt| info!("skipping");
//     let save = move |evt| {};

//     rsx! {
//         div { id: "dogview",
//             // img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
//             img { src: WILLOW }
//         }
//         div { id: "buttons",
//             button { onclick: skip, id: "Yes", "skip" }
//             button { onclick: save, id: "No", "save!" }
//         }
//     }
// }

#[component]
fn DogView() -> Element {
    let mut yes_scale = use_signal(|| 1.0_f32);
    let mut no_scale = use_signal(|| 1.0_f32);
    let mut show_gif = use_signal(|| false);

    let yes = move |_| {
        show_gif.set(true);
    };

    let no = move |_| {
        let no_scale_value = no_scale.read().mul(0.8).max(0.3);
        no_scale.set(no_scale_value);
        let yes_scale_value = yes_scale.read().mul(1.2).min(3.0);
        yes_scale.set(yes_scale_value);
    };

    rsx! {
        /* Celebration overlay */
        if *show_gif.read() {
            div { id: "celebration-overlay",
                img { src: CELEBRATION }
            }
        }

        div { id: "dogview",
            img { src: WILLOW }
        }
        div { id: "buttons",
            button {
                class: "choice-button yes",
                onclick: yes,
                style: "transform: scale({yes_scale});",
                "Yes 💕"
            }

            button {
                class: "choice-button no",
                onclick: no,
                style: "transform: scale({no_scale});",
                "No 💔"
            }
        }
    }
}
