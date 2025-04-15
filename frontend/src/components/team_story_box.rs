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
        }
    }
}

#[function_component(TeamStoryBox)]
pub fn team_story_box(props: &Props) -> Html {
    html!(
        <div class="boxWrapper zespolBox">
            if props.name.clone() != "Anonim" {
                <img class="boxImage" src={
                    if std::path::Path::new(&format!("../graphics/team_stories/{}.png", props.name.clone())).exists() {
                        format!("../graphics/team_stories/{}.png", props.name.clone())
                    } else {
                        "../graphics/anonymous.svg".to_string()
                    }
                } />
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
                <img class="boxImage" src={"../graphics/team_stories/szachownica.png".to_string()} />
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