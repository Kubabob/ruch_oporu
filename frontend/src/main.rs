use yew::prelude::*;
use yew_router::prelude::*;

use frontend::components::navbar::Navbar;
use frontend::components::routes::{Route, switch};
use frontend::components::footer::Footer;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Navbar />
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>    
            <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
     
}