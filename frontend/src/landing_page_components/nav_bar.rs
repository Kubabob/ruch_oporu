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
                <a href="/ruch_oporu/">{ "Logo" }</a>
            </div>

            <div class="navbar-items">
                <a href="#/wystawa" class="nav-button">{ "Wystawa" }</a>
                <a href="#/film" class="nav-button">{ "Film" }</a>
                /*
                <Link<Route> to={Route::Home} classes={get_link_class(Route::Home)}>
                    { "Home" }
                </Link<Route>>*/
                <div class="dropdown-container">
                    <a class="nav-button" onclick={toggle_dropdown}>
                        { "Menu ▾" }
                    </a>
                    
                    <div class={classes!("dropdown-content", (*dropdown_visible).then_some("active"))}>
                        // Add the close button here
                        <a class="close-button" onclick={toggle_dropdown_closing}>
                            { "←" } // Left arrow symbol
                        </a>

                        // Menu items
                        <a href="#/o-nas" class="dropdown-item">{ "O NAS" }</a>
                        <a href="#/coming-outs" class="dropdown-item">{ "ANONIMOWE COMING OUT'Y" }</a>
                        <a href="#/formularz" class="dropdown-item">{ "OPOWIEDZ SWOJĄ HISTORIĘ" }</a>
                        <a href="#/faq" class="dropdown-item">{ "FAQ" }</a>
                        <a href="#/wystawa" class="dropdown-item">{ "WYSTAWA" }</a>
                        <a href="#/film" class="dropdown-item">{ "FILM" }</a>
                        <a href="#/inne-organizacje" class="dropdown-item">{ "INNE ORGANIZACJE" }</a>
                        <a href="#/kontakt" class="dropdown-item">{ "KONTAKT" }</a>
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