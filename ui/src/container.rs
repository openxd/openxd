use yew::{Component, html::ChildrenRenderer, html, Properties, virtual_dom::VChild, Html};

use crate::menubar::MenuBar;

#[derive(PartialEq, Properties)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<ContainerChild>,
    #[prop_or_default]
    pub class: String,
}

pub struct Container;

#[derive(PartialEq, derive_more::From, Clone)]
pub enum ContainerChild {
    MenuBar(VChild<MenuBar>)
}

impl Into<Html> for ContainerChild {
    fn into(self) -> Html {
        match self {
            Self::MenuBar(menubar) => menubar.into()
        }
    }
}

impl Component for Container {
    type Message = ();
    type Properties = ContainerProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Container
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class="container">
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
