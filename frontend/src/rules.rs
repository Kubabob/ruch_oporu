use yew::prelude::*;
use yew_router::prelude::*;

use crate::navigator::Route;

#[function_component(Rules)]
pub fn rules() -> Html {
    html! {
        <>
            <p>{ "Na razie nic nie ma ale potrzebowałem zrobić stronę." }</p>

            <footer>
                <Link<Route> to={Route::Home}>
                    <button>{ "Powrót do strony głównej" }</button>
                </Link<Route>>
                <p>{ "Wszelkie prawa zastrzeżone © 2024" }</p>
            </footer>
        </>
    }
}