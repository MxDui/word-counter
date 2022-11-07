use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub color: Color,
    pub on_load: Callback<String>,
}

#[derive(PartialEq)]
pub enum Color {
    Ok,
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Ok => "ok".to_owned(),
        }
    }
}

#[styled_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    let stylesheet = style!(
        r#"
      .normal {
        color: white;
      }

      .ok {
        color: black;
      }

      .error {
        color: red;
      }
    "#
    )
    .unwrap();


    html! {
      <div class={stylesheet}>
        <h1 class={props.color.to_string()}>{&props.title}</h1>
      </div>
    }
}
