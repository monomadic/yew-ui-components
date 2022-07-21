use yew::{Component, Context, html, Html, Properties};

#[derive(Default, PartialEq, Properties)]
pub struct Props;

pub struct Modal;

impl Component for Modal {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Modal
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { "modal" }
            </div>
        }
    }
}
