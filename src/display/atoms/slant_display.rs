use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
    pub bg_color: AttrValue,
    #[prop_or(true)]
    pub is_left: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: AttrValue,
}

#[styled_component(SlantDisplay)]
pub fn slant_display(props: &Props) -> Html {
    let container = if props.is_left {
        css!(
            r#"
                position: relative; 
                width: 100%; 
                background: ${bg};
                margin-top: 28px;
                margin-bottom: 28px;
                padding-top: 0.5em;
                padding-bottom: 0.5em;
                
                &::after {
                    position:absolute;
                    content: " ";
                    display:block;
                    left: 0;
                    bottom: -30px;
                    border-style: solid;
                    border-width: 0 0 30px 100vw;
                    border-color: transparent transparent transparent ${bg};
                    padding-top: 5px;
                }
                    
                &::before {
                    position:absolute;
                    content: " ";
                    display:block;
                    left: 0;
                    top: -30px;
                    border-style: solid;
                    border-width: 30px 0 0 100vw;
                    border-color: transparent transparent transparent ${bg};
                    padding-bottom: 5px;
                }
                "#,
                bg=props.bg_color
        )
    } else {
        css!(
            r#"
            position: relative; 
            width: 100%; 
            background: ${bg};
            margin-top: 28px;
            margin-bottom: 28px;
            padding-top: 0.5em;
            padding-bottom: 0.5em;
            
            &::after {
                position:absolute;
                content: " ";
                    display:block;
                    left:0;
                    bottom:-30px;
                    border-style: solid;
                    border-width: 0 100vw 30px 0;
                    border-color: transparent ${bg} transparent transparent;
                    padding-top: 5px;
                }
    
                &::before {
                    position:absolute;
                    content: " ";
                    display:block;
                    left: 0;
                    top: -28px;
                    border-style: solid;
                    border-width: 0 0 30px 100vw;
                    border-color: transparent transparent ${bg} transparent;
                }
            "#,
            bg=props.bg_color
        )
    };
    html! {
        <div class={classes!(container, props.class.clone())} style={props.style.clone()}>
            {props.children.clone()}
        </div>
    }
}