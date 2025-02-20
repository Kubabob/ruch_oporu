use yew::prelude::*;
// use super::submissions_count::SubmissionCount;
#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="hero-section">
            <div class="image-container">
                <img src="img/homepage-bg3.jpg" alt="Super cool zdjęcie, które się jeszcze nie załadowało :)" class="faded-border-image" />
            </div>

            <div class="hero-content">
                <h1 class="hero-title">
                    <span class="left-part">{"GRA"}</span>
                    <span class="right-part">{"na ORIENTACJE"}</span>
                </h1>

                // <div class="stats-container">
                //     <div class="stat-item">
                //         <div class="stat-number">
                //             <SubmissionCount user_status="Ally" />
                //         </div>
                //         <div class="stat-label">{"Ally"}</div>
                //     </div>
                //     <div class="stat-item">
                //         <div class="stat-number">
                //             <SubmissionCount user_status="LGBT" />
                //         </div>
                //         <div class="stat-label">{"LGBT"}</div>
                //     </div>
                // </div>
            </div>
        </section>
    }
}
