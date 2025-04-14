use yew::prelude::*;
use yew_router::prelude::*;

use frontend::pages::routes::{Route, switch};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            // <Navbar />
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>    
            // <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
     
}