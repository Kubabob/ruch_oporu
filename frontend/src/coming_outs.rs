use yew::prelude::*;
use crate::coming_outs_components::text_section::TextSection;
use crate::coming_outs_components::box_section::PaginatedBoxes;

#[function_component(ComingOuts)]
pub fn coming_outs() -> Html {
    html! {
        <>
            <TextSection />
            <section class="articles-page">
                <PaginatedBoxes />
            </section>
        </>
    }
}