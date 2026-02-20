use std::env::var;

use dioxus::prelude::*;
use views::Lobby;
use views::Play;

/// Define a components module that contains all shared components for our app.
mod components;

/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Lobby {},
    #[route("/play")]
    Play {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

const COMMIT_HASH: &str = env!("COMMIT_HASH");
const COMMIT_DATE: &str = env!("COMMIT_DATE");
const DESCRIBE_VERSION: &str = env!("DESCRIBE_VERSION");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
