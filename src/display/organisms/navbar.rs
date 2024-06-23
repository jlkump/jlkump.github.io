use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::{platform::spawn_local, prelude::*};
use yew_icons::{Icon, IconId};
use yew::{html, Html};
use yew_router::{components::Link, hooks::use_navigator, navigator::Navigator};

use crate::{display::theme::{use_theme, Theme, ThemeKind}, router::Route};


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
}

#[styled_component(NavBar)]
pub fn nav_bar(props: &Props) -> Html {
    let menu_open = use_state(|| false);
    let onclick = {
        let menu_open_clone = menu_open.clone();
        Callback::from(move |_| menu_open_clone.set(!*menu_open_clone))
    };
    let theme = use_theme();


    let page_style = css!(
        r#"
            display: flex;
            flex-direction: row;
            height: 100%;

            .sidebar {
                flex: 20%;
                transition: 0.5s;
                overflow-y: auto;
                overflow-x: hidden;
            }

            .content {
                flex: 100%;
                overflow-x: hidden;
                position: relative;
            }

            .sidebar.closed {
                flex: 0;
                border-right: 0px;
            }

            .exit_sidebar {
                display: none;
            }


            .navbar {
                position: -webkit-sticky; /* Safari */
                position: sticky;
                top: 0;

                z-index: 100;
            }

            .navbar-button {
                position: absolute;
                border: 3px solid ${navbar};
                color: ${navbar};
                background: ${paper};
                mix-blend-mode: normal;
                padding: 5px;
                border-radius: 5px;
                margin: 20px;
                cursor: pointer;
            }

            .menu {
                left: 0;
                padding-bottom: 2px;
            }

            .theme-toggle {
                right: 20px;
                display: flex;
                justify-content: center;
                align-items: center;
            }

            @media screen and (max-width: 800px) {
                .theme-toggle {
                    right: 0;
                }

                .sidebar {
                    border-right: 0px;
                }

                .sidebar, .content {
                    flex: 100%;
                }

                .content.closed, .sidebar.closed {
                    flex: 0;
                }

                .exit_sidebar {
                    display: flex;
                }
            }
        "#,
        navbar=theme.navbar,
        paper=theme.paper,
    );
    let body_style = css!(
        r#"
            display: flex;
            flex-direction: column;
        "#
    );
    let body_classes = if *menu_open {
        classes!("content", "closed", body_style)
    } else {
        classes!("content", body_style)
    };

    let theme_toggle_onclick = {
        let theme_clone = theme.clone();
        Callback::from(move |_: MouseEvent| {
            theme_clone.toggle();
        })
    };

    html! {
        <div class={page_style}>
            <SideBar sidebar_open={*menu_open} exit_callback={onclick.clone()} />
            <div class={body_classes}>
                <div class="navbar">
                    <div class="menu navbar-button" onclick={onclick}>
                        <Icon icon_id={IconId::FeatherMenu} width={"2.5em".to_owned()} height={"2.5em".to_owned()}/>
                    </div>
                    <div>
                    </div>
                    <div class={"theme-toggle navbar-button"} onclick={theme_toggle_onclick}>
                        if theme.kind() == ThemeKind::Dark {
                                <Icon icon_id={IconId::BootstrapMoonFill} width={"2.5em".to_owned()} height={"2.5em".to_owned()}/>
                        } else {
                            <Icon icon_id={IconId::BootstrapSunFill} width={"2.5em".to_owned()} height={"2.5em".to_owned()}/>
                        }
                    </div>
                </div>
                {props.children.clone()}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct SideBarProps {
    sidebar_open: bool,
    exit_callback: Callback<MouseEvent>,
}

#[styled_component(SideBar)]
fn sider_bar(props: &SideBarProps) -> Html {
    let theme = use_theme();
    let padding = if props.sidebar_open {
        15
    } else {
        0
    };
    let sidebar_style = css!(
        r#"
            background: ${sidebar};
            display: flex;
            flex-direction: column;
            align-items: center;
            width: 100%;
            padding: ${padding}px;

            ul {
                list-style-type: none;
                width: 100%;
                padding: 0;
            }

            li {
                border-radius: 10px;
                cursor: pointer;
                display: flex;
                align-items: center;
                padding: 10px;
                font-size: 1.5em;
            }

            li:hover {
                background: rgb(255, 255, 255, 0.2);
                color: ${invert};
            }
        "#,
        sidebar=theme.sidebar,
        padding=padding,
        invert=theme.text_invert,
    );

    let classes = if props.sidebar_open {
        classes!("sidebar", sidebar_style)
    } else {
        classes!("sidebar", "closed", sidebar_style)
    };

    let copyright_style = css!("color: #343; font-size: 0.5em; display: flex; align-items: center; justify-content: center;");

    let exit_icon_color = css!("color: #100; position: absolute; margin: 7px; top: 0; right: 0;");

    let navigator = use_navigator().unwrap();

    html! {
        <div class={classes}>
            <div style="display: flex; flex-direction: column; flex; width: 100%;">
                <div class={classes!("exit_sidebar", exit_icon_color)} onclick={props.exit_callback.clone()}>
                    // <Icon icon_id={IconId::FontAwesomeSolidXmark} width={"2.5em".to_owned()} height={"2.5em".to_owned()}/>
                </div>
            </div>
            <h3 style="font-size: 2em;">
                {"Menu"}
            </h3>
            <div style="width: 100%;">
                <hr/>
            </div>
            <ul>
                <li>{"Test"}</li>
                <li>{"Test"}</li>
                <li>{"Test"}</li>
            </ul>


            <div class={copyright_style} style="margin-top: auto; width: 100%;">
                {"Copyright (c) 2024 by J. Landon Kump"}
            </div>
        </div>
    }
}

fn get_route_callback(navigator: &Navigator, route: Route) -> Callback<MouseEvent> {
    let nav_clone = navigator.clone();
    Callback::from(move |_: MouseEvent| nav_clone.push(&route))
}

fn get_bar_style(theme: &Theme) -> Style {
    Style::new(format!(
        r#"
            position: sticky;
            position: -webkit-sticky;
            top: 0;
            flex: 10%;
            display: flex;
            flex-direction: row;
            flex-wrap: nowrap;
            justify-content: space-between;
            z-index: 100;
            
            margin: 5px;

            background: transparent;
            border: 2px solid {};
            border-radius: 5px;
        "#,
        theme.navbar
    )).unwrap()
}