use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "william";
    log!("my name is: ", name);

    let my_object = MyObject {
        username: name.to_owned(),
        favorite_language: "rust".to_string(),
    };
    log!(
        "my object is: ",
        serde_json::to_string_pretty(&my_object).unwrap_or("".to_string())
    );
    let my_class = "mi-clase1";

    let message: Option<&str> = None; //Some("I am message");

    html! {
        <>
            <h1 class={my_class}>{"Hello World"}</h1>
            if my_class == "mi-clase" {
                <p>{"This is my first yew app!"}</p>
            } else {
                <p>{"This is not my first yew app!"}</p>
            }

            if let Some(message1) = message {
                <p>{message1}</p>
            } else {
                <p>{"No message"}</p>
            }
        </>
    }
}
