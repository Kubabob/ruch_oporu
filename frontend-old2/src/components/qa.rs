use yew::prelude::*;


#[function_component(IdeaQuestions)]
pub fn idea_questions() -> Html {
    html!(
        <div class="qa">
            <details class="answer">
                <summary class="question">{"Czy zdecydował_ś się powiedzieć komuś w środowisku harcerskim o swojej orientacji?"}</summary>
                <ul>
                    <li>{"Jakie emocje Ci towarzyszyły przed i po tej rozmowie?"}</li>
                    <li>{"Jakie reakcje napotkał_ś w odpowiedzi?"}</li>
                </ul>
            </details>

            <details class="answer">
                <summary class="question">{"Czy w trakcie Twojej harcerskiej drogi (np. podczas zajęć, kursów, zdobywania stopni) pojawiał się temat nieheteronormatywności?"}</summary>
                <ul>
                    <li>{"Jakie informacje zostały wtedy przekazane?"}</li>
                    <li>{"Jakie emocje lub refleksje wywołały w Tobie te rozmowy?"}</li>
                </ul>
            </details>

            <details class="answer">
                <summary class="question">{"Czy kiedykolwiek czuł_ś, że Twoja orientacja lub tożsamość powinna skłonić Cię do odejścia z harcerstwa?"}</summary>
                <ul>
                    <li>{"Czy ktoś sugerował Ci, że nie pasujesz do tej społeczności?"}</li>
                    <li>{"Jeśli tak, to jakie emocje to w Tobie wzbudziło?"}</li>
                </ul>
            </details>

            <details class="answer">
                <summary class="question">{"Czy nadal działasz w harcerstwie?"}</summary>
                <ul>
                    <li>{"Jeśli tak, czy Twoja orientacja lub tożsamość wpływa na Twoje funkcjonowanie w organizacji? W jaki sposób?"}</li>
                    <li>{"Jeśli nie, czy orientacja/tożsamość była jednym z powodów Twojego odejścia? Jaki wpływ to na Ciebie miało?"}</li>
                </ul>
            </details>

            <details class="answer">
                <summary class="question">{"Z perspektywy czasu, jak patrzysz na swoją historię w harcerstwie?"}</summary>
                <ul>
                    <li>{"Czy zrobił_byś coś inaczej?"}</li>
                    <li>{"Co powiedział_byś młodszej wersji siebie?"}</li>
                </ul>
            </details>
        </div>
    )
}

#[function_component(BasicQuestions)]
pub fn basic_questions() -> Html {
    html!(
        <div class="qa">
            <details class="answer">
                <summary class="question">{"Czy w środowisku harcerskim ktoś zdecydował się powiedzieć Ci o swojej orientacji?"}</summary>
                <ul>
                    <li>{"Jakie emocje towarzyszyły Ci podczas tej rozmowy i po niej?"}</li>
                    <li>{"Czy fakt, że odbyła się w kontekście harcerskim, wpłynął na twoją reakcję?"}</li>
                </ul>
            </details>

            <details class="answer">
                <summary class="question">{"Czy w trakcie Twojej harcerskiej drogi (np. podczas zajęć, kursów, zdobywania stopni) pojawiał się temat nieheteronormatywności?"}</summary>
                <ul>
                    <li>{"Jakie informacje zostały wtedy przekazane?"}</li>
                    <li>{"Jakie emocje lub refleksje wywołały w Tobie te rozmowy?"}</li>
                </ul>
            </details>

            <details class="answer">
                <summary class="question">{"Czy kiedykolwiek czuł_ś, że orientacja lub tożsamość jest powodem czyjegoś odejścia z harcerstwa?"}</summary>
                <ul>
                    <li>{"Jakie uczucia wzbudziło to w Tobie?"}</li>
                    <li>{"Czy spotkał_ś się z sugestiami, że taki powód odejścia jest uzasadniony?"}</li>
                </ul>
            </details>

            <details class="answer">
                <summary class="question">{"Z perspektywy czasu, jak patrzysz na swoją postawę wobec osób nieheteronormatywnych w harcerstwie?"}</summary>
                <ul>
                    <li>{"Czy zrobił_byś, powiedział_byś coś inaczej?"}</li>
                    <li>{"Co powiedział_byś młodszej wersji siebie, która zaczyna wspierać osoby nieheteronormatywne w harcerstwie?"}</li>
                    <li>{"Czy chciałał_byś powiedzieć coś swoim przełożonym, podopiecznym albo rówieśnikom?"}</li>
                </ul>
            </details>
        </div>
    )
}