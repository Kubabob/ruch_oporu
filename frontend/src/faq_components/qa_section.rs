use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct FaqItem {
    question: String,
    answer: String,
}

#[function_component(FaqComponent)]
fn faq_component(props: &FaqItem) -> Html {
    let expanded = use_state(|| false);

    let toggle = {
        let expanded = expanded.clone();
        Callback::from(move |_| {
            expanded.set(!*expanded);
        })
    };

    html! {
        <div class="faq-item">
            <button class="faq-question" onclick={toggle}>
                <div class="plus-icon">
                    <svg class={classes!(expanded.then(|| "rotated"))}
                        viewBox="0 0 24 24"
                        width="24"
                        height="24"
                    >
                        <path fill="currentColor" d="M19 13H13V19H11V13H5V11H11V5H13V11H19V13Z"/>
                    </svg>
                </div>
                <h3>{&props.question}</h3>
            </button>

            {if *expanded {
                html! {
                    <div class="faq-answer">
                        <p>{&props.answer}</p>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}

#[function_component(FaqSection)]
pub fn faq_section() -> Html {
    let faqs1 = vec![
        FaqItem {
            question: "Pytanie 1".into(),
            answer: "Odpowiedź 1".into(),
        },
        FaqItem {
            question: "Pytanie 2".into(),
            answer: "Odpowiedź 2".into(),
        },
        // Add more FAQ items
    ];

    let faqs2 = vec![
        FaqItem {
            question: "Pytanie 1".into(),
            answer: "Odpowiedź 1".into(),
        },
        FaqItem {
            question: "Pytanie 2".into(),
            answer: "Odpowiedź 2".into(),
        },
        // Add more FAQ items
    ];

    html! {
        <section class="faq-section">
            <h2>{"Najczęściej zadawanie pytania przez harcerki/harcerzy"}</h2>
            <div class="faq-container">
                {faqs1.into_iter().map(|faq| {
                    html! { <FaqComponent question={faq.question} answer={faq.answer} /> }
                }).collect::<Html>()}
            </div>

            <h2>{"Najczęściej zadawanie pytania przez osoby nie będące w harcerstwie"}</h2>
            <div class="faq-container">
                {faqs2.into_iter().map(|faq| {
                    html! { <FaqComponent question={faq.question} answer={faq.answer} /> }
                }).collect::<Html>()}
            </div>
        </section>
    }
}
