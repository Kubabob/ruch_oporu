use yew::prelude::*;

#[function_component(FAQTextSection)]
pub fn faq_text_section() -> Html {
    html! {
        <section class="about-section">
            <div class="about-content">
                <h2 class="section-title">{"FAQ"}</h2>
                <div class="text-content">
                    <p>{"Masz pytania? Nie rozumiesz naszej inicjatywy?"}</p>
                    <p>{"Chcesz się doedukować lub rozwinąć pewne wątpliwości?"}</p>
                    <p>{"Zjedź niżej i skorzystaj z przygotowanego przez nas FAQ."}</p>
                </div>
            </div>
        </section>
    }
}