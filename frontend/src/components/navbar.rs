use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav>
            <div class="nav" id="wrapper">
                <a class="logo-wrapper" href="/">
                    <img class="logo" src="../graphics/compass.svg" alt="Logo kompasu" />
                    <div class="logo-text">
                        <p class="gra-na-orientacje">
                            {"Gra na Orientację"}
                        </p>
                        <p class="stowarzyszenie">
                            {"Stowarzyszenie"}    
                        </p>
                    </div>
                </a>

                <div class="menu-wrapper">
                    <a href="/o-nas">{"O Nas"}</a>
                    <a href="/wysluchaj">{"Wysłuchaj"}</a>
                    <a href="/podziel-sie">{"Podziel się"}</a>
                    <details id="wiecej-button">
                        <summary id="wiecej">{"Więcej"}</summary>
                        <div id="wiecej-options-wrapper">
                            <div id="wiecej-options">
                                <a href="/inne-organizacje">{"Inne organizacje"}</a>
                                <a href="/wystawa">{"Wystawa"}</a>
                                <a href="/film">{"Film"}</a>
                            </div>
                        </div>
                    </details>

                    <a href="/kontakt">{"Kontakt"}</a>
                </div>
            </div>
        </nav>
    }
}
