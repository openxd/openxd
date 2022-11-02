use yew::{
    html, html::ChildrenRenderer, virtual_dom::VChild, ChildrenWithProps, Component, Html,
    Properties,
};

#[derive(PartialEq, Properties)]
pub struct MenuBarProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<Menu>,
}

pub struct MenuBar;

impl Component for MenuBar {
    type Message = ();
    type Properties = MenuBarProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        MenuBar
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <ul class="menu-bar">
            {for ctx.props().children.iter()}
            </ul>
        }
    }
}

pub struct Menu;

#[derive(PartialEq, Properties)]
pub struct MenuProps {
    pub id: String,
    pub title: String,
    pub disabled: bool,
    pub children: ChildrenRenderer<MenuChild>,
}

#[derive(PartialEq, Clone, derive_more::From)]
pub enum MenuChild {
    Menu(VChild<Menu>),
    Separator(VChild<MenuSeparator>),
    Entry(VChild<MenuEntry>),
}

impl Into<Html> for MenuChild {
    fn into(self) -> Html {
        match self {
            Self::Menu(menu) => menu.into(),
            Self::Separator(separator) => separator.into(),
            Self::Entry(entry) => entry.into(),
        }
    }
}

impl Component for Menu {
    type Message = ();
    type Properties = MenuProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Menu
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let mut class: String = String::from("menu-item");
        if ctx.props().disabled {
            class.push_str(" disabled");
        }

        html! {
            <li class={class}>
                <div class="menu-entry">
                    <a class="menu-title" href="#">{ctx.props().title.clone()}</a>
                    <ul class="menu">
                        {for ctx.props().children.iter()}
                    </ul>
                </div>
            </li>
        }
    }
}

pub struct MenuSeparator;

impl Component for MenuSeparator {
    type Message = ();
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        MenuSeparator
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! { <hr class="menu-separator"/> }
    }
}

#[derive(PartialEq, Properties)]
pub struct MenuEntryProps {
    pub id: String,
    pub title: String,
    #[prop_or_default]
    pub subtitle: Option<String>,
    pub disabled: bool,
}

pub struct MenuEntry;

impl Component for MenuEntry {
    type Message = ();
    type Properties = MenuEntryProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        MenuEntry
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let mut class: String = String::from("menu-item");
        if ctx.props().disabled {
            class.push_str(" disabled");
        }

        html! {
            <li class={class}>
                <div class="menu-entry">
                    <a class="menu-title">{ctx.props().title.clone()}</a>
                    if let Some(subtitle) = ctx.props().subtitle.clone() {
                        <a class="menu-subtitle">{subtitle}</a>
                    }
                </div>
            </li>
        }
    }
}
