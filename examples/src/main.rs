use yew::prelude::*;
use yew_ui_components::{Item, Modal, SelectItem};

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
    let show_modal_title = use_state(|| String::from("Show Modal"));
    let is_open = use_state(|| false);

    let onselect: Callback<_> = {
        let selected_item = selected_item.clone();
        Callback::from(move |item| {
            selected_item.set(item);
        })
    };

    let onclick = {
        let show_modal_title = show_modal_title.clone();
        let is_open = is_open.clone();
        Callback::from(move |_| {
            let show_modal_title = show_modal_title.clone();
            if *is_open {
                show_modal_title.set(String::from("Show Modal"));
                is_open.set(false);
            } else {
                show_modal_title.set(String::from("Hide Modal"));
                is_open.set(true);
            }
        })
    };

    let header = html! {
        <div>{ "header content" }</div>
    };

    let body = html! {
        <div>{ "body content" }</div>
    };

    let footer = html! {
        <div>{ "footer content" }</div>
    };

    let is_open: bool = *is_open.clone();

    html! {
        <div>
            <SelectItem<String> {onselect} {items} />
            <br />
            {"selected item: "}
            { (*selected_item).clone() }

            <div>
                <button {onclick}>{ (*show_modal_title).clone() }</button>
            </div>

            <Modal {is_open} {header} {body} {footer} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
