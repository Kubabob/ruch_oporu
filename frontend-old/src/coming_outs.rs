use crate::coming_outs_components::box_section::PaginatedBoxes;
use crate::coming_outs_components::text_section::TextSection;
use yew::prelude::*;

#[function_component(ComingOuts)]
pub fn coming_outs() -> Html {
    html! {
        <>
            <section class="articles-page">
                <TextSection />
                <PaginatedBoxes />
            </section>
        </>
    }
}
