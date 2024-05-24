#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::{info, LevelFilter};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Deref};

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(app);
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
struct Anagram {
    word: String,
    score: u16,
}

fn app() -> Element {
    let word = use_signal(|| "".to_string());
    let anagrams = use_resource(move || async move { fetch_anagrams(&word()).await });
    let draft = use_signal(|| "".to_string());

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div {
            class: "bg-[#a08cb4] h-screen flex flex-col justify-center ",
            // h1 {
            //     class: "text-4xl font-bold text-center pb-40",
            //     "Ranagams"
            // }
            // TODO: add score
            LetterRow { draft }
            SearchBox { word, draft }
            // FIXME: REDO THIS PART TO OCCUR ON ENTER PRESS
            if let Some(Ok(words)) = anagrams.read().as_ref() {
                {info!("we got anagrams!")}
                DisplayAnagrams { anagrams: words.to_owned() }
            }
        }
    }
}

#[component]
fn Score() -> Element {
    rsx! {
        div {}
    }
}

#[component]
fn DisplayAnagrams(anagrams: Vec<Anagram>) -> Element {
    rsx! {
        div {
            class: "flex flex-wrap gap-4 p-4 bg-purple-800 w-full max-w-4xl",
            {anagrams.iter().map(|gram|
                rsx! {
                    div {
                        class: "flex justify-between items-center bg-white p-2 rounded w-full sm:w-1/2 lg:w-1/3",
                        span {
                            class: "text-black",
                            "{gram.word}"
                        }
                        span {
                            class: "text-black",
                            "{gram.score}"
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
            class: "flex flex-row gap-4 justify-center mb-4",
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
            class: "relative mb-4",
            textarea {
                resize: "none",
                class: "select-none opacity-0 absolute inset-0",
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

async fn fetch_anagrams(input: &str) -> Result<Vec<Anagram>, Error> {
    let res: HashMap<String, u16> = reqwest::get(&format!("http://127.0.0.1:5000?string={input}"))
        .await?
        .json()
        .await?;

    let mut anagrams = Vec::new();

    // append Anagram to Vec
    for (word, score) in res {
        anagrams.push(Anagram { word, score });
    }

    // sort anagrams in desc order
    anagrams.sort_by(|a, b| b.score.cmp(&a.score));

    Ok(anagrams)
}
