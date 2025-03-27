use yew::prelude::*;
use crate::contact_components::contact_page::ContactPage;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <>
            <ContactPage />
        </>
    }
}