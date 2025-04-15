use yew::prelude::*;
use gloo_console::log;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub index: u8,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub text: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            index: 0,
            title: "".to_string(),
            text: "".to_string(),
        }
    }
}

#[function_component(StoryBox)]
pub fn story_box(props: &Props) -> Html {
    html!(
        <div class="boxWrapper storyBox">
            <img class="boxImage" src={
                if std::path::Path::new(&format!("../graphics/histories/{:#?}.jpg", props.index.clone().to_string())).exists() {
                    log!("We did it boys!");
                    format!("../graphics/histories/{:#?}.jpg", props.index.clone().to_string())
                } else {
                    log!("We did not do it boys!");
                    "../graphics/anonymous.svg".to_string()
                }
            } />
            <div class="boxTextWrapperOuter">
                <div class="boxTextWrapperInner">
                    <div class="boxHeadingWrapper">
                        <span class="boxHeader black">{props.title.clone()}</span>
                    </div>
                    <span class="boxText black">
                        {props.text.clone()}
                    </span>
                    <div class="textWrapper horizontal">
                        <a href={format!("nasze-historie/{}", props.index.clone())}>
                            <p class="black">{"Przeczytaj"}</p>
                        </a>
                        <img src="../graphics/arrow-right.svg" />
                    </div>
                </div>
            </div>
            
        </div>
    )
}