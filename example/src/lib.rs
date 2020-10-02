use yew::prelude::*;
use yew_highlight::Highlighted;

pub struct App {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Close
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }


    fn view(&self) -> Html {
        let code = include_str!("main.rs");
        html! {
            <>
                <Highlighted code=code />
            </>
        }
    }
}
