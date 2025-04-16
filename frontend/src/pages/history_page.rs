use yew::prelude::*;

use crate::components::navbar::Navbar;
use crate::components::histories::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub index: u8,
}

#[function_component(HistoryPage)]
pub fn history_page(props: &Props) -> Html {
    html!(
        <section class="subPage">
            <Navbar color="black" />
            <section class="histories textBlocksWrapper">
                <div class="textBlockWrapper">
                    {
                        match props.index {
                            1 => html!(<h1::H1 />),
                            2 => html!(<h2::H2 />),
                            3 => html!(<h3::H3 />),
                            4 => html!(<h4::H4 />),
                            _ => html!("sorry")
                        }
                    }
                </div>
            </section>
        </section>
    )
}