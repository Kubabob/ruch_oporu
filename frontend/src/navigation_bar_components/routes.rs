use yew::prelude::*;
use yew_router::prelude::*;
use crate::landing_page::LandingPage;
use crate::about_us::AboutUs;
use crate::coming_outs::ComingOuts;
use crate::faq::FAQ;
use crate::form::FormPage;
use crate::contact::Contact;

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

    #[at("/regulamin")]
    Rules,

    #[at("/rodo")]
    RODO,

    #[at("/dokumenty/zgoda-na-wizerunek")]
    ImageConsent,

    #[at("/kontakt")]
    Contact,
    
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
        Route::Rules => html! { "Regulamin" },
        Route::RODO => html! { "RODO" },
        Route::ImageConsent => html! { "Zgoda na wizerunek" },
        Route::Contact => html! { <Contact /> },
        Route::NotFound => html! { "404 Not Found" },
    }
}