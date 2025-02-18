use yew::prelude::*;
use web_sys::{File, HtmlInputElement, HtmlTextAreaElement};
use super::status_selector::StatusSelector;
use super::questions_section::{QuestionsSection, Status};
use crate::env;

#[function_component(FormSection)]
pub fn form_section() -> Html {

    // LGBT/Ally/Innx
    let status = use_state(|| Status::LGBT);

    // Historia uzytkownika
    let history = use_state(|| String::new());

    // Title wpisu
    let title = use_state(|| String::new());

    // Cytat który chciałbyś umieścić
    let quote = use_state(|| String::new());

    // Czy chcesz umieścić grafikę?
    let is_graphic = use_state(|| false);

    // graphic
    let graphic = use_state(|| Option::<File>::None);

    // Czy znajduja się osoby trzecie na grafice?
    let is_another = use_state(|| false);

    // Zgoda na wizerunek
    let image_consent = use_state(|| Option::<File>::None);

    // Czy zezwalasz na opublikowanie grafiki?
    let is_public_image = use_state(|| false);

    // Czy chcesz pozostać anonimowy?
    let is_nonanonymous = use_state(|| false);

    // podpis
    let signature = use_state(|| String::new());

    // Czy to jest autentyczny wpis?
    let is_authentic = use_state(|| false);

    // Wyrazam zgode na upublicznienie wpisu
    let is_public = use_state(|| false);

    // Czy zgadasz sie na wykorzystywanie fragmentów?
    let usage_consent = use_state(|| false);

    // Zgoda na regulamin
    let rules_consent = use_state(|| false);

    // Zgoda na RODO
    let rodo_consent = use_state(|| false);

    // Czy wypelniono formularz
    let form_submitted = use_state(|| false);

    // error message
    let error_message = use_state(|| Option::<String>::None);




    // LGBT/Ally/Innx on change
    let on_status_change = {
        let status = status.clone();
        Callback::from(move |new_status: Status| {
            //let input = e.target_unchecked_into::<HtmlInputElement>();
            status.set(new_status);
        })
    };

    // Historia uzytkownika on change
    let on_history_change = {
        let history = history.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlTextAreaElement>();
            history.set(input.value());
        })
    };

    // Title wpisu on change
    let on_title_change = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            title.set(input.value());
        })
    };

    // Cytat który chciałbyś umieścić on change
    let on_quote_change = {
        let quote = quote.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            quote.set(input.value());
        })
    };

    // Czy chcesz umieścić grafikę? on change
    let on_is_graphic_change = {
        let is_graphic = is_graphic.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            is_graphic.set(input.checked());
        })
    };

    // graphic on change
    let on_graphic_change = {
        let graphic = graphic.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            if let Some(file_list) = input.files() {
                if let Some(file) = file_list.get(0) {
                    graphic.set(Some(file));
                }
            }
        })
    };

    // Czy znajduja się osoby trzecie na grafice? on change
    let on_is_another_change = {
        let is_another = is_another.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            is_another.set(input.checked());
        })
    };

    // Zgoda na wizerunek on change
    let on_image_consent_change = {
        let image_consent = image_consent.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            if let Some(file_list) = input.files() {
                if let Some(file) = file_list.get(0) {
                    image_consent.set(Some(file));
                }
            }
        })
    };

    // Czy zezwalasz na opublikowanie grafiki? on change
    let on_is_public_image_change = {
        let is_public_image = is_public_image.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            is_public_image.set(input.checked());
        })
    };

    // Czy chcesz pozostać anonimowy? on change
    let on_is_anonymous_change = {
        let is_nonanonymous = is_nonanonymous.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            is_nonanonymous.set(input.checked());
        })
    };

    // podpis on change
    let on_signature_change = {
        let signature = signature.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            signature.set(input.value());
        })
    };

    // Czy to jest autentyczny wpis? on change
    let on_is_authentic_change = {
        let is_authentic = is_authentic.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            is_authentic.set(input.checked());
        })
    };

    // Wyrazam zgode na upublicznienie wpisu on change
    let on_is_public_change = {
        let is_public = is_public.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            is_public.set(input.checked());
        })
    };


    // Czy zgadasz sie na wykorzystywanie fragmentów? on change
    let on_usage_consent_change = {
        let usage_consent = usage_consent.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            usage_consent.set(input.checked());
        })
    };

    // Zgoda na regulamin on change
    let on_rules_consent_change = {
        let rules_consent = rules_consent.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            rules_consent.set(input.checked());
        })
    };

    // Zgoda na RODO on change
    let on_rodo_consent_change = {
        let rodo_consent = rodo_consent.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            rodo_consent.set(input.checked());
        })
    };
    

    // let on_status_change_clone1 = on_status_change.clone();
    // let on_status_change_clone2 = on_status_change.clone();

    let on_submit = {
        let status = status.clone();
        let history = history.clone();
        let title = title.clone();
        let quote = quote.clone();
        let is_graphic = is_graphic.clone();
        let graphic = graphic.clone();
        let is_another = is_another.clone();
        let image_consent = image_consent.clone();
        let is_public_image = is_public_image.clone();
        let is_nonanonymous = is_nonanonymous.clone();
        let signature = signature.clone();
        let is_authentic = is_authentic.clone();
        let is_public = is_public.clone();
        let usage_consent = usage_consent.clone();
        let rules_consent = rules_consent.clone();
        let rodo_consent = rodo_consent.clone();
        let form_submitted = form_submitted.clone();
        let error_message = error_message.clone();
    


        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if !*is_authentic {
                error_message.set(Some("Dziękujemy za chęć wypełnienia formularza, jednak nie możemy przyjąć nieautentycznego wpisu.".to_string()));
                form_submitted.set(false);
                return;
            }

            error_message.set(None);

            log::info!("Form submitted!");
            log::info!("status: {:?}", *status);
            log::info!("history: {}", *history);
            log::info!("title: {}", *title);
            log::info!("quote: {}", *quote);
            log::info!("is_graphic: {}", *is_graphic);
            if let Some(file) = graphic.as_ref() {
                log::info!("graphic file name: {}", file.name());
            } else {
                log::info!("No graphic file provided");
            };
            log::info!("is_another: {}", *is_another);
            if let Some(file) = image_consent.as_ref() {
                log::info!("image_consent file name: {}", file.name());
            } else {
                log::info!("No image_consent file provided");
            };
            log::info!("is_public_image: {}", *is_public_image);
            log::info!("is_nonanonymous: {}", *is_nonanonymous);
            log::info!("signature: {}", *signature);
            log::info!("is_authentic: {}", *is_authentic);
            log::info!("is_public: {}", *is_public);
            log::info!("usage_consent: {}", *usage_consent);
            log::info!("rules_consent: {}", *rules_consent);
            log::info!("rodo_consent: {}", *rodo_consent);


            // Clear form
            // status.set(String::new());
            status.set(Status::LGBT);
            history.set(String::new());
            title.set(String::new());
            quote.set(String::new());
            is_graphic.set(false);
            graphic.set(None);
            is_another.set(false);
            image_consent.set(None);
            is_public_image.set(false);
            is_nonanonymous.set(false);
            signature.set(String::new());
            is_authentic.set(false);
            is_public.set(false);
            usage_consent.set(false);
            rules_consent.set(false);
            rodo_consent.set(false);
            form_submitted.set(true);
        })
    };
    
    html! {
        <>
            <div class="form-page">    

                // Podziekowanie za wypelnienie formularza
                if *form_submitted {
                    <div class="thank-you-message">
                        <h1>{"Dziękujemy za wypełnienie formularza!"}</h1>
                        <p>{"Twój wpis został zapisany."}</p>
                    </div>
                } else {
                    <>
                        <h1>{"Opowiedz swoją historię"}</h1>
                        <h5>{"*obowiązkowe"}</h5>
                        // Pokaz error jesli wpis jest nieautentyczny
                        if let Some(error_message) = &*error_message {
                            <div class="error-message">
                                {error_message}
                            </div>
                        }
                        <form onsubmit={on_submit}>
                            // Wybór LGBT/Ally/Innx
                            // <div class="form-group">
                            //     <label>{"Kim jesteś?:"}</label>
                            //     <div class="radio-group">
                            //         <label>
                            //             <input
                            //                 type="radio"
                            //                 name="option"
                            //                 value="LGBT"
                            //                 checked={*status == "LGBT"}
                            //                 oninput={on_status_change}
                            //             />
                            //             {"LGBT"}
                            //         </label>
                            //         <label>
                            //             <input
                            //                 type="radio"
                            //                 name="option"
                            //                 value="Ally"
                            //                 checked={*status == "Ally"}
                            //                 oninput={on_status_change_clone1}
                            //             />
                            //             {"Ally"}
                            //         </label>
                            //         <label>
                            //             <input
                            //                 type="radio"
                            //                 name="option"
                            //                 value="Innx"
                            //                 checked={*status == "Innx"}
                            //                 oninput={on_status_change_clone2}
                            //             />
                            //             {"Innx"}
                            //         </label>
                            //     </div>
                            // </div>
                            <StatusSelector 
                                current_status={(*status).clone()} 
                                on_status_change={on_status_change} 
                            />



                            // Historia uzytkownika

                            <div class="form-group">
                                <label for="history">{"Twoja historia:*"}</label>
                                <textarea
                                    id="history"
                                    placeholder="Tutaj napisz swoją historię..."
                                    value={(*history).clone()}
                                    oninput={on_history_change}
                                    rows="4"
                                    required=true
                                ></textarea>
                            </div>


                            // Title wpisu
                            <div class="form-group">
                                <label for="title">{"Tytuł wpisu:"}</label>
                                <input
                                    type="text"
                                    id="title"
                                    placeholder="O czym opowiada Twoja historia w jednym zdaniu lub słowie?"
                                    value={(*title).clone()}
                                    oninput={on_title_change}
                                    //required=true
                                />
                            </div>


                            // Cytat który chciałbyś umieścić
                            <div class="form-group">
                                <label for="quote">{"Wybrany cytat/krótki fragment, który będzie wyszczególniony na stronie:"}</label>
                                <input
                                    type="text"
                                    id="quote"
                                    placeholder="Który fragment uważasz za najważniejszy?"
                                    oninput={on_quote_change}
                                    //required=true
                                />
                            </div>


                            // Czy chcesz umieścić grafikę?
                            <div class="form-group">
                                <label for="czy_grafika">{"Czy chcesz dodać grafikę lub zdjęcie do Twojej historii?"}</label>
                                <input
                                    type="checkbox"
                                    id="czy_grafika"
                                    checked={*is_graphic}
                                    oninput={on_is_graphic_change}
                                />
                            </div>


                            if *is_graphic {
                                // Dodaj grafikę
                                <div class="form-group">
                                    <label for="graphic">{"Dodaj grafikę:"}</label>
                                    <input
                                        type="file"
                                        id="graphic"
                                        multiple=false
                                        onchange={on_graphic_change}
                                    />
                                </div>

                                // Czy znajduja się osoby trzecie na grafice?
                                <div class="form-group">
                                    <label for="is_another">{"Czy na Twoim zdjęciu znajdują się osoby trzecie?"}</label>
                                    <input
                                        type="checkbox"
                                        id="is_another"
                                        checked={*is_another}
                                        oninput={on_is_another_change}
                                    />
                                </div>

                                if *is_another {
                                    // Dodaj zgode na wizerunek
                                    <div class="form-group">
                                        <label for="image_consent">
                                        {"Dodaj zgodę na udostępnienie wizerunku:\n"}
                                        <a href={env::IMAGE_CONSENT_URL} target="_blank">{"zgoda do pobrania"}</a>
                                        </label>
                                        <input
                                            type="file"
                                            id="image_consent"
                                            //multiple=true
                                            onchange={on_image_consent_change}
                                        />
                                    </div>
                                }

                                // Czy zezwalasz na opublikowanie grafiki?
                                <div class="form-group">
                                    <label for="is_public_image">{"Wyrażam zgodę na opublikowanie zdjęcia/grafiki na stronie."}</label>
                                    <input
                                        type="checkbox"
                                        id="is_public_image"
                                        checked={*is_public_image}
                                        oninput={on_is_public_image_change}
                                    />
                                </div>
                            }

                            // Czy chcesz pozostać anonimowy?
                            <div class="form-group">
                                <label for="is_nonanonymous">{"Czy chcesz podpisać się imieniem i nazwiskiem?"}</label>
                                <input
                                    type="checkbox"
                                    id="is_anoymous"
                                    checked={*is_nonanonymous}
                                    oninput={on_is_anonymous_change}
                                />
                            </div>

                            if *is_nonanonymous {
                                // podpis
                                <div class="form-group">
                                    <label for="signature">{"Podpis:"}</label>
                                    <input
                                        type="text"
                                        id="signature"
                                        placeholder="Imię i nazwisko"
                                        value={(*signature).clone()}
                                        oninput={on_signature_change}
                                        required=true
                                    />
                                </div>
                            }

                            // Czy to jest autentyczny wpis?
                            <div class="form-group">
                                <label for="is_authentic">{"Potwierdzam, że wpis opisuje moje osobiste doświadczenie jako osoba nieheteronormatywna lub wspierająca. Moja wypowiedź nie zawiera mowy nienawiści"}</label>
                                <input
                                    type="checkbox"
                                    id="is_authentic"
                                    checked={*is_authentic}
                                    oninput={on_is_authentic_change}
                                />
                            </div>

                            // Zgoda na upublicznienie wpisu
                            <div class="form-group">
                                <label for="is_public">
                                    {"Wyrażam zgodę na upublicznienie mojego wpisu na stronie "}
                                    <a href={env::DOMAIN} target="_blank">{"www.granaorientacje.pl"}</a>
                                </label>
                                <input
                                    type="checkbox"
                                    id="is_public"
                                    checked={*is_public}
                                    oninput={on_is_public_change}
                                    //required=true
                                />
                            </div>

                            // Czy zgadasz sie na wykorzystywanie fragmentów?
                            <div class="form-group">
                                <label for="usage_consent">{"Wyrażam zgodę na wykorzystywanie fragmentów tekstów do promocji projektu w mediach społecznościowych."}</label>
                                <input
                                    type="checkbox"
                                    id="usage_consent"
                                    checked={*usage_consent}
                                    oninput={on_usage_consent_change}
                                />
                            </div>

                            // Zgoda na regulamin
                            <div class="form-group">
                                <label for="rules_consent">
                                    {"Zgadzam się z regulaminem projektu dostępnym na stronie: "}
                                    <a href={env::RULES_URL} target="_blank">{"regulamin"}</a>
                                </label>
                                <input
                                    type="checkbox"
                                    id="rules_consent"
                                    checked={*rules_consent}
                                    oninput={on_rules_consent_change}
                                    required=true
                                />
                            </div>

                            // Zgoda na RODO
                            <div class="form-group">
                                <label for="rodo_consent">
                                    {"Wyrażam zgodę na przetwarzanie danych osobowych "}
                                    <a href={env::RODO_URL} target="_blank">{"RODO"}</a>
                                </label>
                                <input
                                    type="checkbox"
                                    id="rodo_consent"
                                    checked={*rodo_consent}
                                    oninput={on_rodo_consent_change}
                                    required=true
                                />
                            </div>

                            <button type="submit" class="submit-button">{"Submit"}</button>
                        </form>

                        
                    </>
                }
            </div>

            <div class="form-page">
                <QuestionsSection status={(*status).clone()} />
            </div>
        </>
    }
}
