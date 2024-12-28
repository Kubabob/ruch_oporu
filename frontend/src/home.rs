use yew::prelude::*;

use crate::navigator::NavBar;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <header>
                <h1>{ "Witamy w aplikacji" }</h1>
            </header>
            
            <NavBar />

            <footer>
                <p>{ "Wszelkie prawa zastrzeżone © 2024" }</p>
            </footer>
        </>
    }
}