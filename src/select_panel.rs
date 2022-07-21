use yew::{Component, Context, html, Html, Properties};

#[derive(Default, PartialEq, Properties)]
pub struct Props;

pub struct SelectPanel;

impl Component for SelectPanel {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        SelectPanel
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        }
    }
}
