use crate::components::atoms::text_input::TextInput;
use crate::components::atoms::word_counter::WordCounter;
use stylist::{yew::styled_component,Style};

use yew::prelude::*;


const STYLE_FILE : &str = include_str!("main.css");

#[styled_component(CustomForm)]
pub fn custom_form() -> Html {

    let stylesheets = Style::new(STYLE_FILE).unwrap();

    let text_state = use_state(|| "".to_owned());

    let cloned_text_state = text_state.clone();
    let text_changed = Callback::from(move |text| {
        cloned_text_state.set(text);
    });



    html! {
        <div class={stylesheets}>          
          <TextInput name="text" handle_onchange={text_changed} />
          <WordCounter text={text_state.clone().to_string()} />
        </div>
    }
}
