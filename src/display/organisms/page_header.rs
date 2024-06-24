use yew::prelude::*;
use stylist::yew::styled_component;
use yew_icons::{Icon, IconId};
use yew_router::components::Link;

use crate::{display::theme::use_theme, router::Route};


#[styled_component(PageHeader)]
pub fn page_header() -> Html {
    let theme = use_theme();
    let style = css!(r#"
            display: flex;
            
            position: absolute; 
            right: 20px; 
            padding-top: 10px;

            z-index: 100;

            & > a {
                display: block;
                text-decoration: none;
                margin: 10px;
            }

            & > a:visited {
                color: ${link};
            }
    "#,
    link=theme.link,
    );
    html! {
        <div class={style}>
            <a href="https://github.com/jlkump"><Icon icon_id={IconId::BootstrapGithub} width={"2em".to_owned()} height={"2em".to_owned()}/></a>
            <a href="https://www.linkedin.com/in/jonathan-kump-a73b7722b/"><Icon icon_id={IconId::BootstrapLinkedin} width={"2em".to_owned()} height={"2em".to_owned()}/></a>
            <Link<Route> to={Route::Contact}><Icon icon_id={IconId::BootstrapEnvelopeFill} width={"2em".to_owned()} height={"2em".to_owned()}/></Link<Route>>
        </div>
    }
}