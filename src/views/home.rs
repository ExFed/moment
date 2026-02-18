use crate::components::Timer;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Timer {}
    }
}
