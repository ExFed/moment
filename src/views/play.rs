use crate::components::{Table, Timer};
use dioxus::prelude::*;

#[component]
pub fn Play(time_limit: Option<u32>) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center space-y-8 w-full max-w-2xl mx-auto",
            Timer { time_limit }
            Table {}
        }
    }
}
