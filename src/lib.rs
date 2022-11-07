use gloo::console::log;
use yew::prelude::*;

mod components;

use components::atoms::main_title::{Color, MainTitle};
use components::molecules::custom_form::CustomForm;
use stylist::{yew::styled_component,Style};

const STYLE_FILE : &str = include_str!("main.css");


#[styled_component(App)]
pub fn app() -> Html {
    let stylesheets = Style::new(STYLE_FILE).unwrap();

    let main_title_load = Callback::from(|message: String| log!(message));
    html! {
        <div class={stylesheets}>
        <div class="container">
                <MainTitle title="Word Counter" color={Color::Ok} on_load={main_title_load} />
                <CustomForm />
            </div>
        </div>
    }
}
