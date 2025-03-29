use yew::prelude::*;
use yew_router::prelude::*;

use frontend::components::navbar::Navbar;
use frontend::components::routes::{Route, switch};
use frontend::components::footer::Footer;

fn base_path() -> String {
    option_env!("PUBLIC_URL").unwrap_or("/").to_string()
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Navbar />
            <BrowserRouter basename={base_path()}>
                <Switch<Route> render={switch} />
            </BrowserRouter>    
            <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
     
}