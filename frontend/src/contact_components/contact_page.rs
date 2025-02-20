use yew::prelude::*;

#[function_component(ContactPage)]
pub fn contact_page() -> Html {
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
        <section class="box-section" style="background: #ffc6ff3d;">
            <div class="box-grid">
                // Contact Box 4
                <div class="box box-4">
                    <div class="box-content">
                        <h2>{"Napisz do nas za pomocą szybkiej wiadomości"}</h2>
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
                        <p class="email">
                            {"lub skontaktuj się przez naszą elektroniczną skrzynkę pocztową na adres "}
                            <a href="mailto:granaorientacje@gmail.com" class="email-link">
                                <i class="fas fa-envelope email-icon"></i>
                                {"granaorientacje@gmail.com"}
                            </a>
                            {"."}
                        </p>
                    </div>
                </div>

                // Big Box 1 (Top)
                <div class="box box-1">
                    <div class="box-content">
                        <h2>{"→ Chcesz z nami porozmawiać? ←"}</h2>
                        <p>{"Napisz na nasz adres mailowy, opisz o czym chcesz porozmawiać i umów się na rozmowę online"}</p>
                        <h2>{"→ Chcesz się zaangażować? ←"}</h2>
                        <p>{"Poniżej znajdziesz listę możliwych opcji. Napisz do nas!"}</p>
                        <ul>
                            <li>{"Opcja 1"}</li>
                            <li>{"Opcja 2"}</li>
                        </ul>
                    </div>
                </div>


            </div>
        </section>
    }
}
