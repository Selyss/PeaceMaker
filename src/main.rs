#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};
use rand::prelude::*;

#[derive(Props)]
struct LetterProps<'a> {
    letter: &'a str,
}

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch_cfg(
        App,
        Config::default().with_window(WindowBuilder::new().with_title("Peace Maker")),
    );
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        link { rel: "stylesheet", href: "../dist/output.css" },
        // div {
        //     class: "w-full h-screen flex items-center justify-center",
        //     div {
        //         div {
        //             class: "relative mb-4 flex w-full flex-wrap items-stretch",
                    word_input {}
        //         }
        //     }

        // }
    })
}

fn word_input<'a>(cx: Scope<'a>) -> Element<'a> {
    let input = use_state(cx, String::new);

    let oninput = |e: FormEvent| {
        input.set(e.data.value.clone());
    };

    cx.render(rsx! {
        div {
            class: "wrapper",
            div {
                class: "container",
                div {
                    id: "rack",
                    class: "rack",
                    input {
                        id: "input",
                        "type": "text",
                        value: "scrabble",
                        autofocus: "true",
                        oninput: oninput
                    }
                }
            }
        }
    })
}

fn add_tile<'a>(cx: Scope<'a, ch: LetterProps<'a>>) -> Element<'a> {
    let mut rng = rand::thread_rng();
    let rotate_value: f64 = rng.gen_range(-1.5..=1.5);
    let rotate_string = format!("--rotate: {};", rotate_value).as_str();
    cx.render(rsx! {
        div {
            class: "tile",
            "data-letter": ch,
            style: rotate_string,
            ch
        }
    })
}

fn remove_last_tile<'a>(cx: Scope<'a>, ch: &str) -> Element<'a> {
    cx.render(rsx! {
        div {
            "a"
        }
    })
}
