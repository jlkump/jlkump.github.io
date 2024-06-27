use yew::prelude::*;
use yew_router::prelude::*;

use crate::display::pages::{blog::{Blog, BlogNavigation}, contact::Contact, home::Home, page_not_found::PageNotFound, portfolio::Portfolio};

#[derive(Clone, Debug, Routable)]
pub(crate) enum Route {
    #[at("/")]
    Home,
    #[at("/Portfolio")]
    Portfolio,
    #[at("/Contact")]
    Contact,
    #[at("/Blog")]
    BlogNav,
    #[at("/Blog/:post")]
    Blog { post: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub fn try_blog_string(&self) -> Option<&str> {
        match self {
            Route::Blog { post } => Some(&post),
            _ => None,
        }
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Portfolio => html! { <Portfolio />},
        Route::Contact => html! { <Contact />},
        Route::Blog { post } => html! { <Blog {post}/> },
        Route::BlogNav => html! { <BlogNavigation /> },
        Route::NotFound => html! { <PageNotFound/> },
    }
}

#[derive(Properties, PartialEq)]
pub struct Props;

#[function_component(Router)]
pub fn router(_: &Props) -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}