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
    #[layout(PageLayout)]
        #[route("/")]
        Lobby {},
        #[route("/play?:time_limit")]
        Play { time_limit: Option<u32> },
}

#[component]
fn PageLayout() -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col items-center justify-center bg-slate-900 text-slate-100",
            div { class: "flex-grow flex w-full max-w-4xl p-4 flex-col", Outlet::<Route> {} }
            footer { class: "w-full text-center text-xs text-slate-600 p-2",
                span { "{DESCRIBE_VERSION}" }
            }
        }
    }
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

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
