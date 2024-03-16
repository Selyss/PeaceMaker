#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

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
            class: "relative bg-[#a08cb4] h-screen flex flex-col justify-center items-center",
            Score { words: 10, score: 10}
            div {
                class: "absolute",
                textarea {
                    // TODO: capture any key presses on focus instead?
                    class: "select-none opacity-0",
                    cols: "86",
                    rows: "4",
                    "type": "text",
                    spellcheck: "false",
                    maxlength: "6",
                    value: "{word}",
                    oninput: move |evt| word.set(evt.value.clone()),
                    autofocus: "true",
                }
            }
            div {
                class: "flex flex-row gap-4 container justify-center",
                for c in word.chars().take(6) {
                    Tile { ch: "{c}" }
                }
                for _ in 0..(6 - word.chars().count().min(6)) {
                    EmptyTile(cx)
                }
            }
        }
    })
}

fn EmptyTile(cx: Scope) -> Element {
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

#[derive(Props, PartialEq)]
struct ScoreProps {
    #[props(default = 0)]
    words: u32,
    #[props(default = 0000)]
    score: u32,
}

fn Score(cx: Scope<ScoreProps>) -> Element {
    cx.render(rsx! {
            div {
                class: "content-main",
                div {
                class: "content-box",
                div {
                class: "flex flex-col",
                h1 {
                class: "font-black text-xl uppercase",
                "Words: {cx.props.words}"
                h1 {
                    class: "font-extrabold text-3xl uppercase",
                    "Score: {cx.props.score}"
                }
            }
            }
        }
    }

        })
}
