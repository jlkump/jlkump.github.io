use yew::prelude::*;
use stylist::yew::styled_component;
use yew_icons::{Icon, IconId};
use yew_router::components::Link;

use crate::router::Route;


#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(false)]
    pub should_scroll_top: bool
}

#[styled_component(PageHeader)]
pub fn page_header(props: &Props) -> Html {
    let style = css!(r#"
            display: flex;
            
            position: absolute; 
            right: 20px; 
            padding-top: 10px;

            z-index: 3;

            & > a {
                display: block;
                text-decoration: none;
                margin: 10px;
                z-index: 3;
            }
    "#);
    let should_scroll_top = props.should_scroll_top;
    use_effect_with((),
        move |_| {
            if should_scroll_top {
                scroll_to_top();
            }
    });
    html! {
        <div class={style}>
            <a href="https://github.com/jlkump"><Icon icon_id={IconId::BootstrapGithub} width={"2em".to_owned()} height={"2em".to_owned()}/></a>
            <a href="https://www.linkedin.com/in/jonathan-kump-a73b7722b/"><Icon icon_id={IconId::BootstrapLinkedin} width={"2em".to_owned()} height={"2em".to_owned()}/></a>
            <Link<Route> to={Route::Contact}><Icon icon_id={IconId::BootstrapEnvelopeFill} width={"2em".to_owned()} height={"2em".to_owned()}/></Link<Route>>
        </div>
    }
}

pub fn scroll_to_top() {
    let window = web_sys::window();
    if let Some(window) = window {
        window.scroll_to_with_scroll_to_options(web_sys::ScrollToOptions::new().top(0.0).behavior(web_sys::ScrollBehavior::Instant));
    }
}