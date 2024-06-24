use std::fs;

use gloo::console::log;
use post::PostProvider;
use stylist::{yew::{styled_component, Global}, Style, StyleSource};
use theme::{use_theme, Theme, ThemeProvider};
use yew::prelude::*;

pub(crate) mod atoms;
pub(crate) mod molecules;
pub(crate) mod organisms;
pub(crate) mod pages;
pub(crate) mod post;
pub(crate) mod theme;

use crate::router::Router;

pub fn run() {
    yew::Renderer::<Root>::new().render();
}

#[styled_component(App)]
fn app() -> Html {
    let theme = use_theme();

    html! {
        <>
            <Global css={get_global_style(&theme)} />
            <Router />
        </>
    }
}

#[function_component(Root)]
fn root() -> Html {
    html! {
        <PostProvider>
            <ThemeProvider>
                <App />
            </ThemeProvider>
        </PostProvider>
    }
}

fn get_global_style(theme: &Theme) -> StyleSource {
    let Theme {
        text_default,
        text_invert,
        text_colored,
        gradient_dark,
        gradient_light,
        paper,
        paper_secondary,
        h1,
        h2,
        h3,
        h4,
        h5,
        h6,
        hr,
        link,
        link_hover,
        link_visited,
        scroll_bar,
        scroll_bar_hover,
        button,
        button_press,
        ..
    } = theme;
    format!(
    r#"
        --paper: {paper};
        --paper_secondary: {paper_secondary};

        body {{
            margin: 0px;
            color: {text_default};
            background-image: radial-gradient(ellipse at bottom, {gradient_light} 0%, {gradient_dark} 100%);
            background-position: center;
            background-repeat: no-repeat;
            background-attachment: fixed;
            height: 100vh;

            -webkit-user-select: none; /* Safari */
            -ms-user-select: none; /* IE 10 and IE 11 */
            user-select: none; /* Standard syntax */
        }}

        /* width */
        ::-webkit-scrollbar {{
            width: 0px;
        }}

        /* Track */
        ::-webkit-scrollbar-track {{
            background: {gradient_dark};
        }}

        /* Handle */
        ::-webkit-scrollbar-thumb {{
            background: {scroll_bar};
        }}

        /* Handle on hover */
        ::-webkit-scrollbar-thumb:hover {{
            background: {scroll_bar_hover};
        }}

        h1 {{
            color: {h1};
            font-family: "Rubik Mono One", monospace; 
            font-weight: 400; 
            font-style: normal; 
            font-size: 7em;
            text-align: center;
            margin: 0px;
            transform-style: flat;
        }}
        
        h2 {{
            color: {h2};
        }}

        .sub-header {{
            font-family: "Rubik Mono One", monospace; 
            font-weight: 400; 
            font-style: normal; 
            text-align: center;
            font-size: 2em;
            margin-top: 10px;
            transform-style: flat;
        }}

        @media screen and (max-width: 800px) {{
            h1 {{
                font-size: 4em;
            }}
            .sub-header {{
                font-size: 3em;
            }}
        }}

        h3 {{
            color: {h3};
        }}

        h4 {{
            color: {h4};
        }}

        h5 {{
            color: {h5};
        }}

        h6 {{
            color: {h6};
        }}

        hr {{
            color: {hr};
        }}

        a {{
            color: {link};
        }}

        a:hover {{
            color: {link_hover};
        }}

        a:visited {{
            color: {link_visited};
        }}

        button {{
            color: {text_default};
            padding: 10px;
            border-radius: 5px;
            transition-duration: 0.4s;
            cursor: pointer;
            background: transparent;
            outline: none;
            font-size: 1em;
            font-weight: bold;
            border: 4px solid {button};
        }}

        button:hover {{
            box-shadow: 0 12px 16px 0 rgba(0,0,0,0.24), 0 17px 10px 0 rgba(0,0,0,0.19);
        }}

        button:active {{
            box-shadow: none;
            transition-duration: 0.0s;
            color: {button_press};
            border: 4px solid {button_press};
        }}
    "#
    ).try_into().unwrap()
}