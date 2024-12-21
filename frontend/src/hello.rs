use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;
use serde::Deserialize;

// Struktura do parsowania JSON z backendu
#[derive(Deserialize, Clone, PartialEq)]
struct Message {
    message: String,
}

#[function_component(Hello)]
pub fn hello() -> Html {
    let message = use_state(|| None);

    // Funkcja do pobrania danych z backendu
    let fetch_message = {
        let message = message.clone();
        Callback::from(move |_| {
            let message = message.clone();
            spawn_local(async move {
                let client = Client::new();
                let response = client
                    .get("http://127.0.0.1:8080/api/hello") // Adres endpointu backendu
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                    .unwrap()
                    .json::<Message>()
                    .await
                    .unwrap();
                message.set(Some(response));
            });
        })
    };

    html! {
        <div>
            <h1>{ "Frontend zintegrowany z backendem!" }</h1>
            <button onclick={fetch_message}>{ "Pobierz wiadomość z backendu" }</button>
            <div>
                {
                    if let Some(msg) = &*message {
                        html! { <p>{ &msg.message }</p> }
                    } else {
                        html! { <p>{ "Kliknij przycisk, aby pobrać wiadomość." }</p> }
                    }
                }
            </div>
        </div>
    }    
}

