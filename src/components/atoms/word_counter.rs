use yew::prelude::*;
use stylist::{yew::styled_component, Style};
const STYLE_FILE : &str = include_str!("main.css");

#[derive(Properties, PartialEq)]

pub struct Props {
    pub text: String,
}

#[styled_component(WordCounter)]
pub fn word_counter(props: &Props) -> Html {
    let stylesheets = Style::new(STYLE_FILE).unwrap();

    let text = props.text.clone();
    let word_count_no_punctuation = text
        .split_whitespace()
        .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()))
        .filter(|word| !word.is_empty())
        .count();

    html! {
        <div class={stylesheets}>
            <p>{"Word count: "}{word_count_no_punctuation}</p>
        </div>
    }
}
