use yew::prelude::*;

#[function_component]
pub fn SelectPet() -> Html {
	let selected_item = use_state(|| String::from("No Pets"));

	let onclick = {
		let selected_item = selected_item.clone();
		Callback::from(move |_| {
			selected_item.set(String::from("Dog"));
		})
	};

	html! {
		<button {onclick}>{(*selected_item).clone()}</button>
	}
}
