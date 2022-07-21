use crate::select_item::{Item, SelectItem};
use yew::prelude::*;

mod select_item;

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
