use yew::prelude::*;

#[derive(Properties, Debug, PartialEq)]
pub struct Props{
    pub color: String
}

#[function_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
    html!(
        <nav>
            <a href="/">
                <img class="logo" src={format!("../graphics/gno_logotyp_poziom_{}.png", props.color)} />
            </a>
            <div class="navButtonsWrapper">
                <a class={classes!("button", props.color.clone())} href="o-nas">{"O Nas"}</a>
                <a class={classes!("button", props.color.clone())} href="">{"Projekty"}</a>
                <a class={classes!("button", props.color.clone())} href="">{"Nasze historie"}</a>
                <a class={classes!("button", props.color.clone())} href="">{"Kontakt"}</a>
            </div>
        </nav>
    )

}
