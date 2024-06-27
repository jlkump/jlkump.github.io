use yew::prelude::*;
use yew_icons::{Icon, IconId};
use stylist::yew::styled_component;
use yew_router::components::Link;

use crate::{display::{atoms::{card_preview::CardPreview, slant_display::SlantDisplay}, organisms::{animated_banner::AnimatedBanner, page_header::PageHeader}, post::{use_posts, PostData}, theme::use_theme}, router::Route};

#[styled_component(Home)]
pub fn home() -> Html {
    let theme = use_theme();
    let style = css!(
        r#"
            .card-holder {
                display: flex; 
                justify-content: space-evenly;
                flex-wrap: wrap;
                margin-top: 30px;
                margin-bottom: 30px;
                padding-top: 20px;
                padding-bottom: 20px;
                width: 100vw;
            }

            .card-holder > div {
                margin-right: 20px;
                margin-left: 20px;
            }



            .navbar > a:hover {
                color: ${link_hover};
            }

            .about {
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                padding: 20px;
            }

            .about-container {
                display: flex;
            }

            .profile {
                border-radius: 100%; 
                width: 300px; 
                height: 300px;
                margin-right: 30px;
                border: 6px solid ${dark_panel};
                box-shadow: 0 10px 30px 5px rgba(0, 0, 0, 0.2);
                vertical-align: middle;
                align-self: center;
                margin-bottom: 20px;
            }

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
        "#,
        link_hover=theme.link_hover,
        dark_panel=theme.panel_color_dark
    );

    let posts = use_posts();
    html! {
        <div class={style} style="overflow-x: hidden;">
            <PageHeader should_scroll_top=true/>
            <AnimatedBanner class={css!("padding-top: 0px; padding-bottom: 40px;")} size=250>
                <h1>{"Landon Kump"}</h1>
                <h2 class="sub-header">{"Graphics Programmer"}</h2>
            </AnimatedBanner>
            <SlantDisplay bg_color={theme.panel_color_secondary.clone()} is_left=false style="display: flex; flex-direction: column; padding-top: 20px; padding-bottom: 35px;">
                <h3 style="font-size: 3.5em; margin: 0; text-align: center;">{"Projects"}</h3>
                <hr/>
                <div class={"card-holder"}>
                    {for get_project_display(*posts)}
                </div>
                <Link<Route> to={Route::BlogNav} classes={css!("align-self: center;")}><button style="z-index: 4;">{"See All"}</button></Link<Route>>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_primary.clone()} style="display: flex; flex-direction: column; padding-top: 20px; padding-bottom: 35px;">
                <h3 style="font-size: 3.5em; margin: 0; text-align: center;">{"About"}</h3>
                <hr/>
                <div class="about-container">
                    <div class="blog-content">
                        <p>
                            {"I am a recent CS graduate from the University of Texas at Austin, specializing in game programming and graphics. A story-teller and worldbuilder at heart, I aim to bring the best experience possible to players, captivating them with stunning visuals and completely immersing them in other worlds. In particular, I enjoy the technical aspects of game development, including engine development, graphics, physics, and networking."}
                        </p>
                        <p style="align-content: middle;">
                            {"To that end, I have studied throughly with personal projects in all major engines as well as in lower-level software, such as OpenGL and CUDA. I have also programmed for and directed the publishing of "}<a href="https://store.steampowered.com/app/2962650/Banana_Cowboy/">{"Banana Cowboy on Steam"}<Icon icon_id={IconId::BootstrapSteam} style="vertical-align: middle; margin-left: 4px;"/></a>{" with a team of talented inter-disciplinary students. Having completed my degree, I am now looking to work further on challenging projects. If you're looking for a dedicated and innovative programmer to bring your next project or idea to life, I'd love to connect. Get in touch and let's make it happen!"}
                        </p>
                    </div>
                    <img class="profile" src="/images/Profile-picture.png" />
                </div>
                
                <Link<Route> to={Route::Contact} classes={css!("align-self: center;")}><button style="z-index: 4;">{"Get In Touch!"}</button></Link<Route>>
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_secondary.clone()} is_left=false>
                <h3 style="font-size: 3.5em; margin: 0; text-align: center;">{"Artistic Portfolio"}</h3>
                <hr/>
                <div class={"card-holder"}>
                    <CardPreview img="/images/personal-art/Digital-Fracture.png" 
                        class={css!("margin: 10px;")}
                        invert=true
                        title="Fracture"
                        link_text="View full portfolio"
                        route={Route::Portfolio}
                    />
                    <CardPreview img="/images/personal-art/Digital-Magic-Aura.png" 
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
        </div>
    }
}

fn get_project_display(posts: &PostData) -> Vec<Html> {
    let mut res = vec![];
    for post in &posts.showcase_posts {
        if let Some(post) = posts.posts.get(post) {
            res.push(
                html! {
                    <CardPreview img={post.showcase_img.clone()} 
                        style="margin: 10px;"
                        title={post.title.clone()}
                        text={post.brief.clone()}
                        route={post.route.clone()}
                        invert={post.invert_showcase_text}
                    />
                }
            );
        }
    }
    res
}