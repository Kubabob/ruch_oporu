use yew::prelude::*;
use yew_router::prelude::*;
use frontend::navigation_bar_components::routes::{Route, switch};
use frontend::landing_page_components::nav_bar::NavBar;



#[function_component(App)]
fn app() -> Html {

    html! {
        <>
            <NavBar />
            <HashRouter>
                <Switch<Route> render={switch} />
            </HashRouter>        
        </>
    }
}

fn main() {
    console_log::init_with_level(log::Level::Info).unwrap();
    yew::Renderer::<App>::new().render();
}
