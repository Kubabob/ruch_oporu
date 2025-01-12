use yew::prelude::*;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use yew_router::prelude::*;
use crate::navigator::Route;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ankieta {
    historia: String,
    tytul_wpisu: Option<String>,
    wybrany_cytat: Option<String>,
    podpis: Option<String>,
    czas_dodania: String,
}

#[function_component(SurveysViewer)]
pub fn surveys_viewer() -> Html {
    let surveys = use_state(|| Vec::new());

    //let _fetch_surveys = {
    {
        let surveys = surveys.clone();
        use_effect_with((), move |_| {
            let surveys = surveys.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Ok(response) = client.get("http://127.0.0.1:8080/ankieta")
                    .send()
                    .await {
                        if let Ok(surveys_response) = response.json::<Vec<Ankieta>>().await {
                            surveys.set(surveys_response);
                        }
                    }
            });
        })
    };

    html! {
        <>
            <div>
                <h1>{ "Ankiety" }</h1>
                { for surveys.iter().map(|survey| html! {
                    <div class="survey">
                        <p><strong>{ "Historia: " }</strong>{ &survey.historia }</p>
                        <p><strong>{ "Tytuł wpisu: " }</strong>{ survey.tytul_wpisu.clone().unwrap_or_default() }</p>
                        <p><strong>{ "Wybrany cytat: " }</strong>{ survey.wybrany_cytat.clone().unwrap_or_default() }</p>
                        <p><strong>{ "Podpis: " }</strong>{ survey.podpis.clone().unwrap_or_default() }</p>
                        <p><strong>{ "Czas dodania: " }</strong>{ &survey.czas_dodania }</p>
                    </div>
                }) }
            </div>
            <footer>
                <Link<Route> to={Route::Home}>
                    <button>{ "Powrót do strony głównej" }</button>
                </Link<Route>>
                <p>{ "Wszelkie prawa zastrzeżone © 2024" }</p>
            </footer>
        </>
    }
}