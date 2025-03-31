use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct GreenArrowedLinkProps {
    pub href: String,
    pub text: String
}

#[function_component(GreenArrowedLink)]
pub fn green_arrowed_link(props: &GreenArrowedLinkProps) -> Html {
    let href = props.href.clone();
    let text = props.text.clone();

    html!(
        <a href={href} class="rear-text-wrapper">
            <h3>{text}</h3>
            <img src="../graphics/arrow-right.svg" />
        </a>
    )
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub img_src: String,
    pub heading: String,
    pub orange_text: String,
    pub black_text: String,
    pub green_text: String,
    pub href: String
}



#[function_component(FloatingBox)]
pub fn floating_box(props: &Props) -> Html {

    let img_src = props.img_src.clone();
    let heading = props.heading.clone();
    let orange_text1 = props.orange_text.clone();
    let orange_text2 = props.orange_text.clone();
    let black_text1 = props.black_text.clone();
    let black_text2 = props.black_text.clone();
    let green_text = props.green_text.clone();
    let href = props.href.clone();
    
    html!{
        <div class="floating-box">
            <img class="floating-box-img" src={img_src} />
            <div class="text-wrapper">
                <div class="head-text-wrapper">
                    if &orange_text1 != "" {
                        <h3>{orange_text2}</h3>
                    }
                    if &black_text1 != "" {
                        <p>{black_text2}</p>
                    }
                    <h2>{heading}</h2>
                </div>

                <GreenArrowedLink href={href} text={green_text} />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct WyznanieProps {
    pub img_src: String,
    pub heading: String,
    pub orange_text: String,
    pub black_text: String,
    pub green_text: String,
    pub href: String
}


#[function_component(WyznanieFloatingBox)]
pub fn wyznanie_floating_box(props: &WyznanieProps) -> Html {

    let img_src = props.img_src.clone();
    let heading = props.heading.clone();
    let orange_text1 = props.orange_text.clone();
    let orange_text2 = props.orange_text.clone();
    let black_text1 = props.black_text.clone();
    let black_text2 = props.black_text.clone();
    let green_text = props.green_text.clone();
    let href = props.href.clone();
    
    html!{
        <div class="floating-box">
            <img class="floating-box-img" src={img_src} />
            <div class="text-wrapper">
                <div class="head-text-wrapper">
                    <h2>{heading}</h2>
                    
                    if &orange_text1 != "" {
                        <h3>{orange_text2}</h3>
                    }
                    if &black_text1 != "" {
                        <p>{black_text2}</p>
                    }
                </div>

                <GreenArrowedLink href={href} text={green_text} />
            </div>
        </div>
    }
}