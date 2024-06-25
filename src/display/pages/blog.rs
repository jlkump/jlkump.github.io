use yew::prelude::*;
use yew_router::prelude::*;
use stylist::yew::styled_component;

use crate::{display::{atoms::{card_preview::CardPreview, slant_display::SlantDisplay}, organisms::{animated_banner::AnimatedBanner, page_footer::PageFooter, page_header::PageHeader}, post::{use_posts, Post, PostData}, theme::{use_theme, Theme}}, router::Route};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub post: String,
}

#[function_component(Blog)]
pub fn blog(props: &Props) -> Html {
    let theme = use_theme();
    if let Some(post) = use_posts().get(&props.post) {
        html! {
            <div>
                <BlogPost {post} theme={(*theme).clone()} />
            </div>
        }
    } else {
        html! {
            <Redirect<Route> to={Route::NotFound} />
        }
    }
}

#[styled_component(BlogNavigation)]
pub fn blog_navigation() -> Html {
    let post_data = use_posts();
    let theme = use_theme();
    html! {
        <div>
            <PageHeader />
            <AnimatedBanner><h1>{"Projects"}</h1></AnimatedBanner>
            <SlantDisplay bg_color={theme.panel_color_primary.clone()} style="display: flex; flex-direction: column; align-items: center; padding-top: 20px; padding-bottom: 35px;">
                {get_project_display(&post_data)}
            </SlantDisplay>
            <SlantDisplay bg_color={theme.panel_color_secondary.clone()} is_left=false style="display: flex; flex-direction: column; align-items: center; padding-top: 20px; padding-bottom: 35px;">
                <h3>{"Other Pages"}</h3>
            </SlantDisplay>
            <PageFooter exclude={Route::BlogNav} />
        </div>
    }
}

fn get_project_display(posts: &PostData) -> Vec<Html> {
    let mut res = vec![];
    for post in posts.posts.values() {
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
    res
}


#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub post: &'static Post,
    pub theme: Theme,
}

pub struct BlogPost;

impl Component for BlogPost {
    type Message = ();

    type Properties = BlogPostProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        ctx.props().post.get_post_html(ctx, &ctx.props().theme)
    }
}