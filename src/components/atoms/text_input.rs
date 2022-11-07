use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use stylist::{yew::styled_component, Style};
const STYLE_FILE : &str = include_str!("main.css");

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub handle_onchange: Callback<String>,
}

#[styled_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let stylesheets = Style::new(STYLE_FILE).unwrap();

    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_onchange.emit(value);
    });
    html! {
        <div class={stylesheets}>
            <textarea type="text" name={props.name.clone()} onchange={onchange} />
        </div>
    }
}