use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Lobby() -> Element {
    let mut time_limit = use_signal(|| 90);

    rsx! {
        // div { class: "w-full flex flex-col space-y-2 mt-8",
        div { class: "flex flex-col items-center justify-center space-y-4 p-8 w-full max-w-2xl mx-auto bg-gray-800 rounded-lg shadow-xl",
            h1 { class: "text-3xl font-bold text-white", "Moment Timer" }
            div { class: "flex flex-col space-y-2 w-full",
                label { class: "text-gray-300 font-semibold", "Time Limit (seconds)" }
                input {
                    class: "w-full p-3 rounded bg-gray-700 text-white border border-gray-600 focus:outline-none focus:border-blue-500",
                    r#type: "number",
                    min: "1",
                    value: "{time_limit}",
                    oninput: move |e| {
                        if let Ok(val) = e.value().parse::<u32>() {
                            time_limit.set(val);
                        }
                    },
                }
            }
            Link {
                class: "w-full p-3 text-center bg-blue-600 hover:bg-blue-500 text-white font-bold rounded transition-colors",
                to: Route::Play {
                    time_limit: Some(*time_limit.read()),
                },
                "Start"
            }
        }
    }
}
