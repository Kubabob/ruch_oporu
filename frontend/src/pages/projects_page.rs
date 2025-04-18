use yew::prelude::*;

use crate::components::navbar::Navbar;

#[function_component(ProjectsPage)]
pub fn projects_page() -> Html {
    html!(
        <section class="subPage">
            <Navbar color="black" />

            <h2 class="black" style="padding-left: 13.4%;">{"Nasze projekty"}</h2>
            <section class="colorfulButtons boxesWrapper">
                <a href="wystawa" id="wystawaBox" class="boxWrapper colorfulBoxWrapper">
                    <div class="boxHeadingWrapper">
                        <span class="boxHeader black">{"Wystawa"}</span>
                    </div>
                    <div class="textWrapper horizontal">
                        <p class="black">{"Zobacz więcej"}</p>
                        <img src="../graphics/arrow-right-black.svg" />
                    </div>
                </a>
                <a href="film" id="filmBox" class="boxWrapper colorfulBoxWrapper">
                    <div class="boxHeadingWrapper">
                        <span class="boxHeader black">{"Film"}</span>
                    </div>
                    <div class="textWrapper horizontal">
                        <p class="black">{"Zobacz więcej"}</p>
                        <img src="../graphics/arrow-right-black.svg" />
                    </div>
                </a>

                <a href="faq" id="faqBox" class="boxWrapper colorfulBoxWrapper">
                    <div class="boxHeadingWrapper">
                        <span class="boxHeader black">{"Pytania i odpowiedzi"}</span>
                    </div>
                    <div class="textWrapper horizontal">
                        <p class="black">{"Zobacz więcej"}</p>
                        <img src="../graphics/arrow-right-black.svg" />
                    </div>
                </a>
          </section>
        </section>
    )
}