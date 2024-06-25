use yew::prelude::*;
use stylist::yew::styled_component;

use crate::display::theme::use_theme;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub img: String,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
}

#[styled_component(ArtDisplay)]
pub fn art_display(props: &Props) -> Html {
    let theme = use_theme();
    let style = css!(r#"
        position: relative;

        .art-display {
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
    
            padding: 50px;
            width: 14rem;
            height: 14rem;
            border-radius: 10px;
    
            font-family: 'Roboto', sans-serif;
            cursor: pointer;
    
            overflow: hidden;
            position: relative;
    
            transition: box-shadow 0.3s ease-out;
            z-index: 1;
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
        }

        .art-display h2 {
            position: absolute;
            inset: auto auto 30px auto;
            margin: 0;
            transition: opacity 0.2s ease-out;
            font-family: 'Roboto Condensed', sans-serif;
            font-weight: bold;
            font-size: 2em;
            text-align: center;
            text-transform: uppercase;
            opacity: 0%;
            color: ${light};
            -webkit-text-stroke-width: 3px;
            -webkit-text-stroke-color: ${dark};
        }

        .art-display:hover {
            transition: box-shadow 0.3s ease-in;
            box-shadow: 0 10px 10px 5px rgba(0, 0, 0, 0.2);
        }

        .art-display:hover h2 {
            transition: opacity 0.4s ease-in;
            opacity: 100%;
        }

        .art-display::before {
            background: linear-gradient(to right, rgba(255, 255, 255, 0) 0%, rgba(255, 255, 255, 0.15) 100%);
            content: "";
            display: block;
            height: 100%;
            left: -75%;
            position: absolute;
            top: 0;
            transform: skewX(-25deg);
            width: 50%;
            z-index: 2;
        }
        
        .art-display:hover::before, .art-display:focus::before {
            animation: shine 0.45s;
        }

        .art-display.active {
            width: 80vw;
            height: 70vh;
            z-index: 6;
            transition: box-shadow 0.0s;
            box-shadow: 0 0 0 0 rgba(0, 0, 0, 0.2);
        }

        .art-display.active:hover {
            box-shadow: 0 0 0 0 rgba(0, 0, 0, 0.2);
        }
        .art-display.active:hover::before, .art-display.active:focus::before {
            animation: none;
        }

        .art-display.active img {
            transition: opacity 0.2s ease-out;
            opacity: 1;
            object-fit: contain;
        }

        .placeholder.active {
            position: relative;
            padding: 50px;
            width: 18rem;
            height: 18rem;
            border-radius: 10px;
        }

        .bg {
            position: relative;
            transition: background 0.1s ease-out;
            z-index: 0;
            margin: 15px;
        }

        .bg.active {
            margin: 0px;
            position: fixed;
            display: flex;
            justify-content: center;
            align-items: center;
            z-index: 5;

            transition: background 0.1s ease-in;
            top: 0%;
            left: 0%;
            width: 100vw;
            height: 100vh;
            background: rgba(0, 0, 0, 0.2);
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
    "#, 
    light=theme.text_invert.as_str(),
    dark=theme.text_default.as_str()
    );

    let active = use_state(|| false);
    let onclick = {
        let active = active.clone();
        Callback::from(move |_: MouseEvent| {active.set(!*active)})
    };
    let active_class = if *active { "active" } else { "" };
    html! {
        <div class={classes!(style)} {onclick}>
            <div class={classes!("bg", active_class)}>
                <div class={classes!("art-display", props.class.clone(), active_class)} style={props.style.clone()}>
                    <img src={props.img.clone()} alt={format!("Image for {} card", props.title.clone())} />
                    <h2>{props.title.clone()}</h2>
                </div>
            </div>
            <div class={classes!("placeholder", active_class)}></div>
        </div>
    }
}