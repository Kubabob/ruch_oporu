use serde::{Deserialize, Serialize};
use yew::prelude::*;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;


#[derive(Serialize, Deserialize)]
pub struct SurveyNumber {
    number: i64
}

#[function_component(SurveyCounter)]
pub fn survey_counter() -> Html {
    let survey_number = use_state(|| 0);
    {
        let survey_number = survey_number.clone();
        use_effect_with( (),
            move |_| {
                //let survey_number = survey_number.clone();
                spawn_local(async move {
                    let client = Client::new();
                    match client.get("http://127.0.0.1:8080/ankieta/number")
                    .send()
                    .await {
                        Ok(response) => survey_number.set(response.json::<SurveyNumber>().await.unwrap().number),
                        Err(err) => eprintln!("Błąd: {}", err)
                    }
                });
            }
        );
    }
    html! {
        <>
            <p>
                {
                    format!("Jest nas już {:#?}!", *survey_number)
                }
            </p>
        </>
    }
    
}
