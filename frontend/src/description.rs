use yew::prelude::*;
use yew_router::prelude::*;

use crate::navigator::Route;

#[function_component(Description)]
pub fn description() -> Html {
    
    html! {
        <>
            <div>
                <p>
                {
                    "Kolektyw/Stowarzyszenie  „Gra na Orientacje” to wspólnota przyjaciół, założona jako odpowiedź na doświadczenie dyskryminacji /ze względu na orientację seksualną/ w ZHR. Inicjatywa jest tworzona przez byłe i obecne osoby członkowskie i osoby instruktorskie ZHR, w  myśl wartości, w których w tej organizacji zostałyśmy wychowane – odpowiedzialności za społeczeństwo, braterstwa i wzajemnego szacunku. "
                }
                </p>

                <p>
                {
                    "Czujemy się zobowiązanx do zabrania głosu i podjęcia próby zmiany organizacji poprzez stworzenie bezpiecznej przestrzeni wypowiedzi"
                }
                </p>

                <p>
                {
                    "A jednocześnie za pomocą działań i interwencji artystycznych, zwrócić uwagę na przejawy dyskryminacji których doświadczyłyśmy, a których wiele osób jeszcze doświadcza, bądź doświadczy."
                }
                </p>

                <p>
                {
                    "Ostatecznie wyjść poza to doświadczenie, tę organizację i działać szerzej. (?) "
                }
                </p>

                <h2>
                {
                    "GEST EMANCYPACYJNY"
                }
                </h2>

                <p>
                {
                    "Otworzyć dyskusje, wyciągnąć na światło dzienne, zwrócić uwagę na problem, co skutkuje modyfikacją działań."
                }
                </p>
            </div>
            <footer>
                <Link<Route> to={Route::Home}>
                    <button>
                    { "Powrót do strony głównej" }
                    </button>
                </Link<Route>>
                <p>{ "Wszelkie prawa zastrzeżone © 2024" }</p>
            </footer>
        </>
    }
}