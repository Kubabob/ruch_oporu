use yew::prelude::*;
use crate::about_us_components::about_us_text_section::AboutUsTextSection;

#[function_component(AboutUs)]
pub fn about_us() -> Html {
    html! {
        <>
            <AboutUsTextSection />
        </>
    }
}