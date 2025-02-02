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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlikiAnkiety {
    pliki_ankiety_id: i32,
    ankieta_id: i32,
    nazwa_grafiki: Option<String>,
    grafika: Option<Vec<u8>>,
    nazwa_zgody_na_wizerunek: Option<String>,
    zgoda_na_wizerunek: Option<Vec<u8>>,
    czas_dodania: String,
}


#[derive(Serialize, Deserialize)]
pub struct SurveyWithPictures {
    survey: Ankieta,
    pictures: Vec<PlikiAnkiety>,
}

#[function_component(SurveysViewer)]
pub fn surveys_viewer() -> Html {
    /*let surveys = use_state(|| Vec::new());

    {
        let surveys = surveys.clone();
        use_effect_with((), move |_| {
            let surveys = surveys.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Ok(response) = client.get("http://127.0.0.1:8080/ankieta_with_pictures")
                    .send()
                    .await {
                        if let Ok(surveys_response) = response.json::<Vec<SurveyWithPictures>>().await {
                            surveys.set(surveys_response);
                        }
                    }
            });
        })
    
    }

    html! {
        <div>
            <h1>{ "Ankiety" }</h1>
            { for surveys.iter().map(|survey_with_pictures| html! {
                <div class="survey">
                    <p><strong>{ "Historia: " }</strong>{ &survey_with_pictures.survey.historia }</p>
                    <p><strong>{ "Tytuł wpisu: " }</strong>{ survey_with_pictures.survey.tytul_wpisu.clone().unwrap_or_default() }</p>
                    <p><strong>{ "Wybrany cytat: " }</strong>{ survey_with_pictures.survey.wybrany_cytat.clone().unwrap_or_default() }</p>
                    <p><strong>{ "Podpis: " }</strong>{ survey_with_pictures.survey.podpis.clone().unwrap_or_default() }</p>
                    <p><strong>{ "Czas dodania: " }</strong>{ &survey_with_pictures.survey.czas_dodania }</p>
                    { for survey_with_pictures.pictures.iter().map(|picture| html! {
                        <div class="picture">
                            <p><strong>{ "Nazwa grafiki: " }</strong>{ picture.nazwa_grafiki.clone().unwrap_or_default() }</p>
                            { if let Some(grafika) = &picture.grafika {
                                let base64_grafika = base64::encode(grafika);
                                let data_url = format!("data:image/png;base64,{}", base64_grafika);
                                html! { <img src={data_url} alt="Grafika" /> }
                            } else {
                                html! {}
                            }}
                            <p><strong>{ "Nazwa zgody na wizerunek: " }</strong>{ picture.nazwa_zgody_na_wizerunek.clone().unwrap_or_default() }</p>
                            { if let Some(zgoda_na_wizerunek) = &picture.zgoda_na_wizerunek {
                                let base64_zgoda_na_wizerunek = base64::encode(zgoda_na_wizerunek);
                                let data_url = format!("data:image/png;base64,{}", base64_zgoda_na_wizerunek);
                                html! { <img src={data_url} alt="Zgoda na wizerunek" /> }
                            } else {
                                html! {}
                            }}
                        </div>
                    }) }
                </div>
            }) }
        </div>
    }*/
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
        }
    

    

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
