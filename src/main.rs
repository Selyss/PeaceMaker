#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};
use rand::prelude::*;

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        Config::default().with_window(WindowBuilder::new().with_title("Ranagams")),
    );
}

fn App(cx: Scope) -> Element {
    let word = use_state(cx, || "".to_string());
    cx.render(rsx! {
        link { rel: "stylesheet", href: "../dist/output.css" }
        div {
            class: "bg-[#a08cb4] h-screen flex flex-col justify-center items-center",
                    input {
                        value: "{word}",
                        oninput: move |evt| word.set(evt.value.clone()),
                    }

            div {
                class: "flex flex-row gap-4",
                    div {
                        class: "flex flex-row gap-4 justify-center",
                        for c in word.chars().take(6) {
                            Tile { ch: "{c}" }
                        }
                    }
                    div {
                        class: "flex flex-row gap-4 justify-center",
                        if word.chars().count() <= 6 {
                            for _ in 0..(6 - word.chars().count()) {
                                rsx!(empty_space(cx));
                            }
                        }
                    }

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

#[derive(Props)]
struct TileProps<'a> {
    ch: &'a str,
}

fn Tile<'a>(cx: Scope<'a, TileProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "sho h-24 w-24 rounded-lg border-2 border-[#a78f5e] bg-wood",
            p {
                class: "mt-3 text-center text-6xl font-bold uppercase",
                cx.props.ch
            }
        }
    })
}
