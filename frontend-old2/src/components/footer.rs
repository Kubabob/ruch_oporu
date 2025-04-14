use yew::prelude::*;


#[function_component(Footer)]
pub fn footer() -> Html {
    html!(
        <footer>
            <div class="footer-content-wrapper">
                <div class="footer-info">
                    <div class="footer-upper-wrapper">
                        <div class="footer-header-wrapper">
                            <img class="logo" id="footer" src="../graphics/compass-white.svg" alt="Logo kompasu" />
                            <div class="logo-text" id="footer">
                                <p class="gra-na-orientacje" id="footer">
                                    {"Gra na Orientację"}
                                </p>
                                <p class="stowarzyszenie" id="footer">
                                    {"Stowarzyszenie"}    
                                </p>
                            </div>
                        </div>
                    </div>

                    <div class="footer-lower-wrapper">
                        <a href="mailto:granaorientacje@gmail.com">{"granaorientacje@gmail.com"}</a>
                    </div>
                </div>

                <div class="footer-links">
                    <div class="footer-col">
                        <div class="footer-link">
                            <a href="o-nas">{"O Nas"}</a>
                        </div>
                        <div class="footer-link">
                            <a href="posluchaj">{"Posłuchaj"}</a>
                        </div>
                        <div class="footer-link">
                            <a href="podziel-sie">{"Podziel się"}</a>
                        </div>
                        <div class="footer-link">
                            <a href="faq">{"FAQ"}</a>
                        </div>
                    </div>

                    <div class="footer-col">
                        <div class="footer-link">
                            <a href="wystawa">{"Wystawa"}</a>
                        </div>
                        <div class="footer-link">
                            <a href="film">{"Film"}</a>
                        </div>
                        <div class="footer-link">
                            <a href="inne-organizacje">{"Inne organizacje"}</a>
                        </div>
                        <div class="footer-link">
                            <a href="kontakt">{"Kontakt"}</a>
                        </div>
                    </div>

                    <div class="footer-col">
                        <div class="footer-link">
                            <p>{"Obserwuj nas"}</p>
                        </div>
                        
                        <div class="footer-buttons">
                            <a href="instagram.com">
                                <img src="../graphics/Logo-Instagram.svg" alt="Instagram" />
                            </a>
                        
                            <a href="youtube.com">
                                <img src="../graphics/Logo-YouTube.svg" alt="Youtube" />
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </footer>
    )
}