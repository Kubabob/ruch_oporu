use yew::prelude::*;
use yew_router::prelude::*;

use crate::navigator::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <>
            <header>
                <h1>{ "404: Strona nie znaleziona" }</h1>
            </header>
            <Link<Route> to={Route::Home}>
                <button>{ "Powrót do strony głównej" }</button>
            </Link<Route>>
        </>
    }
}