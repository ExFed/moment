use crate::components::Timer;
use dioxus::prelude::*;

#[component]
pub fn Play() -> Element {
    rsx! {
        Timer {}
    }
}
