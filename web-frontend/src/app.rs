use ui::container::Container;
use ui::menubar::{MenuBar, Menu, MenuEntry};
use yew::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <Container>
                    <MenuBar>
                        <Menu id="menu:file" title="File" disabled={false}>
                            <MenuEntry id="menu:file:open" title="Open" disabled=false />
                        </Menu>
                    </MenuBar>
                </Container>
            <p>{ "Hello world!" }</p>
            </div>
        } 
    }
}
