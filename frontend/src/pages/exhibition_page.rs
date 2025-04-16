use yew::prelude::*;
use crate::components::navbar::Navbar;

#[function_component(ExhibitionPage)]
pub fn exhibition_page() -> Html {
    html!(
        <section class="subPage">
            <Navbar color="black" />
            <section class="oNas textBlocksWrapper">
                <div class="textBlockWrapper">
                    <h2 class="black">{r#"Wystawa "Wystąp""#}</h2>
                    <div class="imageTextBlockWrapper">
                        <img class="exhibitionImage" src="../graphics/exhibition/1.jpg" />
                        <p class="black">
                            <b>{r#""Wystąp” to wystawa interdyscyplinarna przedstawiająca historie nieheteronormatywności w Związku Harcerstwa Rzeczypospolitej. "#}</b>
                            <br/>
                            <br/>
                            {"Naszym celem jest stworzenie przestrzeni do wyrażenia emocji i doświadczeń osób LGBTQIA+ oraz osób wspierających poprzez twórczość artystyczną, a tym samym przełamanie tabu. Traktujemy wystawę jako formę dialogu, który ma szansę przybliżyć perspektywę mniejszości oraz pomóc w budowaniu bardziej świadomej oraz otwartej społeczności. "}
                            <br/>
                            <br/>
                            {"Do współtworzenia z nami wystawy zapraszamy wszystkich, którzy chcą podzielić się swoimi przeżyciami i/lub wesprzeć tę ideę poprzez twórczość artystyczną. A tych, którzy będą chcieli uczestniczyć w naszym wydarzeniu i obejrzeć dzieła – rezerwujcie sobie czas na wernisaż "}<b>{"już w czerwcu na terenie Trójmiasta!"}</b>
                            <br/>
                            <br/>
                            {r#"Wkrótce zostanie podana konkretna data oraz miejsce wydarzenia. Jeśli natomiast nie uda Ci się wziąć udziału w naszym wydarzeniu – nie martw się! To nie jedyne miejsce, w którym ukaże się “Wystąp”. Może niedługo zawitamy w Twoim mieście? ;) "#}
                        </p>
                    </div>
                </div>
        

                <div class="textBlockWrapper">
                    <p class="black">
                        {"A jeśli chcesz współtworzyć wystawę wraz z nami…"}
                        <br/>
                        <b>{"Obecnie startujemy z  drugą turą zgłoszeń, więc sprawdź naszego Open Call’a !"}</b>
                        <br/>
                        <br/>
                        {"Do wzięcia udziału w wystawie możesz się zgłosić jako:"}
                        <ol>
                            <li><b>{"Osoba tworząca z osobistym doświadczeniem "}</b> {"- osoby z doświadczeniem nieheteronormatywności w środowisku harcerskim ZHR, które chcą wyrazić się poprzez akt twórczy w dowolnej formie."}</li>
                            <li><b>{"Osoba z osobistym doświadczeniem "}</b> {"-  osoby z doświadczeniem nieheteronormatywności w środowisku harcerskim w ZHR, które nie czują się komfortowo z tworzeniem, ale chcą aby ich historia została opowiedziana. W tym przypadku połączymy Cię z osobą tworzącą, która przekształci Twoje przeżycia w formę artystyczną."}</li>
                            <li><b>{"Osoba tworząca "}</b> {"bez własnych doświadczeń w tym temacie, które chcą wesprzeć wystawę swoim talentem i współpracować z osobami mającymi swoje historie do opowiedzenia poprzez przekształcenie doświadczeń w formę artystyczną."}</li>
                        </ol>
                        <br/>
                        <br/>
                        <br/>
                        <b>{"TERMINY:"}</b>
                        <ul>
                            <li><b>{"30 kwietnia 2025r "}</b> {"- przyjmowanie zgłoszeń poprzez "}<a href="https://docs.google.com/forms/d/e/1FAIpQLScmuKCk-VVhpmu__gnMOpojuXSvzpZvMg7f7JXlgyiFXxMtFw/viewform">{"formularz online"}</a></li>
                            <li><b>{"18 maja 2025r "}</b> {"- deadline przygotowania prac na wystawę."}</li>
                            <li><b>{"czerwiec 2025r "}</b> {"- wernisaż wystawy na terenie Trójmiasta."}</li>
                        </ul>
                        <br/>
                        <br/>
                        {"Po zamknięcia formularza, każda z osób zgłoszonych otrzyma od nas wiadomość na podany adres e-mail z dalszymi, szczegółowymi informacjami."}
                        <br/>
                        <b>{"W razie jakichkolwiek pytań lub wątpliwości prosimy o kontakt na adres "}<a href="mailto:wystawaharc.lgbt@gmail.com">{"wystawaharc.lgbt@gmail.com"}</a></b>
                        <br/>
                        <br/>
                        {"Załączniki:"}
                        <ol>
                            <li>{"Zaproszenie OPEN CALL"}</li>
                            <li>{"Regulamin wystawy"}</li>
                        </ol>
                        <br/>
                        <br/>
                        {"(tu są pliki załączniki: "}
                        <a href="https://drive.google.com/drive/folders/18BfUoesu2amXZ0rVrezNzO1peoJ4bHw_?usp=drive_link">{"https://drive.google.com/drive/folders/18BfUoesu2amXZ0rVrezNzO1peoJ4bHw_?usp=drive_link"}</a>
                        {" )"}
                        <br/>
                        <br/>
                        <br/>
                        <b>{"Zespół kuratorski wystawy:"}</b>
                        <br/>
                        {"Barbara Dowgiałło, Nel Krysiak, Zofia Pinkiewicz,"}
                        <br/>
                        {"Zuzanna Ruszkiewicz, Natalia Zarębska"}

                    </p>
                </div>
        
        
                <div class="textBlockWrapper">
                </div>
            </section>
        </section>
    )
}