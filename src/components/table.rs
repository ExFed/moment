use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TableItem {
    id: usize,
    description: String,
    completed: bool,
}

#[component]
pub fn Table() -> Element {
    let mut items = use_signal(Vec::<TableItem>::new);
    let mut next_id = use_signal(|| 0);
    let mut editing_id = use_signal(|| Option::<usize>::None);
    let mut drag_source = use_signal(|| Option::<usize>::None);
    let mut new_item_text = use_signal(String::new);

    rsx! {
        div {
            class: "w-full flex flex-col space-y-2 mt-8",
            h2 { class: "text-2xl font-bold", "Table" }
            div {
                class: "w-full border border-slate-700 rounded-lg p-4 bg-slate-800",
                for (index, item) in items.read().clone().into_iter().enumerate() {
                    div {
                        key: "{item.id}",
                        class: "flex items-center space-x-2 py-2 border-b border-slate-700 last:border-b-0 group",
                        draggable: "true",
                        ondragstart: move |_| drag_source.set(Some(index)),
                        ondragover: move |e| e.prevent_default(),
                        ondrop: move |e| {
                            e.prevent_default();
                            if let Some(src) = *drag_source.read() {
                                if src != index {
                                    let mut items_write = items.write();
                                    let item = items_write.remove(src);
                                    items_write.insert(index, item);
                                }
                            }
                            drag_source.set(None);
                        },
                        div {
                            class: "cursor-move text-slate-500 hover:text-white px-2",
                            "\u{2630}"
                        }
                        input {
                            class: "h-5 w-5 rounded",
                            r#type: "checkbox",
                            checked: item.completed,
                            onchange: move |e| {
                                if let Some(i) = items.write().iter_mut().find(|i| i.id == item.id) {
                                    i.completed = e.value().parse().unwrap_or(false);
                                }
                            }
                        }
                        if Some(item.id) == *editing_id.read() {
                            input {
                                class: "flex-grow bg-slate-700 text-white p-1 rounded focus:outline-none focus:ring-2 focus:ring-blue-500",
                                r#type: "text",
                                value: "{item.description}",
                                autofocus: true,
                                onkeydown: move |e| {
                                    match e.key() {
                                        Key::Enter => {
                                            let should_remove = items.read().iter().find(|i| i.id == item.id).is_some_and(|i| i.description.trim().is_empty());
                                            if should_remove {
                                                items.write().retain(|x| x.id != item.id);
                                            }
                                            editing_id.set(None);
                                        }
                                        Key::Escape => {
                                            editing_id.set(None);
                                        }
                                        _ => {}
                                    }
                                },
                                oninput: move |e| {
                                    if let Some(i) = items.write().iter_mut().find(|i| i.id == item.id) {
                                        i.description = e.value();
                                    }
                                },
                                onblur: move |_| {
                                    let should_remove = items.read().iter().find(|i| i.id == item.id).is_some_and(|i| i.description.trim().is_empty());
                                    if should_remove {
                                        items.write().retain(|x| x.id != item.id);
                                    }
                                    editing_id.set(None);
                                }
                            }
                        } else {
                            span {
                                class: if item.completed { "flex-grow cursor-pointer text-slate-500 line-through" } else { "flex-grow cursor-pointer text-white" },
                                onclick: move |_| editing_id.set(Some(item.id)),
                                "{item.description}"
                            }
                        }
                        button {
                            class: "text-red-500 hover:text-red-400 font-bold px-2 invisible group-hover:visible",
                            onclick: move |_| items.write().retain(|i| i.id != item.id),
                            "✕"
                        }
                    }
                }

                div {
                    class: "flex items-center space-x-2 py-2",
                    div { class: "w-8" }
                    div { class: "h-5 w-5" }
                    input {
                        class: "flex-grow bg-slate-700 text-white p-2 rounded focus:outline-none focus:ring-2 focus:ring-blue-500",
                        r#type: "text",
                        placeholder: "Add new item...",
                        value: "{new_item_text}",
                        oninput: move |e| new_item_text.set(e.value()),
                        onkeydown: move |e| {
                            if e.key() == Key::Enter {
                                let val = new_item_text.read().clone();
                                if !val.trim().is_empty() {
                                    let id = *next_id.read();
                                    *next_id.write() += 1;
                                    items.write().push(TableItem {
                                        id,
                                        description: val.trim().to_string(),
                                        completed: false,
                                    });
                                    new_item_text.set(String::new());
                                }
                            }
                        }
                    }
                    div { class: "w-8" }
                }
            }
        }
    }
}
