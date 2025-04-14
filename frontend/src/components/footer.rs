use yew::prelude::*;


#[function_component(Footer)]
pub fn footer() -> Html {
    html!(
        <footer>
            <div class="footerContentWrapper">
                <div class="footerLogoMailWrapper">
                    <div class="footerLogoWrapper">
                        <img class="logo" src="../graphics/gno_logotyp_poziom_white.png" />
                    </div>

                    <div class="footerMailWrapper">
                        <h4 class="footerMail white">{"granaorientacje@gmail.com"}</h4>
                    </div>
                </div>

                <div class="footerButtonColumnsWrapper">
                    <div class="footerButtonsWrapper">
                        <div class="footerButtonWrapper">
                            <a href="/o-nas" class="button white">{"O Nas"}</a>
                            <a href="/" class="button white">{"Nasze historie"}</a>
                            <a href="/faq" class="button white">{"FAQ"}</a>
                        </div>
                        <div class="footerButtonWrapper">
                            <a href="/" class="button white">{"Wystawa"}</a>
                            <a href="/" class="button white">{"Film"}</a>
                            <a href="/" class="button white">{"Kontakt"}</a>
                        </div>
                        <div class="footerButtonWrapper">
                            <span class="button white">{"Obserwuj nas"}</span>
                            <div class="footerButtonWrapper horizontal">
                                <a href="/" class="button white">
                                    <img class="footerIcon" src="../graphics/Logo-Instagram.svg" />
                                </a>
                                <a href="/" class="button white">
                                    <img class="footerIcon" src="../graphics/Logo-Facebook.svg" />
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </footer>
    )
}