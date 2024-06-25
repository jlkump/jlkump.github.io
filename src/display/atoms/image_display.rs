use yew::prelude::*;
use stylist::yew::styled_component;

use crate::display::theme::use_theme;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub src: String,
    #[prop_or_default]
    pub alt: String,
    #[prop_or_default]
    pub href: String,
    #[prop_or_default]
    pub ref_text: String,
    #[prop_or("18rem".to_string())]
    pub width: String,
    #[prop_or("18rem".to_string())]
    pub height: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
}

#[styled_component(Image)]
pub fn image(props: &Props) -> Html {
    let theme = use_theme();
    let style = css!(r#"
        position: relative;

        .art-display {
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
    
            padding: 50px;
            width: ${width};
            height: ${height};
            overflow: hidden;
            position: relative;
    
            z-index: 1;
            transition: box-shadow 0.3s ease-out;
            box-shadow: 0 0px 0px opx rgba(0, 0, 0, 0.2);
        }

        .art-display img {
            position: absolute;
            object-fit: cover;
            width: 100%;
            height: 100%;
            top: 0;
            left: 0;

            opacity: 0.9;

            transition: opacity 0.2s ease-out;

            border-radius: 10px;
            cursor: pointer;
            z-index: 2;
        }

        .art-display:hover {
            transition: box-shadow 0.3s ease-in;
            box-shadow: 0 10px 30px 5px rgba(0, 0, 0, 0.2);
        }

        .art-display.active {
            width: 80vw;
            height: 70vh;
            z-index: 6;
        }

        .art-display.active img {
            opacity: 1;
            object-fit: contain;
        }

        .art-display.active {
            transition: opacity 0.2s ease-out, box-shadow 0.0s;
            box-shadow: 0 0 0 0 rgba(0, 0, 0, 0.2);
        }

        .art-display.active:hover {
            box-shadow: 0 0 0 0 rgba(0, 0, 0, 0.2);
        }

        .placeholder.active {
            position: relative;
            padding: 50px;
            width: ${width};
            height: ${height};
            border-radius: 10px;
        }

        .bg {
            position: relative;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            z-index: 0;
            margin: 15px;
        }

        .bg.active {
            margin: 0px;
            position: fixed;
  
            z-index: 5;

            top: 0%;
            left: 0%;
            width: 100vw;
            height: 100vh;
            background: rgba(0, 0, 0, 0.2);
        }

        .bg a {
            position: relative;
            display: none;
            margin: 0;
            margin-top: 5px;
            font-size: 1.25em;
            text-align: center;
            border-radius: 5px;
            width: ${width};
            overflow-wrap: break-word;
            color: ${text_dark};
            z-index: 2;
        }

        
        .bg.active a {
            display: inline;
            opacity: 100%;
            background: ${paper};
            z-index: 7;
        }
        .bg.active a:hover {
            color: ${text_default};
        }
            
        @keyframes shine {
            100% {
                left: 125%;
            }
        }

        @media screen and (max-width: 800px) {
            .art-display {
                width: 10rem;
                height: 10rem;
            }
        }
    "#, paper=theme.paper_secondary, text_dark=theme.text_dark, text_default=theme.text_default,
        width=props.width, height=props.height,
    );

    let active = use_state(|| false);
    let onclick = {
        let active = active.clone();
        Callback::from(move |_: MouseEvent| {active.set(!*active)})
    };
    let refclick = {
        let active = active.clone();
        Callback::from(move |_: MouseEvent| 
            {if *active {active.set(false)}})
    };
    let active_class = if *active { "active" } else { "" };
    html! {
        <div class={classes!(style)}>
            <div class={classes!("bg", active_class)} onclick={refclick.clone()}>
                <div class={classes!("art-display", props.class.clone(), active_class)} style={props.style.clone()}>
                    <img src={props.src.clone()} alt={props.alt.clone()} {onclick} />
                </div>
                <a href={props.href.clone()} onclick={refclick}>{props.ref_text.clone()}</a>
            </div>
            <div class={classes!("placeholder", active_class)}></div>
        </div>
    }
}