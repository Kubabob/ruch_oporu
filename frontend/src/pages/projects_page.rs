use yew::prelude::*;

use crate::components::navbar::Navbar;

#[function_component(ProjectsPage)]
pub fn projects_page() -> Html {
    html!(
        <section class="subPage">
            <Navbar color="black" />

            <h2 class="black" style="padding-left: 13.4%;">{"Nasze projekty"}</h2>
            <section class="colorfulButtons boxesWrapper">
                <div class="boxWrapper colorfulBoxWrapper" style="background-color: #E57E38;">
                <div class="boxHeadingWrapper">
                    <span class="boxHeader black">{"Wystawa"}</span>
                </div>
                <div class="textWrapper horizontal">
                    <a href="wystawa">
                    <p class="black">{"Zobacz więcej"}</p>
                    </a>
                    <img src="../graphics/arrow-right-black.svg" />
                </div>
                </div>
                <div class="boxWrapper colorfulBoxWrapper" style="background-color: #AE94C3;">
                <div class="boxHeadingWrapper">
                    <span class="boxHeader black">{"Film"}</span>
                </div>
                <div class="textWrapper horizontal">
                    <a href="film">
                    <p class="black">{"Zobacz więcej"}</p>
                    </a>
                    <img src="../graphics/arrow-right-black.svg" />
                </div>
                </div>
          </section>
        </section>
    )
}