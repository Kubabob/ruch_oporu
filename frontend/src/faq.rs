use yew::prelude::*;

use crate::faq_components::qa_section::FaqSection;
use crate::faq_components::faq_text_section::FAQTextSection;

#[function_component(FAQ)]
pub fn faq() -> Html {
    html! {
        <>
        <FAQTextSection />
        <FaqSection />
        </>
    }
}