use yew::prelude::*;

#[function_component(TextSection)]
pub fn text_section() -> Html {
    html! {
        <section class="about-section">
            <div class="about-content">
                <h2 class="section-title">{"Anonimowe Coming-Out'y"}</h2>
                <div class="text-content">
                    <p>{"Wasze historie i doświadczenia, którymi podzieliliście się z nami"}</p>
                    <p>{"Chcesz, aby Twoja historia się tu pojawiła?"}</p>
                    <p>
                        {"Przejdź do zakładki "}
                        <a href="/twoja-historia">{"Twoja historia"}</a>
                    </p>
                </div>
            </div>
        </section>
    }
}