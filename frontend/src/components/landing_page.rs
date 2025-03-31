use yew::prelude::*;
use crate::components::floating_box::FloatingBox;
use crate::components::form_box::ContactBox;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="landing-page">
        // <>
            <section class="hero">
                <div class="hero-text-wrapper">
                    <div class="gra-na-orientacje-wrapper">
                        <p id="gra-na-orientacje">{"GRA NA ORIENTACJĘ"}</p>
                    </div>
                    <p id="twoja-bezpieczna-przestrzen">{"Twoja bezpieczna przestrzeń"}</p>
                </div>
            </section>

            <section class="page-description">
                <div class="text-wrapper" id="description-text-wrapper">
                    <h3 class="description-header">{"Kim jesteśmy i do czego dążymy?"}</h3>
                    <div class="description-wrapper">
                        <p>
                            {"Stowarzyszenie „Gra na Orientację” to wspólnota przyjaciół, założona w odpowiedzi na doświadczenie dyskryminacji ze względu na orientację seksualną w Związku Harcerstwa Rzeczypospolitej."}
                        </p>
                        <p>
                            {"Stowarzyszenie tworzą byłe i obecne osoby członkowskie i osoby instruktorskie ZHR, którym zależy na budowaniu bardziej włączającej i bezpiecznej wspólnoty harcerskiej."}
                        </p>
                        <p>
                            {"Działamy w myśl wartości, w których w tej organizacji zostałyśmy wychowane w odpowiedzialności za społeczeństwo, braterstwa i wzajemnego szacunku."}
                        </p>
                    </div>
                </div>
                <img class="rece-w-kole" src="../graphics/rece-w-kole.jpg" />
            </section>

            <section class="floating-boxes">
                <div class="floating-boxes-wrapper">
                    <FloatingBox img_src="../graphics/floating-box-1.jpg" heading="(Nie)Anonimowe Historie" orange_text="Strefa 'Wysłuchaj'" black_text="" green_text="Przeczytaj" href="wysluchaj"/>
                    <FloatingBox img_src="../graphics/floating-box-2.jpg" heading="Opowiedz swoją historię" orange_text="Strefa 'Podziel się'" black_text="" green_text="Wypełnij anonimowy formularz" href="podziel-sie"/>
                    <FloatingBox img_src="../graphics/floating-box-3.jpg" heading="Pytania i odpowiedzi" orange_text="Strefa 'FAQ'" black_text="" green_text="Zobacz więcej" href="faq"/>
                </div>
            </section>

            <section class="contact">
                <ContactBox />
            </section>
        </div>
        // </>
    }
}