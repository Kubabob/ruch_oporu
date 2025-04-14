use yew::prelude::*;
use super::form_box::{InputField, RadioInput, SubmitButton, CheckboxInput};
use super::floating_box::GreenArrowedLink;
use super::qa::{IdeaQuestions, BasicQuestions};

#[function_component(FormPage)]
pub fn form_page() -> Html {
    let tytul_wpisu = use_state(|| String::new());
    let wybrany_cytat = use_state(|| String::new());

    html!(
        <>
            <div class="form-wrapper">
                <h1 class="form-header">{"Opowiedz mi swoją historię..."}</h1>
                <div class="form-description">
                    <p>{"Podziel się swoją historią – opowiedz o emocjach, wyzwaniach i reakcjach z jakimi spotkał_ś się w harcerstwie jako osoba LGBT+ lub osoba wspierająca."}</p>
                    <p>{"Każda historia jest inna, więc możesz napisać ją w dowolnej formie, jaka Ci odpowiada."}</p>
                    <p>{"Jeśli nie wiesz od czego zacząć, daj sobie chwilę na przemyślenie swojego doświadczenia lub skorzystaj z pytań pomocniczych poniżej. Pamiętaj tylko o określonych niżej zasadach:"}</p>
                    <ul>
                        <li>{"Pisz tylko o swoim osobistym doświadczeniu - inne historie zostaw innym do opowiedzenia."}</li>
                        <li><p>{"Nie używaj mowy nienawiści. Nie wiesz co to? Sprawdź tu: "}<a href="/mowa-nienawisci">{"(link)"}</a></p></li>
                        <li>{"Będziemy publikować tylko wypowiedzi na temat doświadczeń nieheteronormatywności w ZHR."}</li>
                        <li>{"Będziemy publikować wypowiedzi tylko na temat doświadczeń osób nieheteronormatywnych i osób je wspierających w ZHR."}</li>
                        <li>{"W zależności od potrzeb, poprawimy interpunkcję czy ortografię przed publikacją tekstu."}</li>
                        <li>{"Wszelkie imiona będą przez nas zmienione dla zachowania anonimowości."}</li>
                    </ul>
                </div>
                <form class="form-box-wrapper">
                    <div class="role-choice">
                        <RadioInput label="LGBT" name="role" value="LGBT" checked=true />
                        <RadioInput label="Ally" name="role" value="Ally" checked=false />
                        <RadioInput label="Innx" name="role" value="Innx" checked=false />
                    </div>

                    <div class="text-input-wrapper">
                        <label for="historia" id="historia-label">{"Miejsce na Twoją historię"}</label>
                        <div class="for-unsures-wrapper">
                            <p>{"Jeśli nie wiesz od czego zacząć - możesz przejrzeć nasze pytania pomocnicze."}</p>
                            <GreenArrowedLink href="#pytania-pomocnicze" text="Pytania pomocnicze" />
                        </div>
                        <textarea id="historia" placeholder="Wpisz" rows=8></textarea>
                        
                        <InputField input_type="text" label="Tytuł wpisu" placeholder="Wpisz" value={tytul_wpisu} />
                        <InputField input_type="text" label="Wybrany cytat/krótki fragment, który będzie wyszczególniony na stronie:" placeholder="Wpisz" value={wybrany_cytat} />
                    </div>

                    <div class="checkbox-wrapper">
                        <CheckboxInput value="czy_grafika" name="czy_grafika" label="Czy chcesz dodać grafikę do swojego wpisu?" checked=false />
                        <CheckboxInput value="podpis" name="podpis" label="Czy chcesz podpisać się imieniem i nazwiskiem?" checked=false />
                        <CheckboxInput value="autentyczny_wpis" name="autentyczny_wpis" label="Potwierdzam, że wpis opisuje moje osobiste doświadczenie jako osoba nieheteronormatywna lub wspierająca. Moja wypowiedź nie zawiera mowy nienawiści." checked=false />
                        <div>
                            <input type="checkbox" id="czy_upublicznic" name="czy_upublicznic" value="czy_upublicznic" checked=false/>
                            <label for="czy_upublicznic">{"Wyrażam zgodę na upublicznienie mojego wpisu na stronie "} <a href="https://granaorientacje.pl">{"www.granaorientacje.pl"}</a></label>
                        </div>
                        <CheckboxInput value="czy_fragmenty" name="czy_fragmenty" label="Wyrażam zgodę na wykorzystywanie fragmentów tekstów do promocji projektu w mediach społeczniościowych." checked=false />
                        <div>
                            <input type="checkbox" id="czy_regulamin" name="czy_regulamin" value="czy_regulamin" checked=false/>
                            <label for="czy_regulamin">{"Zgadzam się z regulaminem projektu dostępnym na stronie "} <a href="regulamin">{"regulamin"}</a></label>
                        </div>
                        <CheckboxInput value="czy_przetwarzanie_danych" name="czy_przetwarzanie_danych" label="Wyrażam zgodę na przetwarzanie moich danych osobowych podanych w powyższym formularzu przez Stowarzyszenie Gra Na Orientację." checked=false />
                    </div>

                    <SubmitButton label="Wyślij" />
                </form>
            </div>

            <div id="pytania-pomocnicze">
                <div class="pytania-ideowe-wrapper">
                    <h2 class="qa-header">{"Pytania ideowe"}</h2>
                    <IdeaQuestions />
                </div>
                
                <div class="pytania-podstawowe-wrapper">
                    <h2 class="qa-header">{"Pytania podstawowe"}</h2>
                    <BasicQuestions />
                </div>
            </div>
        </>
    )
}