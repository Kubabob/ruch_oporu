use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub img_src: String,
    pub heading: String,
    pub orange_text: String,
    pub green_text: String,
    pub href: String
}

#[function_component(FloatingBox)]
pub fn floating_box(props: &Props) -> Html {

    let img_src = props.img_src.clone();
    let heading = props.heading.clone();
    let orange_text = props.orange_text.clone();
    let green_text = props.green_text.clone();
    let href = props.href.clone();
    
    html!{
        <div class="floating-box">
            <img src={img_src} />
            <div class="text-wrapper">
                <div class="head-text-wrapper">
                    <h3>{orange_text}</h3>
                    <h2>{heading}</h2>
                </div>

                <a href={href} class="rear-text-wrapper">
                    <h3>{green_text}</h3>
                    <img src="../graphics/arrow-right.svg" />
                </a>
            </div>
        </div>
    }
}