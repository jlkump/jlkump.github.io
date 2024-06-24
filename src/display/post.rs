use std::ops::Deref;

use once_cell::sync::Lazy;
use stylist::yew::styled_component;
use yew::prelude::*;

use crate::router::Route;

#[derive(Debug, PartialEq, Clone)]
pub struct Post {
    pub title: String,
    pub brief: String,
    pub showcase_img: String,
    pub invert_showcase_text: bool,
    pub route: Route,
    pub md_path: String,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct PostContext {
    inner: UseStateHandle<&'static Vec<Post>>,
}

fn get_posts() -> &'static Vec<Post> {
    static POSTS: Lazy<Vec<Post>> = Lazy::new(|| vec![
        Post { 
            title: "Custom Npr Shader".to_string(), 
            brief: "Exploring NPR (Non-Photorealistic Rendering) techniques with a custom post process shader aiming to create the effect of brushstrokes on a model in realtime. Made using OpenGL and C++.".to_string(), 
            showcase_img: "/images/npr-shader/Showcase.png".to_string(),
            invert_showcase_text: false,
            route: Route::Blog { post: "Opengl-NPR-Brush-Shader" .to_string()}, 
            md_path: "/projects/npr-brush-shader.md".to_string(),
        },
        Post { 
            title: "Inverse Kinematics".to_string(), 
            brief: "Exploring Inverse Kinematics using the FABRIK method. Made using GDExtension and C++.".to_string(),
            showcase_img: "/images/inverse-kinematics/ik-showcase.png".to_string(),
            invert_showcase_text: true, 
            route: Route::Blog { post: "Godot-Inverse-Kinematics" .to_string()}, 
            md_path: "/projects/inverse-kinematics-godot.md".to_string(),
        },
        Post { 
            title: "Water Rendering".to_string(), 
            brief: "Exploring a Screen-Space technique for rendering realistic water in real-time, covered in a 2010 GDC talk by Simon Green. Made using OpenGL and C++.".to_string(),
            showcase_img: "/images/waterflow/Final-water-cube.png".to_string(),
            invert_showcase_text: true, 
            route: Route::Blog { post: "Opengl-Water-Rendering" .to_string()}, 
            md_path: "/projects/opengl-water-rendering.md".to_string(),
        },
    ]);
    &POSTS
}

impl PostContext {
    pub fn new(inner: UseStateHandle<&'static Vec<Post>>) -> Self {
        Self { inner }
    }
}

impl Deref for PostContext {
    type Target = &'static Vec<Post>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}


#[styled_component(PostProvider)]
pub fn post_provider(props: &Props) ->  Html {
    let posts = use_state(|| get_posts());

    let post_ctx = PostContext::new(posts);

    html! {
        <ContextProvider<PostContext> context={post_ctx}>
            {props.children.clone()}
        </ContextProvider<PostContext>>
    }
}

#[hook]
pub(crate) fn use_posts() -> PostContext {
    use yew::use_context;
    use_context::<PostContext>().unwrap()
}