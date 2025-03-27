use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav>
            <div class="nav" id="wrapper">
                <a class="logo-wrapper" href="/">
                    <img class="logo" src="../graphics/compass.svg" alt="Logo kompasu" />
                    <div class="logo-text">
                        <p id="gra-na-orientacje">
                            {"Gra na Orientację"}
                        </p>
                        <p id="stowarzyszenie">
                            {"Stowarzyszenie"}    
                        </p>
                    </div>
                </a>

                <div class="menu-wrapper">
                    <a href="/o-nas">{"O Nas"}</a>
                    <a href="/">{"Wysłuchaj"}</a>
                    <a href="/">{"Podziel się"}</a>
                    <details id="wiecej-button">
                        <summary id="wiecej">{"Więcej"}</summary>
                        <div id="wiecej-options-wrapper">
                            <div id="wiecej-options">
                                <a href="/">{"Inne organizacje"}</a>
                                <a href="/">{"Wystawa"}</a>
                                <a href="/">{"Film"}</a>
                            </div>
                        </div>
                    </details>

                    <a href="/kontakt">{"Kontakt"}</a>
                </div>
            </div>
        </nav>
    }
}
