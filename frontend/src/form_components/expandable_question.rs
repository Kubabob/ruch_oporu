use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ExpandableQuestionProps {
    pub question: String,
    pub children: Children,
}

#[function_component(ExpandableQuestion)]
pub fn expandable_question(props: &ExpandableQuestionProps) -> Html {
    let is_open = use_state(|| false);
    let toggle = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    html! {
        <div class="expandable-question">
            <button class="question-header" onclick={toggle}>
                <span>{&props.question}</span>
                <span class="toggle-icon">{if *is_open { "▼" } else { "▶" }}</span>
            </button>
            if *is_open {
                <div class="question-content">
                    <div class="question-content-inner">
                        {props.children.clone()}
                    </div>
                </div>
            }
        </div>
    }
}