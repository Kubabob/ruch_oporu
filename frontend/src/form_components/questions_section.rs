use yew::prelude::*;
use super::expandable_question::ExpandableQuestion; // Assuming ExpandableQuestion is in parent module

#[derive(Properties, PartialEq)]
pub struct QuestionSectionProps {
    pub status: Status,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Status {
    LGBT,
    Ally,
    Innx,
}

#[function_component(QuestionsSection)]
pub fn questions_section(props: &QuestionSectionProps) -> Html {
    html! {
        <div class="faq-section">
            <h1>{"Pytania pomocnicze"}</h1>
            {match props.status {
                Status::LGBT => html! {
                    <>
                        <ExpandableQuestion question="Czy zdecydował_ś się powiedzieć komuś w środowisku harcerskim o swojej orientacji?">
                            <ul class="faq-list">
                                <li>
                                    {"Jakie emocje Ci towarzyszyły przed i po tej rozmowie?"}
                                </li>
                                <li>
                                    {"Jakie reakcje napotkał_ś w odpowiedzi?"}
                                </li>
                            </ul>
                        </ExpandableQuestion>

                        <ExpandableQuestion question="Czy w trakcie Twojej harcerskiej drogi (np. podczas zajęć, kursów, zdobywania stopni) pojawiał się temat nieheteronormatywności?">
                            <ul class="faq-list">
                                <li>
                                    {"Jakie informacje zostały wtedy przekazane?"}
                                </li>
                                <li>
                                    {"Jakie emocje lub refleksje wywołały w Tobie te rozmowy?"}
                                </li>
                            </ul>
                        </ExpandableQuestion>

                        <ExpandableQuestion question="Czy kiedykolwiek czuł_ś, że Twoja orientacja lub tożsamość powinna skłonić Cię do odejścia z harcerstwa?">
                            <ul class="faq-list">
                                <li>
                                    {"Czy ktoś sugerował Ci, że nie pasujesz do tej społeczności?"}
                                </li>
                                <li>
                                    {"Jeśli tak, to jakie emocje to w Tobie wzbudziło?"}
                                </li>
                            </ul>
                        </ExpandableQuestion>

                        <ExpandableQuestion question="Czy nadal działasz w harcerstwie?">
                            <ul class="faq-list">
                                <li>
                                    {"Jeśli tak, czy Twoja orientacja lub tożsamość wpływa na Twoje funkcjonowanie w organizacji? W jaki sposób?"}
                                </li>
                                <li>
                                    {"Jeśli nie, czy orientacja/tożsamość była jednym z powodów Twojego odejścia? Jaki wpływ to na Ciebie miało?"}
                                </li>
                            </ul>
                        </ExpandableQuestion>

                        <ExpandableQuestion question="Z perspektywy czasu, jak patrzysz na swoją historię w harcerstwie?">
                            <ul class="faq-list">
                                <li>
                                    {"Czy zrobił_byś coś inaczej?"}
                                </li>
                                <li>
                                    {"Co powiedział_byś młodszej wersji siebie?"}
                                </li>
                            </ul>
                        </ExpandableQuestion>
                    </>
                },
                Status::Ally => html! {
                    <>
                        <ExpandableQuestion question="Czy w środowisku harcerskim ktoś zdecydował się powiedzieć Ci o swojej orientacji?">
                            <ul class="faq-list">
                                <li>
                                    {"Jakie emocje towarzyszyły Ci podczas tej rozmowy i po niej?"}
                                </li>
                                <li>
                                    {"Czy fakt, że odbyła się w kontekście harcerskim, wpłynął na twoją reakcję?"}
                                </li>
                            </ul>
                        </ExpandableQuestion>

                        <ExpandableQuestion question="Czy w trakcie Twojej harcerskiej drogi (np. podczas zajęć, kursów, zdobywania stopni) pojawiał się temat nieheteronormatywności?">
                            <ul class="faq-list">
                                <li>
                                    {"Jakie informacje zostały wtedy przekazane?"}
                                </li>
                                <li>
                                    {"Jakie emocje lub refleksje wywołały w Tobie te rozmowy?"}
                                </li>
                            </ul>
                        </ExpandableQuestion>

                        <ExpandableQuestion question="Czy kiedykolwiek czuł_ś, że orientacja lub tożsamość jest powodem czyjegoś odejścia z harcerstwa?">
                            <ul class="faq-list">
                                <li>
                                    {"Jakie uczucia wzbudziło to w Tobie?"}
                                </li>
                                <li>
                                    {"Czy spotkał_ś się z sugestiami, że taki powód odejścia jest uzasadniony?"}
                                </li>
                            </ul>
                        </ExpandableQuestion>

                        <ExpandableQuestion question="Z perspektywy czasu, jak patrzysz na swoją postawę wobec osób nieheteronormatywnych w harcerstwie?">
                            <ul class="faq-list">
                                <li>
                                    {"Czy zrobił_byś, powiedział_byś coś inaczej?"}
                                </li>
                                <li>
                                    {"Co powiedział_byś młodszej wersji siebie, która zaczyna wspierać osoby nieheteronormatywne w harcerstwie?"}
                                </li>
                                <li>
                                    {"Czy chciałał_byś powiedzieć coś swoim przełożonym, podopiecznym albo rówieśnikom?"}
                                </li>
                            </ul>
                        </ExpandableQuestion>

                    </>
                },
                Status::Innx => html! {
                    <>
                    </>
                },
            }}
        </div>
    }
}