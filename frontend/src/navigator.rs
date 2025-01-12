use yew::prelude::*;
use yew_router::prelude::*;


use crate::home::Home;
use crate::survey::Survey;
use crate::not_found::NotFound;
use crate::users::ShowUsers;
use crate::description::Description;
use crate::rules::Rules;
use crate::rodo::RODO;
use crate::surveys_viewer::SurveysViewer;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    
    #[at("/ankieta")]
    Survey,
    
    #[at("/uzytkownicy")]
    Users,

    #[at("/opis")]
    Description,

    #[at("/zasady")]
    Rules,

    #[at("/rodo")]
    RODO,

    #[at("/ankiety")]
    SurveysViewer,

    #[not_found]
    #[at("/404")]
    NotFound,
}


pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Users => html! { <ShowUsers /> },
        Route::Home => html! { <Home /> },
        Route::Survey => html! { <Survey /> },
        Route::Description => html! { <Description /> },
        Route::Rules => html! { <Rules /> },
        Route::NotFound => html! { <NotFound /> },
        Route::RODO => html! { <RODO /> },
        Route::SurveysViewer => html! { <SurveysViewer /> },
    }
}

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html!(
        <nav class="navbar">
            <ul>
                <li><Link<Route> to={Route::Home}>{"Strona główna"}</Link<Route>></li>
                <li><Link<Route> to={Route::Survey}>{"Wypełnij ankietę"}</Link<Route>></li>
                <li><Link<Route> to={Route::SurveysViewer}>{"Ankiety"}</Link<Route>></li>
                <li><Link<Route> to={Route::Users}>{"Użytkownicy"}</Link<Route>></li>
                <li><Link<Route> to={Route::Description}>{"Opis"}</Link<Route>></li>
                <li><Link<Route> to={Route::Rules}>{"Zasady"}</Link<Route>></li>
                <li><Link<Route> to={Route::RODO}>{"RODO"}</Link<Route>></li>
            </ul>
        </nav>
    )

    /*html!(
        <nav class="menu-container">
            //<!-- burger menu -->
            <input type="checkbox" aria-label="Toggle menu" />
            <span></span>
            <span></span>
            <span></span>

            //<!-- logo -->
            <a href="#" class="menu-logo">
                <img src="https://wweb.dev/resources/navigation-generator/logo-placeholder.png" alt="My Awesome Website"/>
            </a>

            //<!-- menu items -->
            <div class="menu">
                <ul>
                <li>
                    <a href="">
                    {"Ankieta"}
                    </a>
                </li>
                </ul>
                <ul>
                <li>
                    <a href="#signup">
                    {"Sign-up"}
                    </a>
                </li>
                <li>
                    <a href="#login">
                    {"Login"}
                    </a>
                </li>
                </ul>
            </div>
        </nav>
    )*/
}