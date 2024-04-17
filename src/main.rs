#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_desktop::{Config, WindowBuilder};

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        Config::default().with_window(WindowBuilder::new().with_title("Ranagams")),
    );
}

fn App(cx: Scope) -> Element {
    let word = use_state(cx, || "".to_string());
    let query = use_state(cx, || "".to_string());
    let future = use_future(cx, query, |query| async move {
        fetch_anagrams(query.as_str()).await.unwrap_or_default()
    });

    cx.render(rsx! {
        link { rel: "stylesheet", href: "../dist/output.css" }
        div {
            class: "relative bg-[#a08cb4] h-screen flex flex-col justify-center items-center",
            h1 {
                class: "text-4xl font-bold m-0 py-5 text-center",
                "Ranagams"
            }
            // Score { words: 10, score: 10}
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
                    autofocus: "true",
                    oninput: move |evt| {
                        word.set(evt.value.clone());
                    },
                    onkeypress: move |evt| {
                        if evt.key() == Key::Enter {
                            query.set(word.to_string())
                        }
                    }
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
            div {
                if let Some(future) = future.value() {
                    rsx!(
                        div {
                            class: "mx-4",
                            GetAnagrams { words: future.clone() }
                        }
                    )
                }
            }
        }
    })
}

#[derive(Props, PartialEq, Clone, Debug, Default)]
struct GetAnagramsProps {
    words: Vec<(String, u32)>,
}

fn GetAnagrams(cx: Scope<GetAnagramsProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col gap-2 p-4 bg-purple-800", // Adjusted for a single column with dark purple background
            cx.props.words.iter().map(|(word, score)| {
                rsx! {
                    div {
                        class: "flex justify-between items-center", // Flex container for inline display of word and score
                        span {
                            class: "text-black", // Word in black
                            "{word}"
                        }
                        span {
                            class: "text-white", // Score in white
                            "{score}"
                        }
                    }
                }
            })
        }
    })
}

fn sort_hashmap(hashmap: HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut vec: Vec<(String, u32)> = hashmap.into_iter().collect();

    // Don't question it...
    vec.sort_by(|a, b| b.1.cmp(&a.1));

    return vec;
}

async fn fetch_anagrams(input: &str) -> reqwest::Result<Vec<(String, u32)>> {
    let res: HashMap<String, u32> = reqwest::get(&format!("http://127.0.0.1:5000/?string={input}"))
        .await?
        .json()
        .await?;
    Ok(sort_hashmap(res))
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
