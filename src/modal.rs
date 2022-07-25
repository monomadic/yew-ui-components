use yew::{html, Component, Context, Html, Properties};

pub enum Msg {
    CloseModal(),
}

#[derive(Default, PartialEq, Properties)]
pub struct Props {
    pub is_open: bool,
    #[prop_or(String::from(""))]
    pub modal_class: String,
    #[prop_or(String::from(""))]
    pub modal_title: String,

    #[prop_or(true)]
    pub show_header: bool,
    #[prop_or(html!{<></>})]
    pub header: Html,
    #[prop_or(String::from(""))]
    pub header_class: String,

    #[prop_or(html!{<></>})]
    pub body: Html,
    #[prop_or(String::from(""))]
    pub body_class: String,

    #[prop_or(html!{<></>})]
    pub footer: Html,
    #[prop_or(String::from(""))]
    pub footer_class: String,
    #[prop_or(true)]
    pub show_footer: bool,
}

pub struct Modal {
    is_open: bool,
}

impl Component for Modal {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_open: _ctx.props().is_open,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CloseModal() => {
                self.is_open = false;
            }
        };
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        self.is_open = _ctx.props().is_open;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();
        // self.is_open = props.is_open;
        let close_modal = link.callback(|_| Msg::CloseModal());
        html! {
            <>
                {
                    if self.is_open {
                        html! {
                            <div class="yew-modal">
                                <div class="yew-modal-content">
                                    {
                                        if props.show_header.clone() {
                                            html! {
                                                <div class="header">
                                                    {
                                                        if props.modal_title.clone() != String::from("") {
                                                            html! {
                                                                <div class="row justify-content-between">
                                                                    <div class="header-title">
                                                                        { props.modal_title.clone() }
                                                                    </div>
                                                                    <svg onclick={close_modal} xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="close-modal"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
                                                                </div>
                                                            }
                                                        } else {
                                                            props.header.clone()
                                                        }
                                                    }
                                                </div>
                                            }
                                        } else {
                                            html! {
                                                <></>
                                            }
                                        }
                                    }

                                    <div class="body">
                                        { props.body.clone() }
                                    </div>

                                    {
                                        if props.show_footer.clone() {
                                            html! {
                                                <div class="footer">
                                                { props.footer.clone() }
                                                </div>
                                            }
                                        } else {
                                            html! {
                                                <></>
                                            }
                                        }
                                    }
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
