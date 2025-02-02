use yew::prelude::*;

#[function_component(Patrons)]
pub fn patrons() -> Html {
    html! {
        <section class="patrons-section">
            <div class="patrons-content">
                <h2 class="section-title">{"Patroni"}</h2>
                <p class="section-subtitle">{"Jeszcze ich nie mamy, ale możesz być jednym z nich!"}</p>
                
                <div class="logos-grid">
                    // Placeholder items
                    {for (0..4).map(|_| {
                        html! {
                            <div class="logo-placeholder">
                                <div class="placeholder-content">
                                    <div class="animated-border"></div>
                                    <span class="join-text">{"Twoje Logo"}</span>
                                </div>
                            </div>
                        }
                    })}
                </div>

                <div class="cta-container">
                    <a href="#contact" class="cta-button">
                        {"Zostań Patronem"}
                    </a>
                </div>
            </div>
        </section>
    }
}