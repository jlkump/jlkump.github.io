use yew::prelude::*;
use yew_icons::{Icon, IconId};
use stylist::{yew::styled_component, Style};
use yew_router::components::Link;

use crate::{display::{atoms::{code::CodeSnippet, slant_display::SlantDisplay, card_preview::CardPreview}, organisms::{animated_banner::AnimatedBanner, navbar::NavBar}, theme::{use_theme, Theme}}, router::Route};

#[styled_component(Home)]
pub fn home() -> Html {
    let theme = use_theme();
    let style = css!(
        r#"
            .navbar {
                display: flex;
                
                position: absolute; 
                right: 20px; 
                padding-top: 10px;

                z-index: 100;
            }

            .card-holder {
                display: flex; 
                justify-content: space-evenly;
                flex-wrap: wrap;
                margin-top: 30px;
                margin-bottom: 30px;
            }

            .card-holder > div {
                margin-right: 20px;
                margin-left: 20px;
            }

            .navbar > a {
                display: block;
                text-decoration: none;
                margin: 10px;
            }

            .navbar > a:visited {
                color: ${link};
            }

            .navbar > a:hover {
                color: ${link_hover};
            }

            @media screen and (max-width: 800px) {
                .card-holder {

                }
            }
        "#,
        link=theme.link,
        link_hover=theme.link_hover
    );
    let banner_style = css!(
        r#"
            .header {
                font-family: "Rubik Mono One", monospace; 
                font-weight: 400; 
                font-style: normal; 
                font-size: 7em;
                text-align: center;
                margin: 0px;
                transform-style: flat;
            }

            .subheader {
                font-family: "Rubik Mono One", monospace; 
                font-weight: 400; 
                font-style: normal; 
                text-align: center;
                font-size: 2em;
                margin-top: 10px;
                transform-style: flat;
            }

            @media screen and (max-width: 800px) {
                .header {
                    font-size: 4em;
                }
                .subheader {
                    font-size: 3em;
                }
            }
        "#
    );
    html! {
        <div class={style} style="overflow-x: hidden;">
            <div class="navbar">
                <a href="https://github.com/jlkump"><Icon icon_id={IconId::BootstrapGithub} width={"2em".to_owned()} height={"2em".to_owned()}/></a>
                <a href="https://www.linkedin.com/in/jonathan-kump-a73b7722b/"><Icon icon_id={IconId::BootstrapLinkedin} width={"2em".to_owned()} height={"2em".to_owned()}/></a>
                <Link<Route> to={Route::Contact}><Icon icon_id={IconId::BootstrapEnvelopeFill} width={"2em".to_owned()} height={"2em".to_owned()}/></Link<Route>>
            </div>
            <AnimatedBanner class={classes!(css!("padding-top: 0px; padding-bottom: 40px;"), banner_style)} size=250>
                <h1 class={"header"}>{"Landon Kump"}</h1>
                <h2 class={"subheader"}>{"Graphics Programmer"}</h2>
            </AnimatedBanner>
            <SlantDisplay bg_color={theme.panel_color_primary.clone()}>
                <h3 style="font-size: 3.5em; margin: 0; text-align: center;">{"Projects"}</h3>
            </SlantDisplay>
            <div class={"card-holder"}>
                <CardPreview img="/images/npr-shader/Showcase.png" 
                    class={css!("margin: 10px;")}
                    title="Custom Npr Shader" 
                    text="Exploring NPR (Non-Photorealistic Rendering) techniques with a custom post process shader aiming to create the effect of brushstrokes on a model in realtime. Made using OpenGL and C++." 
                    route={Route::Blog { post: "TODO".to_string()}}
                />
                <CardPreview img="/images/inverse-kinematics/ik-showcase.png" 
                    class={css!("margin: 10px;")}
                    invert=true
                    title="Inverse Kinematics" 
                    text="Exploring Inverse Kinematics using the FABRIK method. Made using GDExtension and C++." 
                    route={Route::Blog { post: "TODO".to_string()}}
                />
                <CardPreview img="/images/waterflow/Final-water-cube.png" 
                    class={css!("margin: 10px;")}
                    invert=true
                    title="Water Rendering" 
                    text="Exploring a Screen-Space technique for rendering realistic water in real-time, covered in a 2010 GDC talk by Simon Green. Made using OpenGL and C++." 
                    route={Route::Blog { post: "TODO".to_string()}}
                />
            </div>
            <SlantDisplay bg_color={theme.panel_color_primary.clone()}>
                <h3 style="font-size: 3.5em; margin: 0; text-align: center;">{"About"}</h3>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_secondary.clone()} is_left=false>
                <h3 style="font-size: 3.5em; margin: 0; text-align: center;">{"Artistic Portfolio"}</h3>
                <div class={"card-holder"}>
                    <CardPreview img="/images/personal-art/Digital-Fracture.png" 
                        class={css!("margin: 10px;")}
                        invert=true
                        title="Fracture" 
                        link_text="View full portfolio"
                        route={Route::Portfolio}
                    />
                    <CardPreview img="/images/personal-art/Digital-magic-aura.png" 
                        class={css!("margin: 10px;")}
                        invert=true
                        title="Node" 
                        link_text="View full portfolio"
                        route={Route::Portfolio}
                    />
                </div>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_primary.clone()}>
                <h3 style="font-size: 3.5em; margin: 0; text-align: center;">{"Contact"}</h3>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_secondary.clone()} is_left=false>

                <h1>{"Header 1"}</h1>
                <h2>{"Header 2"}</h2>
                <h3>{"Header 3"}</h3>
                <h4>{"Header 4"}</h4>
                <h5>{"Header 5"}</h5>
                <h6>{"Header 6"}</h6>
            </SlantDisplay>
            <CodeSnippet code={get_test_codeblock()}/>
        </div>
    }
}

fn get_test_codeblock() -> String {
    r#"
    struct Test {
        foo: String
    }

    pub fn test() -> Test {
        Test {
            foo: "I am a test!".to_string()
        }
    }
    "#.to_string()
}