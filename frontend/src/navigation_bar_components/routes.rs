use crate::about_us::AboutUs;
use crate::coming_outs::ComingOuts;
use crate::contact::Contact;
use crate::exhibition::Exhibition;
use crate::faq::FAQ;
use crate::form::FormPage;
use crate::landing_page::LandingPage;
use crate::movie::Movie;
use crate::other_organizations::OtherOrganizations;
use yew::prelude::*;
use yew_router::prelude::*;

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

    #[at("/wystawa")]
    Exhibition,

    #[at("/film")]
    Movie,

    #[at("/inne-organizacje")]
    OtherOrganizations,

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
        Route::Exhibition => html! { <Exhibition /> },
        Route::Movie => html! { <Movie /> },
        Route::OtherOrganizations => html! { <OtherOrganizations /> },
        Route::ImageConsent => html! { "Zgoda na wizerunek" },
        Route::Contact => html! { <Contact /> },
        Route::NotFound => html! { "404 Not Found" },
    }
}
