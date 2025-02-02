use yew::prelude::*;
use yew_router::prelude::*;
use console_log::init_with_level;

use frontend::navigator::{Route, switch};
use frontend::landing_page::LandingPage;


#[function_component(App)]
fn app() -> Html {

    html! {
        <>
            /*<BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
            */

            <LandingPage />
            
        </>
    }
}

fn main() {
    console_log::init_with_level(log::Level::Info).unwrap();
    yew::Renderer::<App>::new().render();
}
