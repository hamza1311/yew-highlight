use yew::prelude::*;
use crate::{native_worker, html_to_element};
use yew::services::ConsoleService;

pub struct Highlighted {
    _link: ComponentLink<Self>,
    props: Props,
    highlighted_html: Option<Html>,
    worker: Box<dyn Bridge<native_worker::Worker>>,
}

pub enum Msg {
    CodeHighlighted(Html),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub code: String,
}

impl Component for Highlighted {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut worker = native_worker::Worker::bridge(link.callback(|resp| {
            ConsoleService::log("response received");
            match resp {
                native_worker::Response::Highlighted(html) => Msg::CodeHighlighted(html_to_element(&html))
            }
        }));
        ConsoleService::log("sending request");
        worker.send(native_worker::Request::HighlightCode(props.code.clone()));
        Self { _link: link, props, highlighted_html: None, worker }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            /*Msg::FlipShowCode => {
                if let None = self.highlighted_html {
                    let html_string = SYNTECT_DATA.with(|cell| {
                        let data = cell.borrow();
                        let syntax = data.syntax_set.as_ref().unwrap().find_syntax_by_extension("rs").unwrap();
                        highlighted_html_for_string(&self.props.code, &data.syntax_set.as_ref().unwrap(), syntax, &data.theme.as_ref().unwrap())
                    });
                    self.highlighted_html = Some(html_to_element(&html_string));
                    //
                }
                self.showing_code = !self.showing_code;
                true
            }*/
            Msg::CodeHighlighted(html) => {
                self.highlighted_html = Some(html);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        self.highlighted_html = None;
        self.worker.send(native_worker::Request::HighlightCode(self.props.code.clone()));
        true
    }

    fn view(&self) -> Html {
        html! { <>
            {
                if let Some(highlighed) = &self.highlighted_html { highlighed.clone() } else {
                    html! {
                        <pre><code>{&self.props.code}</code></pre>
                    }
                }
            }
        </> }
    }
}
