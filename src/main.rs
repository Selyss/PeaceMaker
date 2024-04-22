#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::{info, LevelFilter};
use std::{collections::HashMap, ops::Deref};

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(app);
}

fn app() -> Element {
    let word = use_signal(|| "".to_string());
    let anagrams = use_resource(move || async move { fetch_anagrams(&word()).await });
    let draft = use_signal(|| "".to_string());

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div {
            class: "relative bg-[#a08cb4] h-screen flex flex-col justify-center items-center",
            h1 {
                class: "text-4xl font-bold text-center pb-40",
                "Ranagams"
            }
            // TODO: add score
            LetterRow { draft }
            SearchBox { word, draft }
            // FIXME: REDO THIS PART TO OCCUR ON ENTER PRESS
            if let Some(Ok(words)) = anagrams.read().as_ref() {
                {info!("we got anagrams!")}
                DisplayAnagrams { anagrams: words.deref().to_vec() }
            }
        }
    }
}

#[component]
fn DisplayAnagrams(anagrams: Vec<(String, u32)>) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2 p-4 bg-purple-800",
            {anagrams.iter().map(|(word, score)|
                rsx! {
                    div {
                        class: "flex justify-between items-center",
                        span {
                            class: "text-black",
                            "{word}"
                        }
                        span {
                            class: "text-white",
                            "{score}"
                        }
                    }
                })

            }
        }
    }
}

#[component]
fn LetterRow(mut draft: Signal<String>) -> Element {
    rsx! {
        div {
            class: "absolute flex flex-row gap-4 container justify-center",
            for c in draft.read().chars().take(6) {
                Tile { ch: "{c}" }
            }
            for _ in 0..(6 - draft.read().chars().count().min(6)) {
                EmptyTile {}
            }
        }
    }
}

#[component]
fn EmptyTile() -> Element {
    rsx! {
        div {
            class: "shi h-24 w-24 rounded-lg bg-[#504464]",
        }
    }
}

#[component]
fn Tile(ch: String) -> Element {
    rsx! {
        div {
            class: "sho h-24 w-24 rounded-lg border-2 border-[#a78f5e] bg-wood",
            p {
                class: "mt-3 text-center text-6xl font-bold uppercase",
                "{ch}",
            }
        }
    }
}

#[component]
fn SearchBox(mut word: Signal<String>, mut draft: Signal<String>) -> Element {
    rsx! {
        div {
            class: "absolute",
            textarea {
                resize: "none",
                class: "select-none opacity-0",
                cols: "86",
                rows: "4",
                "type": "text",
                spellcheck: "false",
                maxlength: "6",
                minlength: "6",
                value: "{draft}",
                autofocus: "true",
                oninput: move |evt| {
                    draft.set(evt.value());
                },
                onkeydown: move |evt| {
                    if evt.key() == Key::Enter {
                        word.set(draft.to_string());
                    }
                }
            }
        }
    }
}

fn sort_hashmap(hashmap: HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut vec: Vec<(String, u32)> = hashmap.into_iter().collect();

    // Don't question it...
    vec.sort_by(|a, b| b.1.cmp(&a.1));

    return vec;
}

async fn fetch_anagrams(input: &str) -> reqwest::Result<Vec<(String, u32)>> {
    let res: HashMap<String, u32> =
        reqwest::get(&format!("https://flask-anagrams.vercel.app?string={input}"))
            .await?
            .json()
            .await?;
    Ok(sort_hashmap(res))
}
