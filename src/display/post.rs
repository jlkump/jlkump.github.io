use std::{collections::HashMap, ops::Deref};

use once_cell::sync::Lazy;
use stylist::yew::styled_component;
use yew::prelude::*;

use crate::router::Route;

use super::{pages::blog::BlogPost, theme::Theme};

mod data;

#[derive(Debug, PartialEq, Clone)]
pub struct Post {
    pub title: String,
    pub brief: String,
    pub showcase_img: String,
    pub invert_showcase_text: bool,
    pub route: Route,
    html_fn: fn(&Context<BlogPost>, &Theme) -> Html,
}

impl Post {
    fn new(
        title: &str, 
        brief: &str, 
        showcase_img: &str, 
        invert_showcase_text: bool, 
        route: Route,
        html_fn: fn(&Context<BlogPost>, &Theme) -> Html,
    ) -> Post {
        Post { title: title.to_string(), brief: brief.to_string(), showcase_img: showcase_img.to_string(), invert_showcase_text, route, html_fn}
    }

    pub fn get_post_html(&self, ctx: &Context<BlogPost>, theme: &Theme) -> Html {
        (self.html_fn)(ctx, theme)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PostData {
    pub posts: HashMap<String, Post>,
    pub showcase_posts: Vec<String>,
}

impl PostData {
    pub fn get(&self, post: &str) -> Option<&Post> {
        self.posts.get(post)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct PostContext {
    inner: UseStateHandle<&'static PostData>,
}

fn get_posts() -> &'static PostData {
    static POSTS: Lazy<PostData> = Lazy::new(|| data::get_post_data());
    &POSTS
}

impl PostContext {
    fn new(inner: UseStateHandle<&'static PostData>) -> Self {
        Self { inner }
    }
}

impl Deref for PostContext {
    type Target = &'static PostData;

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