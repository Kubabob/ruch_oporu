use yew::prelude::*;
use web_sys::MouseEvent;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let dropdown_visible = use_state(|| false);
    
    let toggle_dropdown = {
        let dropdown_visible = dropdown_visible.clone();
        Callback::from(move |_e: MouseEvent| {
            dropdown_visible.set(!*dropdown_visible);
        })
    };

    let toggle_dropdown_closing = toggle_dropdown.clone();

    html! {
        <nav class="navbar">
            <div class="navbar-logo">
                <a href="/">{ "Logo" }</a>
            </div>

            <div class="navbar-items">
                <button class="nav-button">{ "Wystawa" }</button>
                <button class="nav-button">{ "Film" }</button>
                
                <div class="dropdown-container">
                    <button class="nav-button" onclick={toggle_dropdown}>
                        { "Menu ▾" }
                    </button>
                    
                    <div class={classes!("dropdown-content", (*dropdown_visible).then_some("active"))}>
                        // Add the close button here
                        <button class="close-button" onclick={toggle_dropdown_closing}>
                            { "←" } // Left arrow symbol
                        </button>

                        // Menu items
                        <a href="#profile" class="dropdown-item">{ "O NAS" }</a>
                        <a href="#settings" class="dropdown-item">{ "ANONIMOWE COMING OUT'Y" }</a>
                        <a href="#logout" class="dropdown-item">{ "OPOWIEDZ SWOJĄ HISTORIĘ" }</a>
                        <a href="#profile" class="dropdown-item">{ "FAQ" }</a>
                        <a href="#profile" class="dropdown-item">{ "WYSTAWA" }</a>
                        <a href="#profile" class="dropdown-item">{ "FILM" }</a>
                        <a href="#profile" class="dropdown-item">{ "INNE ORGANIZACJE" }</a>
                        <a href="#profile" class="dropdown-item">{ "KONTAKT" }</a>
                        <a href="https://www.instagram.com/your_instagram_page"
                            class="dropdown-item instagram-button"
                            target="_blank"
                            rel="noopener noreferrer">
                                <img src="/img/Instagram_simple_icon.svg.png"
                                    alt="Instagram"
                                    class="instagram-icon" />
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}