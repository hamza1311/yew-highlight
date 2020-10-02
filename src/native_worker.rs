use serde::{Deserialize, Serialize};
use yew::worker::{Agent, AgentLink, HandlerId, Public};
use syntect::html::highlighted_html_for_string;
use crate::{SYNTECT_DATA};
use yew::services::ConsoleService;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    HighlightCode(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Highlighted(String)
}

pub enum Msg {
    Updating,
}

pub struct Worker {
    link: AgentLink<Worker>,
}

impl Agent for Worker {
    type Reach = Public<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Updating => {
                // log::info!("Tick...");
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::HighlightCode(code) => {
                ConsoleService::log("received request");
                let html_string = SYNTECT_DATA.with(|cell| {
                    let data = cell.borrow();
                    let syntax = data.syntax_set.as_ref().unwrap().find_syntax_by_extension("rs").unwrap();
                    highlighted_html_for_string(&code, &data.syntax_set.as_ref().unwrap(), syntax, &data.theme.as_ref().unwrap())
                });
                self.link.respond(who, Response::Highlighted(html_string));
                ConsoleService::log("sent resp");
            }
        }
    }

    fn name_of_resource() -> &'static str {
        "worker.js"
    }
}
