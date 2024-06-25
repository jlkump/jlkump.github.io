use yew::prelude::*;
use stylist::yew::styled_component;

use crate::{display::atoms::card_preview::CardPreview, router::Route};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub exclude: Route,
}

#[styled_component(PageFooter)]
pub fn page_footer(props: &Props) -> Html {
    let style = css!(
        r#"
            display: flex; 
            justify-content: space-evenly; 
            align-items: center;
            padding-top: 20px; 
            padding-bottom: 60px;

            @media screen and (max-width: 800px) {
                flex-direction: column;
            }
        "#
    );
    html! {
        <div class={style}>
            if props.exclude != Route::Home {
                <CardPreview img="/images/HomeCube.png" 
                    style="margin: 10px;"
                    title={"Home"}
                    route={Route::Home}
                    link_text="Go To Home Page"
                />
            }
            if props.exclude != Route::BlogNav {
                <CardPreview img="/images/npr-shader/Showcase.png" 
                    style="margin: 10px;"
                    title={"Projects"}
                    route={Route::Blog { post: "".to_string()}}
                    link_text="Go To Projects"
                />
            }
            if props.exclude != Route::Portfolio {
                <CardPreview img="/images/personal-art/Watercolor-crystaltree.png" 
                    style="margin: 10px;"
                    title={"Portfolio"}
                    route={Route::Portfolio}
                    invert=true
                    link_text="Go To Portfolio"
                />
            }
        </div>
    }
}