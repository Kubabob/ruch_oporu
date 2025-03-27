use yew::prelude::*;
use frontend::components::navbar::Navbar;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Navbar />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}