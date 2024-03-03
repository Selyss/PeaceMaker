#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};
use rand::prelude::*;

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
        link { rel: "stylesheet", href: "../dist/output.css" }
        div {
            class: "bg-[#a08cb4]",
            div {
                class: "container m-auto grid h-screen w-screen grid-cols-6 content-center gap-4",
                empty_space(cx)
                empty_space(cx)
                empty_space(cx)
                empty_space(cx)
                empty_space(cx)
                empty_space(cx)
                    tile(cx)
                    tile(cx)
                    tile(cx)
                    tile(cx)
                    tile(cx)
                    tile(cx)
                }



        }

    })
}

fn empty_space<'a>(cx: Scope<'a>) -> Element<'a> {
    cx.render(rsx! {
                div {
                    class: "shi h-24 w-24 rounded-lg bg-[#504464]",
                }

    })
}
fn tile<'a>(cx: Scope<'a>) -> Element<'a> {
    cx.render(rsx! {
                    div {
                        class: "sho size-28 h-24 w-24 rounded-lg border-2 border-[#a78f5e] bg-wood",
                        p {
                            class: "mt-3 text-center align-middle text-6xl font-bold",
                            "A"
                        }

                    }

    })
}

#[warn(dead_code)]
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
