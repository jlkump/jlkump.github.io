use yew::prelude::*;
use stylist::yew::styled_component;
use yew_icons::{Icon, IconId};
use yew_router::components::Link;

use crate::{display::theme::use_theme, router::Route};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub img: String,
    pub title: String,
    #[prop_or_default]
    pub text: Option<String>,
    #[prop_or("Find out more".to_string())]
    pub link_text: String,
    pub route: Route,
    #[prop_or(false)]
    pub invert: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
}

#[styled_component(CardPreview)]
pub fn card_preview(props: &Props) -> Html {
    let theme = use_theme();
    let title_inset_height = if props.text.is_some() { 220 } else { 90 };
    let style = css!(r#"
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        padding: 50px;
        font-family: 'Roboto', sans-serif;

        width: 18rem;
        height: 18rem;
        border-radius: 10px;
        overflow: hidden;
        position: relative;
        color: ${text};
        box-shadow: 0 10px 30px 5px rgba(0, 0, 0, 0.2);

        & img {
            position: absolute;
            object-fit: cover;
            width: 100%;
            height: 100%;
            top: 0;
            left: 0;
            opacity: 0.9;
            transition: opacity 0.2s ease-out;
        }
        & h2 {
            position: absolute;
            inset: auto auto 30px 30px;
            margin: 0;
            transition: inset 0.3s 0.3s ease-out, color 0.3s 0.3s ease-out;
            font-family: 'Roboto Condensed', sans-serif;
            font-weight: bold;
            text-transform: uppercase;
            color: ${invert};
        }
        & p, & a {
            position: absolute;
            opacity: 0;
            max-width: 80%;
            transition: opacity 0.3s ease-out;
        }
        & p {
            font-size: 1.25em;
            inset: auto auto 60px 30px;
        }
        & a {
            inset: auto auto 40px 30px;
            color: inherit;
            text-decoration: none;
        }
        &:hover h2 {
            inset: auto auto ${title_inset_height}px 30px;
            transition: inset 0.3s ease-out, color 0.3s ease-out;
            color: ${text};
        }
        &:hover p, &:hover a {
            opacity: 1;
            transition: opacity 0.5s 0.1s ease-in;
        }
        &:hover img {
            transition: opacity 0.3s ease-in;
            opacity: 0.2;
        }

        @media screen and (max-width: 800px) {
            width: 14rem;
            height: 14rem;
        }
    "#, 
    text=theme.text_default, 
    invert={if props.invert { theme.text_invert.as_str() } else { theme.text_default.as_str() }},
    title_inset_height=title_inset_height
    );
    html! {
        <div class={classes!(style, props.class.clone())} style={props.style.clone()}>
            <img src={props.img.clone()} alt={format!("Image for {} card", props.title.clone())} />
            <div>
                <h2>{props.title.clone()}</h2>
                if let Some(text) = props.text.clone() {
                <p>{text}</p>
                }
            </div>
            <Link<Route> to={props.route.clone()} classes={css!("display: flex; align-items: center;")}>
            {props.link_text.clone()}
            <Icon icon_id={IconId::FontAwesomeSolidArrowRight} class={css!("margin-left: 10px;")} width={"2em".to_owned()} height={"2em".to_owned()}/></Link<Route>>
        </div>
    }
}
