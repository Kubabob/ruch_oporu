use yew::prelude::*;

#[function_component(TextSection)]
pub fn text_section() -> Html {
    html! {
        <section class="about-section" style="background: #ffd6a542;">
            <div class="about-content">
                <h2 class="section-title">{"Anonimowe Coming-Out'y"}</h2>
                <div class="text-content">
                    <p>{"Wasze historie i doświadczenia, którymi podzieliliście się z nami"}</p>
                    <p>{"Chcesz, aby Twoja historia się tu pojawiła?"}</p>
                    <p>
                        {"Przejdź do zakładki "}
                        <a href="#/formularz">{"Opowiedz swoją historię"}</a>
                    </p>
                </div>
            </div>
        </section>
    }
}
