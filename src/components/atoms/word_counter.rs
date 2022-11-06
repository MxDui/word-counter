use yew::prelude::*;

#[derive(Properties, PartialEq)]

pub struct Props {
    pub text: String,
}

#[function_component(WordCounter)]
pub fn word_counter(props: &Props) -> Html {
    let text = props.text.clone();
    let word_count_no_punctuation = text
        .split_whitespace()
        .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()))
        .filter(|word| !word.is_empty())
        .count();

    html! {
      <p>{format!("{} {}", word_count_no_punctuation, "words")}</p>
    }
}
