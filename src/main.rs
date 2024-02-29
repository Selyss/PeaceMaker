#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};


fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch_cfg(
        App,
        Config::default().with_window(
            WindowBuilder::new()
                .with_title("Peace Maker"),
        ),
    );
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        link { rel: "stylesheet", href: "../dist/output.css" },
        div {
            class: "w-full h-screen flex items-center justify-center",
            div {
                div {
                    class: "relative mb-4 flex w-full flex-wrap items-stretch",
                    input {
                        class: "relative m-0 -mr-0.5 block min-w-0 flex-auto rounded-l border border-solid border-neutral-300 bg-transparent bg-clip-padding px-3 py-[0.25rem] text-base font-normal leading-[1.6] text-neutral-700 outline-none transition duration-200 ease-in-out focus:z-[3] focus:border-primary focus:text-neutral-700 focus:shadow-[inset_0_0_0_1px_rgb(59,113,202)] focus:outline-none dark:border-neutral-600 dark:text-neutral-200 dark:placeholder:text-neutral-200 dark:focus:border-primary",
                        "type": "text",
                        placeholder: "Enter a word",
                        autofocus: "true",
                    }
                }
            }
            
        }
    })
}
