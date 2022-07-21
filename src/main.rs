use yew::prelude::*;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props<T: PartialEq + Clone> {
    pub onselect: Callback<T>,
    pub items: Vec<Item<T>>,
}

#[derive(PartialEq, Clone)]
pub struct Item<T: PartialEq + Clone> {
    pub content: Html,
    pub inner: T,
}

#[function_component]
fn SelectItem<T: PartialEq + Clone + 'static>(props: &Props<T>) -> Html {
    let items = props.items.clone();
    html! {
        <div>
            {for items.into_iter().map(|item: Item<T>| {
                let onclick = {
                    let cb = props.onselect.clone();
                    let item = item.clone();
                    Callback::from(move |_| {
                        cb.emit(item.inner.clone());
                    })
                };
                html! { <div class={classes!("item")} {onclick}>{item.content}</div> }
            })}
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let items = vec![
        Item {
            content: html! {"red"},
            inner: String::from("red"),
        },
        Item {
            content: html! {"blue"},
            inner: String::from("blue"),
        },
    ];
    let selected_item = use_state(|| String::from("none"));

    let onselect: Callback<_> = {
        let selected_item = selected_item.clone();
        Callback::from(move |item| {
            selected_item.set(item);
        })
    };

    html! {
        <div>
            <SelectItem<String> {onselect} {items} />
            <br />
            {"selected item: "}
            { (*selected_item).clone() }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
