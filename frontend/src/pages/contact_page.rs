use yew::prelude::*;

use crate::components::navbar::Navbar;

#[function_component(ContactPage)]
pub fn contact_page() -> Html {
    html!(
        <section class="subPage">
                <Navbar color="black" />
                <section class="contactUs textBlocksWrapper">
                    <div class="textBlockWrapper">
                    <h2 class="black">
                    {"Zainteresowały Cię nasze działania? A może masz pytania?"}
                    <br/>
                    {"Skontaktuj się z nami poprzez adres mailowy:"}
                    </h2>
                    <br/>
                    <a href="mailto:granaorientacje@gmail.com" class="footerMailWrapper" style="border: 2px solid #000;">
                        <h4 class="footerMail black">{"granaorientacje@gmail.com"}</h4>
                    </a>
                    <br/>
                    <h2 class="black">
                    {"Chcesz z nami porozmawiać?"}
                    </h2>
                    <p class="black">
                    {"Napisz na nasz adres mailowy, opisz o czym chcesz porozmawiać i umów się na rozmowę online"}
                    </p>

                    <br/>

                    <h2 class="black">
                    {"Chcesz się zaangażować?"}
                    </h2>
                    <p class="black">
                    {"Chcesz się zaangażować w działalność Stowarzyszenia lub prowadzenie i rozwój strony? Napisz do nas!"}
                    </p>
                    </div>
                </section>
        </section>
    )
}