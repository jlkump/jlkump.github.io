use yew::prelude::*;
use stylist::{yew::styled_component, Style};

use crate::display::theme::{use_theme, Theme};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Html,
    #[prop_or(150)]
    pub size: i32,
    #[prop_or(20)]
    pub layer: i32,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(Default)]
struct MousePos {
    x: f32,
    y: f32,
}

struct Rotation {
    x: f32,
    y: f32,
}

#[styled_component(AnimatedBanner)]
pub fn animated_banner(props: &Props) -> Html {
    let click_held = use_state(|| false);
    let mouse_tracker = use_state(|| MousePos::default());
    let rotation = use_state(|| Rotation { x: -30.0, y: 0.0});
    let rot_speed_x: f32 = 2.0;
    let rot_speed_y: f32 = 2.0;

    let onmousedown = {
        let click_held_clone = click_held.clone();
        Callback::from(move |_: MouseEvent| { click_held_clone.set(true) })
    };
    let onmouseup = {
        let click_held_clone = click_held.clone();
        Callback::from(move |_: MouseEvent| { click_held_clone.set(false) })
    };
    let onmouseover =  {
        let mouse_clone = mouse_tracker.clone();
        Callback::from(move |e: MouseEvent| {
            let x = e.offset_x() as f32;
            let y = e.offset_y() as f32;
            mouse_clone.set(MousePos { x, y });
        })
    };
    let onmousemove =  {
        let mouse_clone = mouse_tracker.clone();
        let click_held_clone = click_held.clone();
        let rotation_clone = rotation.clone();
        Callback::from(move |e: MouseEvent| {
            let delta_x;
            let delta_y;
            {
                let x = e.offset_x() as f32;
                let y = e.offset_y() as f32;
                let prev = &mouse_clone;
                delta_x = x - prev.x;
                delta_y = y - prev.y;
                mouse_clone.set(MousePos { x, y });
            }
            if *click_held_clone {
                let x = rotation_clone.x + delta_x.clamp(-1.0, 1.0) * rot_speed_x; // (delta_x * rot_speed_x).clamp(-max_rot_speed_x, max_rot_speed_x);
                let y = rotation_clone.y + delta_y.clamp(-1.0, 1.0) * rot_speed_y; // (delta_y * rot_speed_y).clamp(-max_rot_speed_y, max_rot_speed_y);
                rotation_clone.set( Rotation { x, y });
            }
        })
    };
    let onmouseleave = {
        let click_held_clone = click_held.clone();
        Callback::from(move |_: MouseEvent| {
            click_held_clone.set(false);
        })
    };

    let theme = use_theme();
    html! {
        <div style="display: flex; flex-direction: column; justify-content: center; position: relative;">
            <div class={classes!(props.class.clone(), get_animated_css(&theme, &format!("{}px", props.size), &*rotation))} {onmousedown} {onmouseup} {onmousemove} {onmouseover} {onmouseleave}>
            <div style="position: absolute; display: flex; flex-direction: column; justify-content: center; align-items: center; top: 0; left: 0; z-index: 2; width: 100%; height: 100%;">
                {props.children.clone()}
            </div>
                <div class={classes!("monitor")}>
                    <div class="camera o-x">
                        <div class="camera o-y">
                            <div class="camera o-z">
                                <div class="vr">
                                    {get_vr_layers(props.size, props.layer, *click_held)}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn get_vr_layers(size: i32, layer: i32, click_held: bool) -> Vec<Html> {
    let mut res = vec![];
    let size = size as f32;
    let layer = layer as f32;
    for i in 0..20 {
        let i = i as f32;
        res.push(
            html! {
                <div class="vr_layer" style={format!("transform: translateZ({}px)", (i * size) / layer  - size / 2.0)}>
                    <div class={if click_held {"vr_layer_item active" } else {"vr_layer_item"}} style={format!("animation-delay: {}ms", i * -210.0)}></div>
                </div>
            }
        );
    }
    res
}


fn get_animated_css(theme: &Theme, size: &str, rot: &Rotation) -> Style {
    let Theme {
        banner_cube_color,
        banner_cube_border_color,
        banner_sphere_color,
        banner_color_one,
        banner_color_two,
        banner_color_three,
        ..
    } = theme;
    Style::new(
        format!(
        r#"
            background: transparent;
            height: 60vh;
            overflow: hidden;
            display: flex;
            font-family: 'Anton', sans-serif;
            justify-content: center;
            align-items: center;
            perspective: 500px;

            .monitor {{
                position: relative;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                pointer-events: none;
                transform-style: preserve-3d;
            }}

            .vr_layer_item.active {{
                width: 40%;
                height: 40%;
                border-radius: 5px;
            }}

            .camera {{
                position: absolute;
                width: 100%;
                height: 100%;
                transform-style: preserve-3d;
                transition: 500ms;
            }}

            .camera.o-y {{
                transform: rotateY({1}deg);
            }}

            .camera.o-x {{
                transform: rotateX(-20deg);
            }}

            .camera.o-z {{
                animation: rotation 16000ms linear normal infinite;
                transition: 500ms;
            }}

            @keyframes rotation {{
                0% {{
                    transform: rotateZ(0deg);
                }}

                100% {{
                    transform: rotateZ(360deg);
                }}
            }}

            .vr {{
                position: absolute;
                top: 50%;
                left: 50%;
                width: {0};
                height: {0};
                transform: translate(-50%, -50%);
                transform-style: preserve-3d;
            }}

            .vr_layer {{
                position: absolute;
                display: flex;
                justify-content: center;
                align-items: center;
                width: 100%;
                height: 100%;
                border: 1px solid {banner_cube_border_color};
                background: {banner_cube_color};
                border-radius: 10px;
                transform: preserve-3d;
            }}

            .vr_layer_item {{
                width: 70%;
                height: 70%;
                border: 3px solid #fff;
                border-radius: 100%;
                background: {banner_sphere_color};
                animation: sphere 3000ms cubic-bezier(0.215, 0.61, 0.355, 1) alternate infinite, color 5000ms linear alternate infinite;
                transition: 500ms;
            }}

            @keyframes sphere {{
                0% {{
                    transform: scale(0) rotateZ(45deg);
                }}
                
                50% {{
                    transform: scale(0) rotateZ(45deg);
                }}
                
                100% {{
                    transform: scale(1) rotateZ(45deg);
                }}
            }}

            @keyframes color {{
                0% {{
                    border-color: {banner_color_one};
                }}
                
                50% {{
                    border-color: {banner_color_two};
                }}
                
                100% {{
                    border-color: {banner_color_three};
                }}
            }}
        "#,
        size,
        rot.x,
        )
    ).unwrap()
}