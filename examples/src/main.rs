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
        let is_open = is_open.clone();

        Callback::from(move |item| {
            selected_item.set(item);
            is_open.set(false);
        })
    };

    let onclick = {
        let show_modal_title = show_modal_title.clone();
        let is_open = is_open.clone();
        Callback::from(move |_| {
            let show_modal_title = show_modal_title.clone();
            show_modal_title.set(String::from("Show Modal"));
            is_open.set(true);
        })
    };

    // let close_modal = {
    //     let show_modal_title = show_modal_title.clone();
    //     let is_open = is_open.clone();
    //     Callback::from(move |_| {
    //         let show_modal_title = show_modal_title.clone();
    //         show_modal_title.set(String::from("Show Modal"));
    //         is_open.set(false);
    //     })
    // };

    let modal_title = String::from("Modal title");

    let body = html! {
        <div>
            <SelectItem<String> {onselect} {items} />
        </div>
    };

    let footer = html! {
        <div>
            { "footer" }
        </div>
    };

    let is_open: bool = *is_open.clone();
    let modal_id: String = String::from("Modal_1");
    html! {
        <div>
            {"selected item: "}
            { (*selected_item).clone() }
            <br />
            <div>
                <button {onclick}>{ (*show_modal_title).clone() }</button>
            </div>

            <Modal {is_open} {modal_title} {body} {footer} id={modal_id} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
