use yew::prelude::*;

use crate::faq_components::faq_text_section::FAQTextSection;
use crate::faq_components::qa_section::FaqSection;

#[function_component(FAQ)]
pub fn faq() -> Html {
    html! {
        <>
            <div class="faq-wrapper">
                <FAQTextSection />
                <FaqSection />
            </div>
        </>
    }
}
