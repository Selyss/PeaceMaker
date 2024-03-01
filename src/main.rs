#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};

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
