use yew::prelude::*;
use yew_router::prelude::*;

use frontend::components::navbar::Navbar;
use frontend::components::routes::{Route, switch};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Navbar />
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>    
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
     
}