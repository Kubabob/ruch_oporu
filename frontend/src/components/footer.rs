use yew::prelude::*;


#[function_component(Footer)]
pub fn footer() -> Html {
    html!(
        <footer>
            <div class="footerContentWrapper">
                <div class="footerLogoMailWrapper">
                    <div class="footerLogoWrapper">
                        <a href="">
                            <img class="logo" src="../graphics/gno_logotyp_poziom_black.png" />
                        </a>
                    </div>

                    <div class="footerMailWrapper">
                        <a href="mailto:granaorientacje@gmail.com">
                            <h4 class="footerMail black">{"granaorientacje@gmail.com"}</h4>
                        </a>
                    </div>
                </div>

                <div class="footerButtonColumnsWrapper">
                    <div class="footerButtonsWrapper">
                        <div class="footerButtonWrapper">
                            <a href="/o-nas" class="button black">{"O Nas"}</a>
                            <a href="/nasze-historie" class="button black">{"Nasze historie"}</a>
                            <a href="/faq" class="button black">{"FAQ"}</a>
                        </div>
                        <div class="footerButtonWrapper">
                            <a href="/wystawa" class="button black">{"Wystawa"}</a>
                            <a href="/film" class="button black">{"Film"}</a>
                            <a href="/kontakt" class="button black">{"Kontakt"}</a>
                        </div>
                        <div class="footerButtonWrapper">
                            <span class="button black">{"Obserwuj nas"}</span>
                            <div class="footerButtonWrapper horizontal">
                                <a href="/" class="button black">
                                    <img class="footerIcon" src="../graphics/Logo-Instagram.svg" />
                                </a>
                                <a href="/" class="button black">
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