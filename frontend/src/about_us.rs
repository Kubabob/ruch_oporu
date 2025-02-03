use yew::prelude::*;
use crate::about_us_components::text_section::TextSection;

#[function_component(AboutUs)]
pub fn about_us() -> Html {
    html! {
        <>
            <TextSection />
        </>
    }
}