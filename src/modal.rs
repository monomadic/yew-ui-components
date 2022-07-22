use yew::{Component, Context, html, Html, Properties};

#[derive(Default, PartialEq, Properties)]
pub struct Props {
    pub is_open: bool,
}

pub struct Modal;

impl Component for Modal {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                {
                    if ctx.props().is_open {
                        html! {
                            <div>
                                <div class="header"> {"Header"} </div>
                                <div class="body"> {"Body"} </div>
                                <div class="footer"> {"Footer"} </div>
                            </div>
                        }
                    } else {
                        html! {
                            <></>
                        }
                    }
                }
            </>
        }
    }
}
