use yew::prelude::*;
use yew_router::prelude::*;

use super::landing_page::LandingPage;
use super::team_stories_page::TeamStoriesPage;
use super::about_us_page::AboutUs;
use super::faq_page::FAQ;
use super::contact_page::ContactPage;
use super::histories_page::HistoriesPage;
use super::history_page::HistoryPage;
use super::exhibition_page::ExhibitionPage;
use super::movie_page::MoviePage;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/o-nas")]
    AboutUs,

    #[at("/nasze-historie")]
    HistoriesPage,

    #[at("/nasze-historie/:index")]
    HistoryPage {index: u8},

    #[at("/nasz-zespol")]
    TeamStoriesPage,

    #[at("/faq")]
    FAQ,


    #[at("/wystawa")]
    ExhibitionPage,

    #[at("/film")]
    MoviePage,

    // #[at("/dokumenty/zgoda-na-wizerunek")]
    // ImageConsent,

    #[at("/kontakt")]
    ContactPage,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <LandingPage /> },
        Route::AboutUs => html! { <AboutUs /> },
        Route::HistoriesPage => html! { <HistoriesPage /> },
        Route::HistoryPage { index } => html!(<HistoryPage index={index} />),
        Route::TeamStoriesPage => html!( <TeamStoriesPage /> ),
        Route::FAQ => html! { <FAQ /> },
        Route::ExhibitionPage => html! { <ExhibitionPage /> },
        Route::MoviePage => html! { <MoviePage /> },
        // Route::ImageConsent => html! { "Zgoda na wizerunek" },
        Route::ContactPage => html! { <ContactPage /> },
        Route::NotFound => html! { "404 Not Found" },
    }
}
