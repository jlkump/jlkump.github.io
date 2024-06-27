use yew::prelude::*;
use stylist::{yew::styled_component, Style};

use crate::display::theme::{use_theme, Theme};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    #[prop_or(None)]
    pub starting_line_num: Option<i32>, // Use NONE when there are no line numbers
    #[prop_or_default]
    pub language: Option<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
}

#[styled_component(CodeSnippet)]
pub fn code_snippet(props: &Props) -> Html {
    let theme = use_theme();
    html! {
        if let Some(line_num) = props.starting_line_num {
            <pre class={classes!(format!("prettyprint linenums:{}", line_num), props.class.clone(), get_code_style(&theme))} style={props.style.clone()}>
                {get_code_block(props)}
            </pre>
        } else {
            <pre class={classes!("prettyprint", get_code_style(&theme), props.class.clone())} style={props.style.clone()}>
                {get_code_block(props)}
            </pre>
        }
    }
}

fn get_code_block(props: &Props) -> Html {
    html! {
        if let Some(language) = &props.language {
            <code class={format!("language-{}", language)}>{props.children.clone()}</code>
        } else {
            <code>{props.children.clone()}</code>
        }
    }
}

fn get_code_style(theme: &Theme) -> Style {
    Style::new(
    r#"
        /* SPAN elements with the classes below are added by prettyprint. */
        .pln { color: #000 }  /* plain text */

        -webkit-user-select: text; /* Safari */
        -ms-user-select: text; /* IE 10 and IE 11 */
        user-select: text; /* Standard syntax */
        code {
            font-family: "Source Code Pro", monospace;
            font-optical-sizing: auto;
            font-weight: bold;
            font-style: normal;
        }

        @media screen {
            .str { color: #089 }  /* string content */
            .kwd { color: #008 }  /* a keyword */
            .com { color: #800 }  /* a comment */
            .typ { color: #606 }  /* a type name */
            .lit { color: #066 }  /* a literal value */
            /* punctuation, lisp open bracket, lisp close bracket */
            .pun, .opn, .clo { color: #660 }
            .tag { color: #008 }  /* a markup tag name */
            .atn { color: #606 }  /* a markup attribute name */
            .atv { color: #080 }  /* a markup attribute value */
            .dec, .var { color: #606 }  /* a declaration; a variable name */
            .fun { color: red }  /* a function name */
        }

        /* Use higher contrast and text-weight for printable form. */
        @media print, projection {
            .str { color: #060 }
            .kwd { color: #006; font-weight: bold }
            .com { color: #600; font-style: italic }
            .typ { color: #404; font-weight: bold }
            .lit { color: #044 }
            .pun, .opn, .clo { color: #440 }
            .tag { color: #006; font-weight: bold }
            .atn { color: #404 }
            .atv { color: #060 }
        }

        /* Put a border around prettyprinted code snippets. */
        pre.prettyprint { padding: 2px; border: 1px solid #888 }

        /* Specify class=linenums on a pre to get line numbering */
        ol.linenums { margin-top: 0; margin-bottom: 0 } /* IE indents via margin-left */
        li.L0,
        li.L1,
        li.L2,
        li.L3,
        li.L5,
        li.L6,
        li.L7,
        li.L8 { list-style-type: none }
        /* Alternate shading for lines */
        li.L1,
        li.L3,
        li.L5,
        li.L7,
        li.L9 { background: #eee }
    "#).unwrap()
}