pub mod native_worker;
mod highlighted;

use std::cell::RefCell;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::Theme;

pub use syntect;
pub use highlighted::Highlighted;
use yew::Html;

pub struct SyntectData {
    theme: Option<Theme>,
    syntax_set: Option<SyntaxSet>,
}

thread_local!(pub static SYNTECT_DATA: RefCell<SyntectData> = RefCell::new(SyntectData {
    theme: None,
    syntax_set: None,
}));

pub fn load_theme(func: impl Fn() -> Theme) {
    let theme = func();
    SYNTECT_DATA.with(|cell| { cell.borrow_mut().theme = Some(theme) });
}

pub fn load_syntax_set(func: impl Fn() -> SyntaxSet) {
    let syntax_set = func();
    SYNTECT_DATA.with(|cell| { cell.borrow_mut().syntax_set = Some(syntax_set) });
}

fn html_to_element(html: &str) -> Html {
    let template: wasm_bindgen::JsValue = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("template")
        .unwrap()
        .into();
    let template: web_sys::HtmlTemplateElement = template.into();
    let html = html.trim();
    template.set_inner_html(html);
    Html::VRef(template.content().first_child().unwrap().into())
}
