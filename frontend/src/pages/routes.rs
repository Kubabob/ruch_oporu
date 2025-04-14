use yew::prelude::*;
use yew_router::prelude::*;

use super::landing_page::LandingPage;
use super::form_page::FormPage;
use super::coming_outs_page::ComingOutsPage;
use super::about_us_page::AboutUs;
use super::faq_page::FAQ;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/o-nas")]
    AboutUs,

    #[at("/wysluchaj")]
    ComingOutsPage,

    #[at("/faq")]
    FAQ,

    #[at("/podziel-sie")]
    FormPage,

    // #[at("/regulamin")]
    // Rules,

    // #[at("/rodo")]
    // RODO,

    // #[at("/wystawa")]
    // Exhibition,

    // #[at("/film")]
    // Movie,

    // #[at("/inne-organizacje")]
    // OtherOrganizations,

    // #[at("/dokumenty/zgoda-na-wizerunek")]
    // ImageConsent,

    // #[at("/kontakt")]
    // Contact,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <LandingPage /> },
        Route::AboutUs => html! { <AboutUs /> },
        Route::ComingOutsPage => html! { <ComingOutsPage /> },
        Route::FAQ => html! { <FAQ /> },
        Route::FormPage => html! { <FormPage /> },
        // Route::Rules => html! { "Regulamin" },
        // Route::RODO => html! { "RODO" },
        // Route::Exhibition => html! { <Exhibition /> },
        // Route::Movie => html! { <Movie /> },
        // Route::OtherOrganizations => html! { <OtherOrganizations /> },
        // Route::ImageConsent => html! { "Zgoda na wizerunek" },
        // Route::Contact => html! { <Contact /> },
        Route::NotFound => html! { "404 Not Found" },
    }
}
