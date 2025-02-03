use yew::prelude::*;

#[function_component(TextSection)]
pub fn text_section() -> Html {
    html! {
        <section class="about-section">
            <div class="about-content">
                <h2 class="section-title">{"O nas"}</h2>
                <div class="text-content">
                    <h2>{"→ KIM JESTEŚMY? ←"}</h2>
                    <p>{"Stowarzyszenie  „Gra na Orientacje” to wspólnota przyjaciół, założona jako odpowiedź na doświadczenie dyskryminacji /ze względu na orientację seksualną/ w ZHR. Inicjatywa jest tworzona przez byłe i obecne osoby członkowskie i osoby instruktorskie ZHR, w  myśl wartości, w których w tej organizacji zostałyśmy wychowane – odpowiedzialności za społeczeństwo, braterstwa i wzajemnego szacunku."}</p>
                    <h2>{"→ CZYM SIĘ ZAJMUJEMY? ←"}</h2>
                    <p>{"Treść"}</p>
                    <h2>{"→ PO CO JEST TA STRONA? ←"}</h2>
                    <p>{
                        "Czujemy się zobowiązanx do zabrania głosu i podjęcia próby zmiany organizacji poprzez stworzenie bezpiecznej przestrzeni wypowiedzi
                        A jednocześnie za pomocą działań i interwencji artystycznych, zwrócić uwagę na przejawy dyskryminacji których doświadczyłyśmy, a których wiele osób jeszcze doświadcza, bądź doświadczy.
                        Ostatecznie wyjść poza to doświadczenie, tę organizację i działać szerzej.
                        "}</p>
                </div>
            </div>
        </section>
    }
}