use yew::prelude::*;
use yew_router::prelude::*;

use crate::display::pages::{home::Home, page_not_found::PageNotFound};

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route {
    #[at("/")]
    Home,
    #[at("/Portfolio")]
    Portfolio,
    #[at("/Contact")]
    Contact,
    #[at("/Blog/:post")]
    Blog { post: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    // TODO: Redirect most routes if not logged-in
    match routes {
        Route::Home => html! { <Home/> },
        Route::Portfolio => html! { <Redirect<Route> to={Route::Home} />},
        Route::Contact => html! { <Redirect<Route> to={Route::Home} />},
        Route::Blog { post } => html! { <Redirect<Route> to={Route::Home} />},
        Route::NotFound => html! { <PageNotFound/> },
    }
}

#[derive(Properties, PartialEq)]
pub struct Props;

#[function_component(Router)]
pub fn router(_: &Props) -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}