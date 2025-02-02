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

#[function_component(Ankieta)]
pub fn ankieta() -> Html {
    html! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <meta name="description" content="Share your thoughts and experiences with the LGBTQ+ community survey." />
                <title>{"LGBTQ+ Community Survey"}</title>
                <link rel="stylesheet" href="survey-styles.css" />
                <link data-trunk="true" rel="copy-dir" href="./img" />
            </head>
            <body>
                /*<header class="hero">
                    <div class="hero-content">
                        <h1>{"We Value Your Voice"}</h1>
                        <p>{"Your feedback helps us create a more inclusive and supportive community. Take a few moments to complete our survey."}</p>
                    </div>
                </header>*/
                
                <section class="survey-container">
                    <form action="#" method="post" enctype="multipart/form-data">
                        <div class="form-group">
                            <label for="name">{"Your Name:"}</label>
                            <input type="text" id="name" name="name" placeholder="Enter your name" required=true />
                        </div>

                        <div class="form-group">
                            <label for="email">{"Your Email:"}</label>
                            <input type="email" id="email" name="email" placeholder="Enter your email" required=true />
                        </div>

                        <div class="form-group">
                            <label>{"How do you identify?"}</label>
                            <div class="radio-group">
                                <label><input type="radio" name="identity" value="LGBTQ+" required=true />{" LGBTQ+"}</label>
                                <label><input type="radio" name="identity" value="Ally" />{" Ally"}</label>
                                <label><input type="radio" name="identity" value="Other" />{" Other"}</label>
                            </div>
                        </div>

                        <div class="form-group">
                            <label for="comments">{"Share Your Experience:"}</label>
                            <textarea id="comments" name="comments" rows="5" placeholder="Tell us about your experience..."></textarea>
                        </div>

                        <div class="form-group">
                            <label for="file-upload">{"Upload a Photo or Document (Optional):"}</label>
                            <input type="file" id="file-upload" name="file-upload" accept="image/*,application/pdf" />
                        </div>

                        <button type="submit" class="btn primary">{"Submit Survey"}</button>
                    </form>
                </section>
            </body>
        </html>
    }
}
