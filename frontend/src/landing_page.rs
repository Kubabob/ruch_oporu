use yew::prelude::*;
use crate::landing_page_components::nav_bar::NavBar;
use crate::landing_page_components::hero::Hero;
use crate::landing_page_components::box_section::BoxSection;
use crate::landing_page_components::patrons::Patrons;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <>
            <NavBar />
            <Hero />
            <BoxSection />
            <Patrons />            
            // Your page content here
        </>
    }
}