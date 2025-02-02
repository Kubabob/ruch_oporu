use yew::prelude::*;

use crate::navigator::NavBar;
use crate::survey_counter::SurveyCounter;


#[function_component(Home)]
pub fn home() -> Html {
    
    html! {
        <>
            <header>
                <h1>{ "Witamy w aplikacji" }</h1>
            </header>
            
            <NavBar />

            <section>
                <SurveyCounter />
            </section>

            <footer>
                <p>{ "Wszelkie prawa zastrzeżone © 2024" }</p>
            </footer>

        </>
    }
}