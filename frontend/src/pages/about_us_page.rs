use yew::prelude::*;
use crate::components::navbar::Navbar;

#[function_component(AboutUs)]
pub fn about_us() -> Html {
    html!(
        <>
            <section class="subPage">
                <Navbar color="black" />

                <section class="oNas textBlocksWrapper">
                    <div class="textBlockWrapper">
                    <h2 class="black">{"Kim jesteśmy?"}</h2>
                    <p class="black">{r#"Stowarzyszenie „Gra na Orientację” to wspólnota przyjaciół, powstała w odpowiedzi na doświadczenia dyskryminacji ze względu na orientację psychoseksualną w Związku Harcerstwa Rzeczypospolitej. Tworzą je osoby związane z ZHR – zarówno te, które pełniły funkcje instruktorskie, jak i osoby aktywne lub wcześniej zaangażowane jako harcerki i harcerze. Wszystkim nam zależy na budowaniu bardziej włączającej, bezpiecznej i opartej na szacunku wspólnoty harcerskiej. Działamy w duchu wartości, które wyniosłyśmy z harcerstwa – odpowiedzialności za społeczeństwo, braterstwa i wzajemnego szacunku."#}</p>
                    </div>
                    <div class="textBlockWrapper">
                    <h2 class="black">{"Czym się zajmujemy?"}</h2>
                    <p class="black">{r#"Prowadzimy działania informacyjne i artystyczne, mające na celu poprawę sytuacji osób LGBT+ w ZHR. Poprzez sztukę zwracamy uwagę na przejawy dyskryminacji, których same doświadczyłyśmy i które wciąż dotykają wiele osób. Dzieląc się naszymi doświadczeniami, dążymy do budowania większego zrozumienia i empatii zarówno w środowisku harcerskim, jak i poza nim."#}</p>
                    </div>
                    <div class="textBlockWrapper">
                    <h2 class="black">{"Po co jest ta strona?"}</h2>
                    <p class="black">{r#"Prezentujemy tu naszą perspektywę na sytuację osób LGBT+ w ZHR. Informujemy o realizowanych przez nas projektach oraz misji naszego stowarzyszenia. Strona pełni także rolę przestrzeni, w której osoby LGBT+ z ZHR mogą dzielić się swoimi doświadczeniami i zabierać głos we własnej sprawie bez obaw przed wykluczeniem. Wierzymy, że takie świadectwa pomagają zwiększać świadomość i zrozumienie naszej sytuacji zarówno wśród członków ZHR, jak i w szerszym społeczeństwie."#}</p>
                    </div>
                </section>
            </section>
        </>
    )
}