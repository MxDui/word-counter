use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use crate::components::atoms::word_counter::WordCounter;
use yew::prelude::*;

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let text_state = use_state(|| "no text set".to_owned());
    let button_count_state = use_state(|| 0_u32);

    let cloned_text_state = text_state.clone();
    let text_changed = Callback::from(move |text| {
        cloned_text_state.set(text);
    });

    let cloned_button_count_state = button_count_state.clone();
    let button_clicked = Callback::from(move |_| {
        let count = *cloned_button_count_state;
        cloned_button_count_state.set(count + 1);
    });
    html! {
      <div>
        <TextInput name="text" handle_onchange={text_changed} />
        <CustomButton label="Submit" onclick={button_clicked} />
        <WordCounter text={text_state.clone().to_string()} />
        <p>{"Username: "}{&*text_state}</p>
        <p>{"Button has been clicked "}{*button_count_state}{" times"}</p>
      </div>
    }
}
