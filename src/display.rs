use std::fs;

use gloo::console::log;
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
            <script src="https://cdn.jsdelivr.net/gh/google/code-prettify@master/loader/run_prettify.js"></script>
        </>
    }
}

#[function_component(Root)]
fn root() -> Html {
    html! {
        <ThemeProvider>
            <App />
        </ThemeProvider>
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
        }}
        
        h2 {{
            color: {h2};
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
    "#
    ).try_into().unwrap()
}