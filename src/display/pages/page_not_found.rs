use yew::prelude::*;
use stylist::yew::styled_component;

#[styled_component(PageNotFound)]
pub fn home() -> Html {
    html! {
        <h1>{"404: Not Found"}</h1>
    }
}