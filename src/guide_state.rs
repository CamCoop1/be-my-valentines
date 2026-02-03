use dioxus::prelude::*;
use dioxus_timer::{use_timer, DioxusTimer, TimerState};

use std::{
    ops::{Add, Mul},
    time::Duration,
};

static CSS: Asset = asset!("/assets/main.css");
static WILLOW: Asset = asset!("/assets/willow2.jpg", AssetOptions::image().with_avif());
static F1: Asset = asset!("/assets/f1-meme.jpg", AssetOptions::image().with_avif());
static CELEBRATION: Asset = asset!("/assets/cat.gif");

#[component]
pub fn App() -> Element {
    let timer = use_timer(Duration::from_millis(3000));

    rsx! {
        document::Stylesheet { href: CSS }
        Title {timer}
        DogView {}
        ButtonElement {timer}
    }
}

#[component]
fn Title(timer: Signal<DioxusTimer>) -> Element {
    rsx! {
        div { id: "title",
            h1 { "Will you be my valentines for 2026? ♥️" }
        }
    }
}

#[component]
fn DogView() -> Element {
    rsx! {
    div { id: "dogview",
        img { src: WILLOW }
    }
    }
}

#[component]
fn ButtonElement(timer: Signal<DioxusTimer>) -> Element {
    let mut yes_scale = use_signal(|| 1.0_f32);
    let mut no_scale = use_signal(|| 1.0_f32);
    let mut show_gif = use_signal(|| false);
    let mut counter = use_signal(|| 0_u32);

    let yes = move |_| {
        show_gif.set(true);
    };

    let no = move |_| {
        // Update counter
        let updated_counter = *counter.read() + 1u32;
        counter.set(updated_counter);
        // Update no_scale signal
        let no_scale_value = no_scale.read().mul(0.8).max(0.3);
        no_scale.set(no_scale_value);
        // Update yes_scale signal
        let yes_scale_value = yes_scale.read().mul(1.2).min(3.0);
        yes_scale.set(yes_scale_value);
        // If we have hit 4 presses, start timer
        if counter() == 4 {
            // Must set a preset time for the timer to work
            timer.write().set_preset_time(Duration::from_secs(8));
            // Starting here turns it into TimerState::Working
            timer.write().start();
        }
    };

    let element = match timer.read().state() {
        TimerState::Inactive => {
            rsx! {
if *show_gif.read() {

                    div { id: "celebration-overlay",
                        h1 { "You and me buns for 2026 ♥️"}
                        img { src: CELEBRATION }
                    }
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
        TimerState::Working => {
            rsx! {
                if *show_gif.read() {

                    div { id: "celebration-overlay",
                        h1 { "You and me buns for 2026 ♥️"}
                        img { src: CELEBRATION }
                    }
                }
                    div { id: "celebration-overlay",
                                img { src: F1 }
                    },
            }
        }
        TimerState::Finished => rsx! {
            if *show_gif.read() {

                div { id: "celebration-overlay",
                    h1 { "You and me buns for 2026 ♥️"}
                    img { src: CELEBRATION }
                }
            }
            div { id: "buttons",
            button {
                class: "choice-button yes",
                onclick: yes,
                // style: "transform: scale({yes_scale});",
                "Yes 💕"
            }
        }},
        TimerState::Paused => rsx! {},
    };
    element
}
