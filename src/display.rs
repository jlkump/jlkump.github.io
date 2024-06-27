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
        text_highlight,
        gradient_dark,
        gradient_light,
        paper,
        paper_secondary,
        panel_color_dark,
        panel_color_accent,
        h1,
        h2,
        h3,
        h4,
        h5,
        h6,
        hr,
        link,
        link_hover,
        link_invert,
        link_hover_invert,
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

            font-family: "DM Serif Display", serif;
            font-weight: 400;
            font-style: normal;

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
                font-size: 2.5em;
            }}
            .sub-header {{
                font-size: 2em;
            }}
        }}

        h3 {{
            font-size: 3.5em; 
            text-align: center;
            margin-top: 5px; 
            margin-bottom: 5px; 
            color: {h3};
        }}

        h4 {{
            font-size: 1.5em; 
            text-indent: 40px;
            margin-top: 2.5px; 
            margin-bottom: 2.5px; 
            color: {h4};
        }}

        h5 {{
            font-size: 1.5em; 
            margin-top: 2px; 
            margin-bottom: 2px; 
            color: {h5};
        }}

        h6 {{
            color: {h6};
        }}

        hr {{
            border: 0px;
            border-top: 5px double {hr};
            margin-right: 10vw;
            margin-left: 10vw;
        }}

        p {{
            font-size: 1.5em;
            text-indent: 20px;
        }}

        blockquote {{
            font-size: 1.5em;
            margin-left: 30px;
            border-style: solid;
            border-width: 0px 0px 0px 15px;
            padding: 10px;
            border-color: {panel_color_accent};
            background: {panel_color_dark};
            color: {text_invert};
            font-family: "DM Serif Display", serif;
            font-weight: 400;
            font-style: italic;
        }}

        a, a:visited {{
            color: {link};
            text-decoration: none;
        }}

        a:hover, a:hover:visited {{
            color: {link_hover};
        }}

        code {{
            font-family: "Source Code Pro", monospace;
            font-optical-sizing: auto;
            font-weight: bold;
            font-style: normal;
            color: {text_colored};
        }}

        pre.prettyprint {{
            border: 3px solid {panel_color_accent};
            border-radius: 10px;
            text-wrap: wrap;
        }}

        @media screen and (max-width: 800px) {{
            code {{
                font-size: 0.75em;
            }}
        }}

        blockquote a {{
            color: {link_invert} !important;
        }}

        blockquote a:hover {{
            color: {link_hover_invert} !important;
        }}


        button {{
            color: {button};
            padding: 15px;
            border-radius: 10px;
            transition-duration: 0.4s;
            cursor: pointer;
            background: transparent;
            outline: none;
            font-family: "Rubik Mono One", monospace; 
            font-size: 2em;
            font-weight: bold;
            border: 4px solid {button};
        }}

        button:hover {{
            color: {button_press};
            border: 4px solid {button_press};
            background: rgba(0, 0, 0, 0.15);
        }}

        button:active {{
            box-shadow: 0 12px 16px 0 rgba(0,0,0,0.24), 0 17px 10px 0 rgba(0,0,0,0.19);
        }}

        ::-moz-selection {{
            color: {text_invert};
            background: {text_highlight};
        }}

        ::selection {{
            color: {text_invert};
            background: {text_highlight};
        }}

        .blog-content {{
            padding: 50px;
            -webkit-user-select: text; /* Safari */
            -ms-user-select: text; /* IE 10 and IE 11 */
            user-select: text; /* Standard syntax */
        }}

        .blog-content hr {{
            border-top: 3px dashed {h4}; 
            margin: 70px 0 20px 0;
        }}

        .split {{
            display: flex;
            flex-wrap: wrap;
        }}

        .media-display {{
            width: 60vw; 
            height: 500px;
        }}

        @media screen and (max-width: 800px) {{
            .split {{
                flex-direction: column;
            }}
            .blog-content {{
                padding: 20px;
            }}
            .media-display {{
                width: 80vw; 
                height: 300px;
            }}
        }}

        .references {{
            padding: 20px;
        }}

        .references a {{
            color: {link_invert};
        }}
        
        .references li {{
            font-size: 1.5em;
        }}
    "#
    ).try_into().unwrap()
}