use yew::prelude::*;
use yew_router::prelude::*;


use crate::home::Home;
use crate::survey::Survey;
use crate::not_found::NotFound;
use crate::users::ShowUsers;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/survey")]
    Survey,
    #[at("/users")]
    Users,
    #[not_found]
    #[at("/404")]
    NotFound,
}


pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Users => html! { <ShowUsers /> },
        Route::Home => html! { <Home /> },
        Route::Survey => html! { <Survey /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html!(
        <nav class="navbar">
            <ul>
                <li><Link<Route> to={Route::Home}>{"Strona główna"}</Link<Route>></li>
                <li><Link<Route> to={Route::Survey}>{"Ankieta"}</Link<Route>></li>
                <li><Link<Route> to={Route::Users}>{"Użytkownicy"}</Link<Route>></li>
            </ul>
        </nav>
    )
}