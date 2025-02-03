use yew::prelude::*;
use yew_router::prelude::*;
use crate::landing_page::LandingPage;
use crate::about_us::AboutUs;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    
    #[at("/o-nas")]
    AboutUs,
    
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <LandingPage /> },
        Route::AboutUs => html! { <AboutUs /> },
        Route::NotFound => html! { "404 Not Found" },
    }
}