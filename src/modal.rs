use yew::{html, Component, Context, Html, Properties};

#[derive(Default, PartialEq, Properties)]
pub struct Props {
    pub is_open: bool,
    #[prop_or(html!{<></>})]
    pub header: Html,
    #[prop_or(html!{<></>})]
    pub body: Html,
    #[prop_or(html!{<></>})]
    pub footer: Html,
}

pub struct Modal;

impl Component for Modal {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
            <>
                {
                    if props.is_open {
                        html! {
                            <div class="yew-default-modal">
                                <div class="header">
                                    { props.header.clone() }
                                </div>

                                <div class="body">
                                    { props.body.clone() }
                                </div>

                                <div class="footer">
                                    { props.footer.clone() }
                                </div>
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
