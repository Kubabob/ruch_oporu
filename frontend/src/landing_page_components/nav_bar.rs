use yew::prelude::*;
use web_sys::MouseEvent;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;


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

    // Close dropdown when clicking outside
    {
        let dropdown_visible = dropdown_visible.clone();
        use_effect_with((), move |_| {
            let dropdown_visible = dropdown_visible.clone();

            // Attach a click event listener to the document
            let document = web_sys::window().unwrap().document().unwrap();
            let document_clone = document.clone();
            let closure = Closure::<dyn Fn(MouseEvent)>::new(move |e: MouseEvent| {
                if let Some(target) = e.target() {
                    let dropdown_element = document_clone
                        .query_selector(".dropdown-container")
                        .unwrap();

                    // Check if the click is outside the dropdown
                    if let Some(dropdown_element) = dropdown_element {
                        if let Some(target_element) = target
                            .dyn_into::<web_sys::Node>()
                            .ok()
                            .and_then(|node| Some(node))
                        {
                            if !dropdown_element.contains(Some(&target_element)) {
                                dropdown_visible.set(false);
                            }
                        }
                    }
                }
            });

            document
                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                .unwrap();
            // Cleanup the event listener
            move || {
                document.remove_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
                closure.forget(); // Prevent memory leaks
            }
        });
    }
    

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
                    // <div class={classes!("dropdown-menu", (*dropdown_visible).then_some("active"))}>
                        // Add the close button here
                        <a class="close-button" onclick={toggle_dropdown_closing}>
                            <img class="left-arrow" src="img/arrow_back.svg" alt="←" /> // Left arrow symbol
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
                                <img src="img/Instagram_simple_icon.svg.png"
                                    alt="Instagram"
                                    class="instagram-icon" />
                        </a>
                    </div>
                </div>

                // <div class="dropdown_menu">
                //     <a class="arrow-back" onclick={toggle_dropdown_closing}>
                //         <svg xmlns="http://www.w3.org/2000/svg" width="59" height="54" viewBox="0 0 59 54" fill="none">
                //             <g filter="url(#filter0_d_41_211)">
                //                 <path d="M19.2366 29.25L33.0033 41.85L29.5002 45L9.8335 27L29.5002 9L33.0033 12.15L19.2366 24.75H49.1668V29.25H19.2366Z" fill="#1D1B20"/>
                //             </g>
                //             <defs>
                //                 <filter id="filter0_d_41_211" x="5.8335" y="9" width="47.3335" height="44" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
                //                 <feFlood flood-opacity="0" result="BackgroundImageFix"/>
                //                 <feColorMatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0" result="hardAlpha"/>
                //                 <feOffset dy="4"/>
                //                 <feGaussianBlur stdDeviation="2"/>
                //                 <feComposite in2="hardAlpha" operator="out"/>
                //                 <feColorMatrix type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.25 0"/>
                //                 <feBlend mode="normal" in2="BackgroundImageFix" result="effect1_dropShadow_41_211"/>
                //                 <feBlend mode="normal" in="SourceGraphic" in2="effect1_dropShadow_41_211" result="shape"/>
                //                 </filter>
                //             </defs>
                //         </svg>
                //     </a>

                //     // Menu items
                //     <a href="#/o-nas" class="o-nas-button">
                //         <p class="onas">{"O NAS"}</p>
                //     </a>
                //     // <a href="#/coming-outs" class="dropdown-item">{ "ANONIMOWE COMING OUT'Y" }</a>
                //     // <a href="#/formularz" class="dropdown-item">{ "OPOWIEDZ SWOJĄ HISTORIĘ" }</a>
                //     // <a href="#/faq" class="dropdown-item">{ "FAQ" }</a>
                //     // <a href="#/wystawa" class="dropdown-item">{ "WYSTAWA" }</a>
                //     // <a href="#/film" class="dropdown-item">{ "FILM" }</a>
                //     // <a href="#/inne-organizacje" class="dropdown-item">{ "INNE ORGANIZACJE" }</a>
                //     // <a href="#/kontakt" class="dropdown-item">{ "KONTAKT" }</a>
                //     // <a href="https://www.instagram.com/your_instagram_page"
                //     //     class="dropdown-item instagram-button"
                //     //     target="_blank"
                //     //     rel="noopener noreferrer">
                //     //         <img src="img/Instagram_simple_icon.svg.png"
                //     //             alt="Instagram"
                //     //             class="instagram-icon" />
                //     // </a>
                // </div>
            </div>
            // </div>
        </nav>
    }
}