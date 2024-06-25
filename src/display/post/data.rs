use std::collections::HashMap;

use stylist::yew::styled_component;
use yew::{html, Context, Html, Properties};

use crate::{display::{atoms::slant_display::SlantDisplay, organisms::{animated_banner::AnimatedBanner, page_footer::PageFooter, page_header::PageHeader}, pages::blog::Blog, theme::use_theme}, router::Route};

use super::{Post, PostData};

mod inverse_kinematics_godot;

#[derive(Properties, PartialEq)]
struct Props {
    title: String,
    #[prop_or_default]
    children: Html,
    #[prop_or_default]
    flip_bottom: bool,
}

#[styled_component(BlogTemplate)]
fn blog_template(props: &Props) -> Html {
    let theme = use_theme();
    html! {
        <div>
            <PageHeader />
            <AnimatedBanner><h1>{props.title.clone()}</h1></AnimatedBanner>
            {props.children.clone()}
            if props.flip_bottom {
                <SlantDisplay bg_color={theme.panel_color_secondary.clone()} is_left=false><h3>{"Other Pages"}</h3></SlantDisplay>
            } else {
                <SlantDisplay bg_color={theme.panel_color_primary.clone()}><h3>{"Other Pages"}</h3></SlantDisplay>
            }
            <PageFooter exclude={Route::BlogNav}/>
        </div>
    }
}

pub fn get_post_data() -> PostData {
    let post_list = vec![
        inverse_kinematics_godot_post()
    ];
    let mut posts = HashMap::new();
    let mut showcase_posts = vec![];
    for p in post_list.into_iter() {
        if showcase_posts.len() < 3 {
            showcase_posts.push(p.route.try_blog_string().unwrap().to_string());
        }
        posts.insert(p.route.try_blog_string().unwrap().to_string(), p);
    }

    PostData { 
        posts, 
        showcase_posts,
    }
}

fn inverse_kinematics_godot_post() -> Post {
    Post::new(
        "Inverse Kinematics", 
        "Exploring Inverse Kinematics using the FABRIK method. Made using GDExtension and C++.", 
        "/images/inverse-kinematics/ik-showcase.png", 
        true, 
        Route::Blog { post: "Godot-Inverse-Kinematics" .to_string()},
        inverse_kinematics_godot::page,
    )
}