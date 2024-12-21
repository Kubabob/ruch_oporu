use yew::prelude::*;

mod survey;
mod hello;
mod users;

use survey::Survey;
use hello::Hello;
use users::{AddUser, ShowUsers};


#[function_component(App)]
fn app() -> Html {

    html! {
        <>
            <Hello />

            <AddUser />
            
            <ShowUsers />

            <Survey />

            <footer>
                <p>{ "Wszelkie prawa zastrzeżone © 2024" }</p>
            </footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
