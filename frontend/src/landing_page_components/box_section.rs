//use log::{info, warn, error};
use yew::prelude::*;

#[function_component(BoxSection)]
pub fn box_section() -> Html {

    let email = use_state(|| String::new());
    let message = use_state(|| String::new());

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            email.set(input.value());
        })
    };

    let on_message_change = {
        let message = message.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>();
            message.set(input.value());
        })
    };

    let on_submit = {
        let email = email.clone();
        let message = message.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            // Handle form submission (e.g., send to backend)
            log::info!("Form submitted!");
            log::info!("Email: {}", *email);
            log::info!("Message: {}", *message);

            // Clear form
            email.set(String::new());
            message.set(String::new());
        })
    };

    html! {
        <section class="box-section">
            <div class="box-grid">
                // Big Box 1 (Top)
                <div class="box box-1">
                    <div class="box-content">
                        <h2>{"→ KIM JESTEŚMY? ←"}</h2>
                        <p>{"Stowarzyszenie  „Gra na Orientacje” to wspólnota przyjaciół, założona jako odpowiedź na doświadczenie dyskryminacji /ze względu na orientację seksualną/ w ZHR. Inicjatywa jest tworzona przez byłe i obecne osoby członkowskie i osoby instruktorskie ZHR, w  myśl wartości, w których w tej organizacji zostałyśmy wychowane – odpowiedzialności za społeczeństwo, braterstwa i wzajemnego szacunku."}</p>
                        <h2>{"→ PO CO JEST TA STRONA? ←"}</h2>
                        <p>{
                            "Czujemy się zobowiązanx do zabrania głosu i podjęcia próby zmiany organizacji poprzez stworzenie bezpiecznej przestrzeni wypowiedzi
                            A jednocześnie za pomocą działań i interwencji artystycznych, zwrócić uwagę na przejawy dyskryminacji których doświadczyłyśmy, a których wiele osób jeszcze doświadcza, bądź doświadczy.
                            Ostatecznie wyjść poza to doświadczenie, tę organizację i działać szerzej.
                            "}</p>
                    </div>
                </div>

                // Small Box 2
                <div class="box box-small">
                    <div class="box-content">
                        <img src="image1.jpg" alt="Zdjęcie 1" class="box-image" />
                        <h3>{"Tytuł 1"}</h3>
                        <p>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}</p>
                        <a href="#" class="read-more">{"Czytaj więcej"}</a>
                    </div>
                </div>

                // Small Box 3
                <div class="box box-small">
                    <div class="box-content">
                        <img src="image2.jpg" alt="Zdjęcie 2" class="box-image" />
                        <h3>{"Tytuł 2"}</h3>
                        <p>{"Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."}</p>
                        <a href="#" class="read-more">{"Czytaj więcej"}</a>
                    </div>
                </div>

                // Small Box 4
                <div class="box box-small">
                    <div class="box-content">
                        <img src="image3.jpg" alt="Zdjęcie 3" class="box-image" />
                        <h3>{"Tytuł 3"}</h3>
                        <p>{"Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."}</p>
                        <a href="#" class="read-more">{"Czytaj więcej"}</a>
                    </div>
                </div>

                // Big Box 5 (Bottom)
                <div class="box box-4">
                    <div class="box-content">
                        <h2>{"NAPISZ DO NAS!"}</h2>
                        <form onsubmit={on_submit}>
                            <div class="form-group">
                                <label for="email">{"mail do kontaktu:"}</label>
                                <input
                                    type="email"
                                    id="email"
                                    placeholder="Twój adres e-mail..."
                                    value={(*email).clone()}
                                    oninput={on_email_change}
                                    required=true
                                />
                            </div>
                            <div class="form-group">
                                <label for="message">{"treść wiadomości:"}</label>
                                <textarea
                                    id="message"
                                    placeholder="Twoja wiadomość..."
                                    rows="4"
                                    value={(*message).clone()}
                                    oninput={on_message_change}
                                    required=true
                                ></textarea>
                            </div>
                            <button type="submit" class="submit-button">{"Wyślij"}</button>
                        </form>
                    </div>
                </div>
            </div>
        </section>
    }
}