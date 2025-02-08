use yew::prelude::*;
use yew_router::prelude::*;
use crate::landing_page::LandingPage;
use crate::about_us::AboutUs;
use crate::coming_outs::ComingOuts;
use crate::faq::FAQ;
use crate::form::FormPage;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    
    #[at("/o-nas")]
    AboutUs,

    #[at("/coming-outs")]
    ComingOuts,

    #[at("/faq")]
    FAQ,

    #[at("/formularz")]
    FormPage,
    
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <LandingPage /> },
        Route::AboutUs => html! { <AboutUs /> },
        Route::ComingOuts => html! { <ComingOuts /> },
        Route::FAQ => html! { <FAQ /> },
        Route::FormPage => html! { <FormPage /> },
        Route::NotFound => html! { "404 Not Found" },
    }
}