use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use yew_router::prelude::*;

use crate::navigator::Route;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct User {
    //id: i32,
    imie: String,
    nazwisko: String,
    email: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct CreateUser {
    imie: String,
    nazwisko: String,
    email: String,
}

#[function_component(AddUser)]
pub fn add_user() -> Html {
    let users = use_state(|| Vec::new());

    let imie = use_state(|| "".to_string());
    let nazwisko = use_state(|| "".to_string());
    let email = use_state(|| "".to_string());

    

    // Dodawanie użytkownika
    let add_user = {
        let users = users.clone();
        let imie = imie.clone();
        let nazwisko = nazwisko.clone();
        let email = email.clone();
        Callback::from(move |_| {
            let imie_value = (*imie).clone();
            let nazwisko_value = (*nazwisko).clone();
            let email_value = (*email).clone();
            let users = users.clone();
            spawn_local(async move {
                let client = Client::new();
                let new_user = CreateUser { 
                    imie: imie_value, 
                    nazwisko: nazwisko_value,
                    email: email_value 
                };
                let _ = client.post("http://127.0.0.1:8080/uzytkownicy")
                    .json(&new_user)
                    .send()
                    .await;

                // Odśwież listę po dodaniu użytkownika
                let response = client.get("http://127.0.0.1:8080/uzytkownicy")
                    .send()
                    .await;
                if let Ok(updated_users) = response.unwrap().json::<Vec<User>>().await {
                    users.set(updated_users);
                }
            });
        })
    };

    html! {
        // Formularz dodawania użytkownika
        <>
            <div>
            <h2>{ "Dodaj użytkownika" }</h2>
            
            <input
                placeholder="Imię"
                value={(*imie).clone()}
                oninput={Callback::from(move |e: InputEvent| {
                    imie.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())
                })}
            />

            <input
                placeholder="Nazwisko"
                value={(*nazwisko).clone()}
                oninput={Callback::from(move |e: InputEvent| {
                    nazwisko.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())
                })}
            />

            <input
                placeholder="Email"
                value={(*email).clone()}
                oninput={Callback::from(move |e: InputEvent| {
                    email.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())
                })}
            />
            <button onclick={add_user}>{ "Dodaj użytkownika" }</button>
            
            </div>
        </>
    }
}

#[function_component(ShowUsers)]
pub fn show_users() -> Html {
    let users = use_state(|| Vec::new());
    

    // Pobieranie listy użytkowników
    let fetch_users = {
        let users = users.clone();
        Callback::from(move |_| {
            let users = users.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Ok(response) = client.get("http://127.0.0.1:8080/uzytkownicy")
                    .send()
                    .await
                {
                    if let Ok(users_response) = response.json::<Vec<User>>().await {
                        users.set(users_response);
                    }
                }
            });
        })
    };

    html! {
        <>
            // Wyświetlanie listy użytkowników
            <div>
                <h2>{ "Lista użytkowników" }</h2>
                <button onclick={fetch_users}>{ "Odśwież listę" }</button>
                <ul>
                    { for users.iter().map(|user| html! {
                        <li>{ format!("{} - {} - {}", user.imie, user.nazwisko, user.email) }</li>
                    }) }
                </ul>
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