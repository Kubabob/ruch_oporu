use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use reqwest;
//use reqwest::Error;
use serde::Deserialize;
use serde_json::from_str;

#[derive(Properties, PartialEq, Debug)]
pub struct SubmissionsCountProps {
    pub user_status: String,
}

#[derive(Deserialize)]
struct SubmissionCountResponse {
    count: u64,
}

#[function_component(SubmissionCount)]
pub fn submission_count(props: &SubmissionsCountProps) -> Html {
    let user_status = props.user_status.clone();
    let count = use_state(|| None);
    let error = use_state(|| None);

    {
        let user_status = user_status.clone();
        let count = count.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            let count = count.clone();
            let error = error.clone();
            spawn_local(async move {
                let client = reqwest::Client::new();
                let url = format!("http://127.0.0.1:8000/stats/status/{}/count", user_status);
                match client
                    .get(&url)
                    .header(reqwest::header::CONTENT_TYPE, "application/json")
                    .send()
                    .await{
                    Ok(response) => {
                        let status_code = response.status();
                        let text = response.text().await.unwrap_or_default();

                        log::info!("Response: {}", text);

                        if status_code.is_success() {
                            match from_str::<SubmissionCountResponse>(&text) {
                                Ok(data) => count.set(Some(data.count)),
                                Err(e) => error.set(Some(format!("Failed to parse response: {}", e))),
                            }
                        } else {
                            error.set(Some(format!(
                                "HTTP {} | Response: {}",
                                status_code, text
                            )));
                        }
                    }
                    Err(e) => error.set(Some(format!("Request error: {}", e))),
                }
            });
            || ()
        });
    }


    html! {

        // <p>{*count}</p>
        // <div class="submission-count">
            if let Some(count) = *count {
                <p>{ count }</p>
            } else if let Some(error) = &*error {
                <p class="error">{error}</p>
            } else {
                <p>{"Loading..."}</p>
            }
        // </div>
    }
}