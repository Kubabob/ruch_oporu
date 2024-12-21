//use std::ops::Deref;

use yew::prelude::*;
use serde::Serialize;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize, Clone)]
pub struct SurveyForm {
    name: String,
    email: String,
    answer: String,
}

#[function_component(Survey)]
pub fn survey() -> Html {
    let name = use_state(|| "".to_string());
    let email = use_state(|| "".to_string());
    let answer = use_state(|| "".to_string());
    //let submitted = use_state(|| false);

    let submit_survey = {
        let name = name.clone();
        let email = email.clone();
        let answer = answer.clone();
        //let submitted = submitted.clone();

        Callback::from(move |_| {
            let name_value = name.clone().to_string();
            let email_value = email.clone().to_string();
            let answer_value = answer.clone().to_string();
            //let submitted = submitted.clone();

            spawn_local(async move {
                let client = Client::new();
                let form = SurveyForm {
                    name: name_value,
                    email: email_value,
                    answer: answer_value,
                };

                let _ = client.post("http://127.0.0.1:8080/survey")
                    .json(&form)
                    .send()
                    .await;

                
                //submitted.set(true);
            });
        })
    };

    html! {
        <>
            <header>
                <h1>{ "Ankieta" }</h1>
            </header>
            <div class="card">
                if *submitted {
                    <p>{ "Dziękujemy za wypełnienie ankiety!" }</p>
                } else {
                    <form>
                        <div>
                            <label>{ "Imię: " }</label>
                            <input
                                value={(*name).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    name.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())
                                })}
                            />
                        </div>
                        <div>
                            <label>{ "Email: " }</label>
                            <input
                                value={(*email).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    email.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())
                                })}
                            />
                        </div>
                        <div>
                            <label>{ "Odpowiedź: " }</label>
                            <textarea
                                value={(*answer).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    answer.set(e.target_unchecked_into::<web_sys::HtmlTextAreaElement>().value())
                                })}
                            />
                        </div>
                        <button type="button" onsubmit={submit_survey}>{ "Wyślij" }</button>
                    </form>
                }
            </div>
        </>
    }
}

/*
pub fn show_survey() -> Html {
    let surveys = use_state(|| Vec::new());

    let fetch_surveys = {
        let surveys = surveys.clone();
        Callback::from(move |_| {
            let surveys = surveys.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Ok(response) = client.get("http://127.0.0.1:8080/survey")
                    .send()
                    .await
                {
                    if let Ok(survey_responses) = response.json::<Vec<SurveyForm>>().await {
                        surveys.set(survey_responses);
                    }
                }
            });
        })
    };
    /*
    <div>
        <h2>{ "Ankieta" }</h2>
        <button onclick={fetch_surveys}>{ "Pokaz ankiety" }</button>

    </div>
    */
}

*/