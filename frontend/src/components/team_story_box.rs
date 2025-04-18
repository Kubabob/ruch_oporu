use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub position: String,
    #[prop_or_default]
    pub kiedys: String,
    #[prop_or_default]
    pub poza: String,
    #[prop_or_default]
    pub ceni: String,
    #[prop_or_default]
    pub teraz: String,
    #[prop_or_default]
    pub zachowuje: String,
    #[prop_or_default]
    pub zdjecie: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            position: "".to_string(),
            kiedys: "".to_string(),
            poza: "".to_string(),
            ceni: "".to_string(),
            teraz: "".to_string(),
            zachowuje: "".to_string(),
            zdjecie: false,
        }
    }
}

#[function_component(TeamStoryBox)]
pub fn team_story_box(props: &Props) -> Html {
    html!(
        <div class="boxWrapper zespolBox">
            if props.name.clone() != "Anonim" {
                if props.zdjecie {
                    <img loading="lazy" class="boxImage" src={
                        format!("../graphics/team_stories/{}.jpeg", props.name.clone())
                    } />    
                } else {
                    <div class="boxImageContainer">
                        <img loading="lazy" class="boxImage" style="max-width: 50%; height: 50%; align-self: center;" src={"../graphics/gno_logotyp_pion_black.png".to_string()} />
                    </div>
                }
                <div class="boxTextWrapperOuter">
                    <div class="boxTextWrapperInner">
                        <div class="boxHeadingWrapper">
                            <span class="boxHeader black">{props.name.clone()}</span>
                            if props.position != "" {
                                <span class="boxSubHeader orange">{props.position.clone()}</span>
                            }
                        </div>
                        <span class="boxText black">
                            <b>{"Kiedyś w ZHR: "}</b> {props.kiedys.clone()}
                            <br/>
                            <b>{"Poza ZHR: "}</b> {props.poza.clone()}
                            <br/>
                            <b>{"Ceni sobie harcerstwo za: "}</b> {props.ceni.clone()}
                        </span>
                    </div>
                </div>
            } else {
                <div class="boxImageContainer">
                    <img loading="lazy" class="boxImage" style="max-width: 50%; height: 50%; align-self: center;" src={"../graphics/gno_logotyp_pion_black.png".to_string()} />
                </div>
                <div class="boxTextWrapperOuter">
                    <div class="boxTextWrapperInner">
                        <div class="boxHeadingWrapper">
                            <span class="boxHeader black">{props.name.clone()}</span>
                        </div>
                        <span class="boxText black">
                            <b>{"Teraz w ZHR: "}</b> {props.teraz.clone()}
                            <br/>
                            <b>{"Zachowuje anonimowość ponieważ: "}</b> {props.zachowuje.clone()}
                        </span>
                    </div>
                </div>
            }
            
        </div>
    )
}