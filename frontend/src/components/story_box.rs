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
        <div class="boxImageContainer">
            <img loading="lazy" class="boxImage" style="max-width: 50%; height: 50%; align-self: center;" src={"../graphics/gno_logotyp_pion_black.png".to_string()} />
        </div>
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
                        <img src="../graphics/arrow-right-black.svg" />
                    </div>
                </div>
            </div>
            
        </div>
    )
}