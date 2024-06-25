use yew::prelude::*;
use yew_icons::{Icon, IconId};
use stylist::yew::styled_component;

use crate::{display::{atoms::slant_display::SlantDisplay, organisms::{animated_banner::AnimatedBanner, page_header::PageHeader, page_footer::PageFooter}, theme::use_theme}, router::Route};

#[derive(Properties, PartialEq)]
pub struct Props;

#[styled_component(Contact)]
pub fn contact(_: &Props) -> Html {
    let theme = use_theme();

    let style = css!(r#"
            .contact > p {
                font-size: 1.5em;
            }

            .contact > div > a {
                font-size: 2em; 
                display: flex; 
                align-items: center;
            }

            @media screen and (max-width: 800px) {
                .about-container {
                    flex-direction: column;
                }

                .contact > p {
                    font-size: 1.5em;
                }

                .contact > div > a {
                    font-size: 1em;
                }
            }
    "#);
    html! {
        <div class={style}>
            <PageHeader />
            <AnimatedBanner><h1 style="margin-top: 40px;">{"Contact"}</h1></AnimatedBanner>
            <div class="contact" style="padding-top: 20px; padding-bottom: 60px;">
                <div style="display: flex; justify-content: space-evenly; align-items: center;">
                    <a href="https://www.linkedin.com/in/jonathan-kump-a73b7722b/">
                    <Icon icon_id={IconId::BootstrapLinkedin} style="margin-right: 10px;" width="1em" height="1em"/>{"Jonathan Landon Kump"}
                    </a>
                    <a href="landon200@gmail.com">
                    <Icon icon_id={IconId::BootstrapEnvelopeFill} style="margin-right: 10px;" width="1em" height="1em"/>{"Landon2002@gmail.com"}
                    </a>
                </div>
                <p style="text-align: center;">
                    {"If you would like to get in contact, please use the email above or message me through Linked-In."}
                </p>
            </div>
            <SlantDisplay bg_color={theme.panel_color_secondary.clone()} is_left=false>
                <h3 style="font-size: 3.5em; margin: 0; text-align: center;">{"Other Pages"}</h3>
            </SlantDisplay>
            <PageFooter exclude={Route::Contact}/>
        </div>

    }
}