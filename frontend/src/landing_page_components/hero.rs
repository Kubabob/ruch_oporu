use yew::prelude::*;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
//use gloo::utils::document;
use wasm_bindgen::{prelude::*, JsCast};
use gloo_utils::format::JsValueSerdeExt;

#[function_component(Hero)]
pub fn hero() -> Html {
    let opacity = use_state(|| 1.0);
    let hero_ref = use_node_ref();

    {
        let opacity = opacity.clone();
        let hero_ref = hero_ref.clone();
        use_effect_with((), move |_| {
            let element = hero_ref.cast::<web_sys::Element>().unwrap();
            
            let callback = Closure::<dyn Fn(Vec<IntersectionObserverEntry>, IntersectionObserver)>::new(
                move |entries: Vec<IntersectionObserverEntry>, _observer: IntersectionObserver| {
                    if let Some(entry) = entries.get(0) {
                        let ratio = entry.intersection_ratio();
                        let new_opacity = ratio.clamp(0.1, 1.0); // Never fade below 30%
                        opacity.set(new_opacity as f32);
                    }
                }
            );

            let options = IntersectionObserverInit::new();
            options.set_threshold(&JsValue::from_serde(&[0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]).unwrap());
            
            let observer = IntersectionObserver::new_with_options(
                callback.as_ref().unchecked_ref(),
                &options
            ).unwrap();
            
            observer.observe(&element);
            
            move || {
                observer.disconnect();
                drop(callback);
            }
        });
    }

    html! {
        <section class="hero-section" ref={hero_ref} style={format!("--bg-opacity: {};", *opacity)}>
            <div class="hero-content">
                <h1 class="hero-title">
                    <span class="left-part">{"GRA"}</span>
                    <span class="right-part">{"na ORIENTACJE"}</span>
                </h1>
                
                <div class="stats-container">
                    <div class="stat-item">
                        <div class="stat-number">{"1542"}</div>
                        <div class="stat-label">{"ally"}</div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-number">{"893"}</div>
                        <div class="stat-label">{"lgbtq"}</div>
                    </div>
                </div>
            </div>    
        </section>
    }
}