use crate::exhibition_components::exhibition_page::ExhibitionPage;
use yew::prelude::*;

#[function_component(Exhibition)]
pub fn exhibition() -> Html {
    html! {
        <>
            <ExhibitionPage />
        </>
    }
}
