use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use stylist::{yew::styled_component, style, Style};

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

const STYLE_FILE : &str = include_str!("main.css");
// #[function_component(App)]
#[styled_component(App)]
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
    let tasks = vec![
        "Learn Rust",
        "Learn Yew",
        "Make awesome web apps",
        "Take over the world"];

    // let stylesheet = style!(
    //     r#"
    //         ul {
    //             color: red;
    //         }
    //         p {
    //             color: blue;
    //         }
            
    //     "#
    // ).unwrap();

    let stylesheet = Style::new(STYLE_FILE).unwrap();
    
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
            <div class={stylesheet}>
                <ul>
                    {list_task_to_li(tasks)}
                </ul>
                <p>{"Hello World"}</p>
                <h2 class={css!("color: yellow; font-size: 75px;")}>{"Mundo feliz"}</h2>
            </div>

            
        </>
    }
}


fn list_task_to_li(tasks: Vec<&str>) -> Vec<Html> {
    tasks.iter().map(|task| html!{<li>{task}</li>}).collect()
}