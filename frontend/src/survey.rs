use yew::prelude::*;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use wasm_bindgen_futures::spawn_local;
use yew_router::prelude::*;
use web_sys::{HtmlInputElement, FileReader};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use chrono::NaiveDateTime;

use crate::navigator::Route;


#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DodajAnkiete {
    pub user_id: i32,
    pub historia: String,
    pub tytul_wpisu: String,
    pub wybrany_cytat: String,
    pub czy_grafika: bool,
    pub czy_osoby_trzecie: bool,
    pub nazwa_grafiki: String,
    pub nazwa_zgody_na_wizerunek: String,
    pub czy_zgoda_na_opublikowanie_grafiki: bool,
    pub czy_anonimowo: bool,
    pub podpis: String,
    pub czy_autentyczny_wpis: bool,
    pub czy_zgoda_na_publikacje_wpisu: bool,
    pub czy_zgoda_na_wykorzystywanie_fragmentow: bool,
    pub czy_zgoda_na_regulamin: bool,
    pub czy_zgoda_na_rodo: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PlikiAnkiety {
    pub pliki_ankiety_id: i32,
    pub ankieta_id: Option<i32>,
    pub nazwa_grafiki: Option<String>,
    pub grafika: Option<Vec<u8>>,
    pub nazwa_zgody_na_wizerunek: Option<String>,
    pub zgoda_na_wizerunek: Option<Vec<u8>>,
    pub czas_dodania: Option<NaiveDateTime>
}


#[function_component(Survey)]
pub fn survey() -> Html {

    let user_id = use_state(|| 0);
    let historia = use_state(|| String::new());
    let tytul_wpisu = use_state(|| String::new());
    let wybrany_cytat = use_state(|| String::new());
    let czy_grafika = use_state(|| false);
    let czy_osoby_trzecie = use_state(|| false);
    let nazwa_grafiki = use_state(|| String::new());
    let grafika = use_state(|| None);
    let nazwa_zgody_na_wizerunek = use_state(|| String::new());
    let zgoda_na_wizerunek = use_state(|| None);
    let czy_zgoda_na_opublikowanie_grafiki = use_state(|| false);
    let czy_anonimowo = use_state(|| false);
    let podpis = use_state(|| String::new());
    let czy_autentyczny_wpis = use_state(|| false);
    let czy_zgoda_na_publikacje_wpisu = use_state(|| false);
    let czy_zgoda_na_wykorzystywanie_fragmentow = use_state(|| false);
    let czy_zgoda_na_regulamin = use_state(|| false);
    let czy_zgoda_na_rodo = use_state(|| false);

    let submitted = use_state(|| false);

    let on_key_down = Callback::from(|e: KeyboardEvent| {
        if e.key() == "Enter" {
            e.prevent_default(); // Zapobiega domyślnej akcji (wysyłaniu formularza)
        }
    });

    let grafika_input_ref = use_node_ref(); // Referencja do elementu input
    
    let on_grafika_change = {
        
        let grafika_input_ref = grafika_input_ref.clone();
        let nazwa_grafiki = nazwa_grafiki.clone();
        let grafika = grafika.clone();
        
        Callback::from(move |_| {
            if let Some(input) = grafika_input_ref.cast::<HtmlInputElement>() {
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    nazwa_grafiki.set(file.name());

                    let reader = FileReader::new().unwrap();
                    let reader_clone = reader.clone();
                    let grafika = grafika.clone();

                    let onloadend = Closure::wrap(Box::new(move || {
                        let result = reader_clone.result().unwrap();
                        let array = js_sys::Uint8Array::new(&result);
                        let data = array.to_vec();
                        grafika.set(Some(data));
                    }) as Box<dyn Fn()>);

                    reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
                    reader.read_as_array_buffer(&file).unwrap();
                    onloadend.forget();
                }
            }
            }
        })
    };
    


    let zgoda_na_wizerunek_ref = use_node_ref();
    
    let on_file_change = {

        let zgoda_na_wizerunek_ref = zgoda_na_wizerunek_ref.clone();
        let nazwa_zgody_na_wizerunek = nazwa_zgody_na_wizerunek.clone();
        let zgoda_na_wizerunek = zgoda_na_wizerunek.clone();

        Callback::from(move |_| {
            if let Some(input) = zgoda_na_wizerunek_ref.cast::<HtmlInputElement>() {
                if let Some(files) = input.files() {
                    if let Some(file) = files.get(0) {
                        nazwa_zgody_na_wizerunek.set(file.name());

                        let reader = FileReader::new().unwrap();
                        let reader_clone = reader.clone();
                        let zgoda_na_wizerunek = zgoda_na_wizerunek.clone();

                        let onloadend = Closure::wrap(Box::new(move || {
                            let result = reader_clone.result().unwrap();
                            let array = js_sys::Uint8Array::new(&result);
                            let data = array.to_vec();
                            zgoda_na_wizerunek.set(Some(data));
                        }) as Box<dyn Fn()>);

                        reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
                        reader.read_as_array_buffer(&file).unwrap();
                        onloadend.forget();
                    }
                }
            }
        })
    };

    

    let submit_survey = {
        let user_id = user_id.clone();
        let historia = historia.clone();
        let tytul_wpisu = tytul_wpisu.clone();
        let wybrany_cytat = wybrany_cytat.clone();
        let czy_grafika = czy_grafika.clone();
        let czy_osoby_trzecie = czy_osoby_trzecie.clone();
        let nazwa_grafiki = nazwa_grafiki.clone();
        let grafika = grafika.clone();
        let nazwa_zgody_na_wizerunek = nazwa_zgody_na_wizerunek.clone();
        let zgoda_na_wizerunek = zgoda_na_wizerunek.clone();
        let czy_zgoda_na_opublikowanie_grafiki = czy_zgoda_na_opublikowanie_grafiki.clone();
        let czy_anonimowo = czy_anonimowo.clone();
        let podpis = podpis.clone();
        let czy_autentyczny_wpis = czy_autentyczny_wpis.clone();
        let czy_zgoda_na_publikacje_wpisu = czy_zgoda_na_publikacje_wpisu.clone();
        let czy_zgoda_na_wykorzystywanie_fragmentow = czy_zgoda_na_wykorzystywanie_fragmentow.clone();
        let czy_zgoda_na_regulamin = czy_zgoda_na_regulamin.clone();
        let czy_zgoda_na_rodo = czy_zgoda_na_rodo.clone();
        let submitted = submitted.clone();



        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let user_id_value = (*user_id).clone();
            let historia_value = (*historia).clone();
            let tytul_wpisu_value = (*tytul_wpisu).clone();
            let wybrany_cytat_value = (*wybrany_cytat).clone();
            let czy_grafika_value = (*czy_grafika).clone();
            let czy_osoby_trzecie_value = (*czy_osoby_trzecie).clone();
            let nazwa_grafiki_value = (*nazwa_grafiki).clone();
            let nazwa_zgody_na_wizerunek_value = (*nazwa_zgody_na_wizerunek).clone();
            let czy_zgoda_na_opublikowanie_grafiki_value = (*czy_zgoda_na_opublikowanie_grafiki).clone();
            let czy_anonimowo_value = (*czy_anonimowo).clone();
            let podpis_value = (*podpis).clone();
            let czy_autentyczny_wpis_value = (*czy_autentyczny_wpis).clone();
            let czy_zgoda_na_publikacje_wpisu_value = (*czy_zgoda_na_publikacje_wpisu).clone();
            let czy_zgoda_na_wykorzystywanie_fragmentow_value = (*czy_zgoda_na_wykorzystywanie_fragmentow).clone();
            let czy_zgoda_na_regulamin_value = (*czy_zgoda_na_regulamin).clone();
            let czy_zgoda_na_rodo_value = (*czy_zgoda_na_rodo).clone();
            let submitted = submitted.clone();

            let user_id_value_multiform = (*user_id).clone();
            let nazwa_grafiki_value_multiform = (*nazwa_grafiki).clone();
            let nazwa_grafiki_value_multiform2 = (*nazwa_grafiki).clone();
            let grafika_value_multiform = (*grafika).clone();
            let nazwa_zgody_na_wizerunek_multiform = (*nazwa_zgody_na_wizerunek).clone();
            let nazwa_zgody_na_wizerunek_multiform2 = (*nazwa_zgody_na_wizerunek).clone();
            let zgoda_na_wizerunek_multiform = (*zgoda_na_wizerunek).clone();


                   
        

            spawn_local(async move {
                let client = Client::new();
                

                let form = DodajAnkiete {
                    user_id: user_id_value,
                    historia: historia_value,
                    tytul_wpisu: tytul_wpisu_value,
                    wybrany_cytat: wybrany_cytat_value,
                    czy_grafika: czy_grafika_value,
                    czy_osoby_trzecie: czy_osoby_trzecie_value,
                    nazwa_grafiki: nazwa_grafiki_value,
                    nazwa_zgody_na_wizerunek: nazwa_zgody_na_wizerunek_value,
                    czy_zgoda_na_opublikowanie_grafiki: czy_zgoda_na_opublikowanie_grafiki_value,
                    czy_anonimowo: czy_anonimowo_value,
                    podpis: podpis_value,
                    czy_autentyczny_wpis: czy_autentyczny_wpis_value,
                    czy_zgoda_na_publikacje_wpisu: czy_zgoda_na_publikacje_wpisu_value,
                    czy_zgoda_na_wykorzystywanie_fragmentow: czy_zgoda_na_wykorzystywanie_fragmentow_value,
                    czy_zgoda_na_regulamin: czy_zgoda_na_regulamin_value,
                    czy_zgoda_na_rodo: czy_zgoda_na_rodo_value,
                };

                let mut multipart_form = Form::new()
                .text("ankieta_id", user_id_value_multiform.to_string());
            
                if let Some(file_data) = grafika_value_multiform {
                    let part = Part::bytes(file_data)
                    .file_name(nazwa_grafiki_value_multiform)
                    .mime_str("application/octet-stream").unwrap();
                
                    multipart_form = multipart_form
                    .text("nazwa_zgody_na_wizerunek", nazwa_grafiki_value_multiform2)
                    .part("zgoda_na_wizerunek", part);
                };

                if let Some(file_data) = zgoda_na_wizerunek_multiform {
                    let part = Part::bytes(file_data)
                    .file_name(nazwa_zgody_na_wizerunek_multiform)
                    .mime_str("application/octet-stream").unwrap();
                
                    multipart_form = multipart_form
                    .text("nazwa_zgody_na_wizerunek", nazwa_zgody_na_wizerunek_multiform2)
                    .part("zgoda_na_wizerunek", part);
                };

                

                if czy_autentyczny_wpis_value {
                    match client.post("http://127.0.0.1:8080/ankieta")
                        .json(&form)
                        .send()
                        .await {
                            Ok(_) => submitted.set(true),
                            Err(err) => eprintln!("Error sending form: {:?}", &err),
                        }

                    match client.post("http://127.0.0.1:8080/ankieta/upload")
                        .multipart(multipart_form)
                        .send()
                        .await {
                            Ok(_) => println!("File uploaded successfully"),
                            Err(err) => eprintln!("Error uploading file: {:?}", &err),
                        }
                    
                    
                } else {
                    submitted.set(true);
                }
            });
            
                
        })
    };

    let czy_grafika_clone = czy_grafika.clone();
    let czy_osoby_trzecie_clone = czy_osoby_trzecie.clone();    
    let czy_anonimowo_clone = czy_anonimowo.clone();
    let czy_autentyczny_wpis_clone = czy_autentyczny_wpis.clone();


    html! {
        <>
            <header>
                <h1>{ "Ankieta" }</h1>
            </header>
            <div class="card">
                if *submitted && *czy_autentyczny_wpis {
                    <p>{ "Dziękujemy za wypełnienie ankiety!" }</p>
                } else if *submitted && !(*czy_autentyczny_wpis) {
                    <p>{ "Dziękujemy za wypełnienie ankiety, jednak nie możemy opublikować wpisu, ponieważ nie jest to autentyczny wpis." }</p>  
                } else {
                    <form onsubmit={submit_survey} onkeydown={on_key_down}>

                        <div>
                            <label>{ "Identyfikator użytkownika: " }</label>
                            <input
                                placeholder="Identyfikator użytkownika"
                                value={(*user_id).clone().to_string()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    user_id.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap())
                                })}
                            />
                        </div>

                        <div>
                            <label>{ "Twoja historia: " }</label>
                            <textarea
                                placeholder="Twoja historia"
                                value={(*historia).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    historia.set(e.target_unchecked_into::<web_sys::HtmlTextAreaElement>().value())
                                })}
                            />
                        </div>

                        <div>
                            <label>{ "Tytuł wpisu: " }</label>
                            <input
                                placeholder="Tytuł wpisu"
                                value={(*tytul_wpisu).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    tytul_wpisu.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())
                                })}
                            />
                        </div>

                        <div>
                            <label>{ "Wybrany cytat: " }</label>
                            <input
                                placeholder="Wybrany cytat"
                                value={(*wybrany_cytat).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    wybrany_cytat.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())
                                })}
                            />
                        </div>
                        
                        
                        <div>
                            <label>{ "Czy chcesz dodać grafikę?: " }</label>
                            <input
                                type="checkbox"
                                checked={(*czy_grafika).clone()}
                                onchange={Callback::from(move |e: Event| {
                                    czy_grafika_clone.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                                })}
                            />
                        </div>

                        if *czy_grafika {

                            <div>
                                <label for="file">{ "Dodaj grafikę: " }</label>
                                <input type="file" ref={grafika_input_ref} onchange={on_grafika_change} />
                            </div>

                            <div>
                                <label>{ "Czy na Twojej grafice znajdują się osoby trzecie?: " }</label>
                                <input
                                    type="checkbox"
                                    checked={*czy_osoby_trzecie}
                                    onchange={Callback::from(move |e: Event| {
                                        czy_osoby_trzecie_clone.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                                    })}
                                />
                            </div>

                            if *czy_osoby_trzecie {

                                <div>
                                    <label for="file">{ "Dodaj zgodę na udostępnienie wizerunku: " }</label>
                                    <input type="file" ref={zgoda_na_wizerunek_ref} onchange={on_file_change} />
                                </div>
                            }
                            
                            <div>
                                <label>{ "Czy chcesz zezwolić na opublikowanie grafiki?: " }</label>
                                <input
                                    type="checkbox"
                                    checked={*czy_zgoda_na_opublikowanie_grafiki}
                                    onchange={Callback::from(move |e: Event| {
                                        czy_zgoda_na_opublikowanie_grafiki.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                                    })}
                                />
                            </div>
                        }
                        
                        
                        
                        <div>
                            <label>{ "Czy chcesz by twoja odpowied została anonimizowana?: " }</label>
                            <input
                                type="checkbox"
                                checked={*czy_anonimowo}
                                onchange={Callback::from(move |e: Event| {
                                    czy_anonimowo_clone.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                                })}
                            />
                        </div>

                        if !*czy_anonimowo {
                            <div>
                                <label>{ "Podpis: " }</label>
                                <input
                                    placeholder="Podpis"
                                    value={(*podpis).clone()}
                                    oninput={Callback::from(move |e: InputEvent| {
                                        podpis.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())
                                    })}
                                />
                            </div>
                        }
                        
                        
                        <div>
                            <label>{ "Czy to jest autentyczny wpis? : " }</label>
                            <input
                                type="checkbox"
                                checked={*czy_autentyczny_wpis}
                                onchange={Callback::from(move |e: Event| {
                                    czy_autentyczny_wpis_clone.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                                })}
                            />
                        </div>

                        <div>
                            <label>{ "Czy zgadzasz się na wykorzystanie fragmentów?: " }</label>
                            <input
                                type="checkbox"
                                checked={*czy_zgoda_na_wykorzystywanie_fragmentow}
                                onchange={Callback::from(move |e: Event| {
                                    czy_zgoda_na_wykorzystywanie_fragmentow.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                                })}
                            />
                        </div>

                        <div>
                            <label>
                            {
                                "Czy zgadzasz się na " 
                            }
                            <a href="http://localhost:3001/rules" target="_blank" rel="noopener noreferrer">{ "regulamin" }</a>
                            {
                                "?: "
                            }
                            </label>
                            <input
                                type="checkbox"
                                checked={*czy_zgoda_na_regulamin}
                                onchange={Callback::from(move |e: Event| {
                                    czy_zgoda_na_regulamin.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                                })}
                                required=true
                            />
                        </div>
                        
                        <div>
                        <label>
                        {
                            "Czy zgadzasz się na " 
                        }
                        <a href="http://localhost:3001/rodo" target="_blank" rel="noopener noreferrer">{ "RODO" }</a>
                        {
                            "?: "
                        }
                        </label>
                            <input
                                type="checkbox"
                                checked={*czy_zgoda_na_rodo}
                                onchange={Callback::from(move |e: Event| {
                                    czy_zgoda_na_rodo.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                                })}
                                required=true
                            />
                        </div>

                        <button type="submit">{ "Wyślij" }</button>
                    </form>
                }
            </div>
            <footer>
                <Link<Route> to={Route::Home}>
                    <button>{ "Powrót do strony głównej" }</button>
                </Link<Route>>
                <p>{ "Wszelkie prawa zastrzeżone © 2024" }</p>
            </footer>
        </>
}

/*html! {
    <>
        if *submitted && *czy_autentyczny_wpis {
            <p>{ "Dziękujemy za wypełnienie ankiety!" }</p>
        } else if *submitted && !(*czy_autentyczny_wpis) {
            <p>{ "Dziękujemy za wypełnienie ankiety, jednak nie możemy opublikować wpisu, ponieważ nie jest to autentyczny wpis." }</p>  
        } else {
            <form class="form-container" onsubmit={submit_survey} onkeydown={on_key_down}>
                <div class="form-group">
                    <div class="form-label">{"Twoja historia"}</div>
                    <div class="form-input">
                        <textarea
                            class="textarea"
                            value={(*history).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                history.set(e.target_unchecked_into::<web_sys::HtmlTextAreaElement>().value())
                            })}
                        />
                    </div>
                </div>
                <div class="form-group">
                    <div class="form-label">{"Tytuł wpisu"}</div>
                    <div class="form-input">
                        <input
                            class="input"
                            value={(*tytul_wpisy).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                tytul_wpisy.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())
                            })}
                        />
                    </div>
                </div>
                <div class="form-group">
                    <div class="form-label">{"Wybrany cytat"}</div>
                    <div class="form-input">
                        <input
                            class="input"
                            value={(*wybrany_cytat).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                wybrany_cytat.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())
                            })}
                        />
                    </div>
                </div>
                <div class="form-group">
                    <div class="form-checkbox">
                        <input
                            type="checkbox"
                            class="checkbox"
                            checked={(*czy_grafika).clone()}
                            onchange={Callback::from(move |e: Event| {
                                czy_grafika.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                            })}
                        />
                        <div class="checkbox-label">{"Czy chcesz dodać grafikę?"}</div>
                    </div>
                </div>
                if *czy_grafika_clone {
                    <div class="form-group">
                        <div class="form-checkbox">
                            <input
                                type="checkbox"
                                class="checkbox"
                                checked={(*czy_osoby_trzecie).clone()}
                                onchange={Callback::from(move |e: Event| {
                                    czy_osoby_trzecie.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                                })}
                            />
                            <div class="checkbox-label">{"Czy na grafice znajdują się osoby trzecie?"}</div>
                        </div>
                    </div>
                    if *czy_osoby_trzecie_clone {
                        <div class="dodaj-grafike">
                            <div class="file-upload-label">{"Dodaj zgodę"}</div>
                            <input type="file" ref={file_input_ref} onchange={on_file_change} />
                        </div>
                    }
                    <div class="form-group">
                        <div class="form-checkbox">
                            <input
                                type="checkbox"
                                class="checkbox"
                                checked={(*czy_zgoda_na_opublikowanie_grafiki).clone()}
                                onchange={Callback::from(move |e: Event| {
                                    czy_zgoda_na_opublikowanie_grafiki.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                                })}
                            />
                            <div class="checkbox-label">{"Czy chcesz zezwolić na opublikowanie grafiki?"}</div>
                        </div>
                    </div>
                }
                <div class="form-group">
                    <div class="form-checkbox">
                        <input
                            type="checkbox"
                            class="checkbox"
                            checked={(*czy_anonimowo).clone()}
                            onchange={Callback::from(move |e: Event| {
                                czy_anonimowo.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                            })}
                        />
                        <div class="checkbox-label">{"Czy chcesz aby ankieta była anonimowa?"}</div>
                    </div>
                </div>
                if !*czy_anonimowo_clone {
                    <div class="form-group">
                        <div class="form-label">{"Podpis"}</div>
                        <div class="form-input">
                            <input
                                class="input"
                                value={(*podpis).clone()}
                                oninput={Callback::from(move |e: InputEvent| {
                                    podpis.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())
                                })}
                            />
                        </div>
                    </div>
                }
                <div class="form-group">
                    <div class="form-checkbox">
                        <input
                            type="checkbox"
                            class="checkbox"
                            checked={(*czy_autentyczny_wpis).clone()}
                            onchange={Callback::from(move |e: Event| {
                                czy_autentyczny_wpis.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                            })}
                        />
                        <div class="checkbox-label">{"Czy wpis jest autentyczny?"}</div>
                    </div>
                </div>
                <div class="form-group">
                    <div class="form-checkbox">
                        <input
                            type="checkbox"
                            class="checkbox"
                            checked={(*czy_zgoda_na_publikacje_wpisu).clone()}
                            onchange={Callback::from(move |e: Event| {
                                czy_zgoda_na_publikacje_wpisu.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                            })}
                        />
                        <div class="checkbox-label">{"Czy chcesz aby Twój wpis był opublikowany na stronie?"}</div>
                    </div>
                </div>
                <div class="form-group">
                    <div class="form-checkbox">
                        <input
                            type="checkbox"
                            class="checkbox"
                            checked={(*czy_zgoda_na_regulamin).clone()}
                            onchange={Callback::from(move |e: Event| {
                                czy_zgoda_na_regulamin.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                            })}
                        />
                        <div class="checkbox-label">
                            <span>{"Czy zgadzasz się z "}</span>
                            <span class="link">{"regulaminem"}</span>
                            <span>{"?"}</span>
                        </div>
                    </div>
                </div>
                <div class="form-group">
                    <div class="form-checkbox">
                        <input
                            type="checkbox"
                            class="checkbox"
                            checked={(*czy_zgoda_na_rodo).clone()}
                            onchange={Callback::from(move |e: Event| {
                                czy_zgoda_na_rodo.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().checked())
                            })}
                        />
                        <div class="checkbox-label">
                            <span>{"Czy zgadzasz się z "}</span>
                            <span class="link">{"RODO"}</span>
                            <span>{"?"}</span>
                        </div>
                    </div>
                </div>
                <div class="form-submit">
                    <button type="submit" class="submit-button">
                        <div class="submit-button-text">{"Wyślij ankietę"}</div>
                    </button>
                </div>
            </form>
        }
        <footer>
            <Link<Route> to={Route::Home}>
                <button>{ "Powrót do strony głównej" }</button>
            </Link<Route>>
            <p>{ "Wszelkie prawa zastrzeżone © 2024" }</p>
        </footer>
    </>
}*/
}
