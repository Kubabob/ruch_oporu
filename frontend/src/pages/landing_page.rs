use yew::prelude::*;
use crate::components::footer::Footer;
use crate::components::navbar::Navbar;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
  html!(
    <>
      <section class="landingPage">
        <Navbar color="white" />
        <div class="teeth">
          <img src="../graphics/lampion_cze_zeby.png" class="tooth" />
          <img src="../graphics/lampion_cze_zeby.png" class="tooth" />
          <img src="../graphics/lampion_cze_zeby.png" class="tooth" />
          // <img src="../graphics/lampion_cze_zeby.png" class="tooth" />
        </div>


        <div class="landingPageContentWrapper">
          <section class="hero" id="hero">
            <h1 class="white" id="gra-na-orientacje">{"GRA NA ORIENTACJĘ"}</h1>
            <h3 class="white" id="prosze-nie-zrywac">{"Proszę nie zrywać"}</h3>
          </section>

          <section class="oNas textBlocksWrapper">
            <div class="textBlockWrapper">
              <h2 class="white">{"Kim jesteśmy?"}</h2>
              <p class="white">{r#"Stowarzyszenie „Gra na Orientację” to wspólnota przyjaciół, założona w odpowiedzi na doświadczenie dyskryminacji ze względu na orientację seksualną w Związku Harcerstwa Rzeczypospolitej. Stowarzyszenie tworzą byłe i obecne osoby członkowskie i osoby instruktorskie ZHR, którym zależy na budowaniu bardziej włączającej i bezpiecznej wspólnoty harcerskiej. Działamy w myśl wartości, w których w tej organizacji zostałyśmy wychowane odpowiedzialności za społeczeństwo, braterstwa i wzajemnego szacunku."#}</p>
            </div>
            <div class="textBlockWrapper">
              <h2 class="white">{"Czym się zajmujemy?"}</h2>
              <p class="white">{r#"Prowadzimy działania informacyjne i artystyczne, mające na celu poprawę sytuacji osób LGBT+ w ZHR. Poprzez sztukę zwracamy uwagę na przejawy dyskryminacji, których same doświadczyłyśmy i które wciąż dotykają wiele osób. Dzieląc się naszymi doświadczeniami, dążymy do budowania większego zrozumienia i empatii zarówno w środowisku harcerskim, jak i poza nim."#}</p>
            </div>
            <div class="textBlockWrapper">
              <h2 class="white">{"Po co jest ta strona?"}</h2>
              <p class="white">{r#"Prezentujemy tu naszą perspektywę na sytuację osób LGBT+ w ZHR. Informujemy o realizowanych przez nas projektach oraz misji naszego stowarzyszenia. Strona pełni także rolę przestrzeni, w której osoby LGBT+ z ZHR mogą dzielić się swoimi doświadczeniami i zabierać głos we własnej sprawie bez obaw przed wykluczeniem. Wierzymy, że takie świadectwa pomagają zwiększać świadomość i zrozumienie naszej sytuacji zarówno wśród członków ZHR, jak i w szerszym społeczeństwie."#}</p>
            </div>
          </section>

          <section class="colorfulButtons boxesWrapper">
            <div class="boxWrapper" style="background-color: #C6341C;">
              <div class="boxHeadingWrapper">
                <span class="boxHeader white">{"Nasze historie"}</span>
              </div>
              <div class="textWrapper horizontal">
                <p class="white">{"Przeczytaj"}</p>
                <img src="../graphics/arrow-right-white.svg" />
              </div>
            </div>
            <div class="boxWrapper" style="background-color: #019849;">
              <div class="boxHeadingWrapper">
                <span class="boxHeader white">{"Nasze projekty"}</span>
              </div>
              <div class="textWrapper horizontal">
                <p class="white">{"Zobacz więcej"}</p>
                <img src="../graphics/arrow-right-white.svg" />
              </div>
            </div>
            <div class="boxWrapper" style="background-color: #54BEE0;">
              <div class="boxHeadingWrapper">
                <span class="boxHeader white">{"Pytania i odpowiedzi"}</span>
              </div>
              <div class="textWrapper horizontal">
                <p class="white">{"Zobacz więcej"}</p>
                <img src="../graphics/arrow-right-white.svg" />
              </div>
            </div>
          </section>

          <section class="naszZespol textBlocksWrapper">
            <div class="textBlockWrapper">
              <h2 class="white">{"Nasz zespół"}</h2>
              <p class="white">{r#"Kto stoi za Stowarzyszeniem Gra na Orientacje? Poznaj nasz zarząd oraz wszystkie jawne i tajne osoby członkowskie. To właśnie one budują przestrzeń do rozmowy i torują drogę do zmian. To dzięki nim inicjatywa przerodziła się w czyny. Poniżej przedstawiamy Ci nasze historie i przebyte ścieżki: te harcerskie dawne lub aktualne, a także prywatne i zawodowe."#}</p>
            </div>

            <div class="boxesLinkWrapper">
              <div class="boxesWrapper zespolBoxes">
                <div class="boxWrapper zespolBox">
                  <img class="boxImage" src="../graphics/szachownica.png" />
                  <div class="boxTextWrapperOuter">
                    <div class="boxTextWrapperInner">
                      <div class="boxHeadingWrapper">
                        <span class="boxHeader black">{"Julia Raczyńska"}</span>
                        <span class="boxSubHeader orange">{"Członkini Zarządu"}</span>
                      </div>
                      <span class="boxText black">
                        <b>{"Kiedyś w ZHR: "}</b> {"drużynowa harcerek i wędrowniczek, hufcowa, referentka ds harcerek; komendantka 5 akcji zimowych i 7 letnich, działała w Pomorskiej Szkole Instruktorek"}
                        <br/>
                        <b>{"Poza ZHR: "}</b> {"studiuje Psychologię oraz Zarządzanie Instytucjami Artystycznymi, animatorka lokalnej społeczności, wspólnie z narzeczoną prowadzi Klub Sąsiedzki"}
                        <br/>
                        <b>{"Ceni sobie harcerstwo za: "}</b> {"obserwowanie rozwoju ludzi, z którymi działała, działanie z konkretnymi celami, wyczyn oraz wspólnotę"}
                      </span>
                    </div>
                  </div>
                </div>
                <div class="boxWrapper zespolBox">
                  <img class="boxImage" src="../graphics/szachownica.png" />
                  <div class="boxTextWrapperOuter">
                    <div class="boxTextWrapperInner">
                      <div class="boxHeadingWrapper">
                        <span class="boxHeader black">{"Natalia Zarębska"}</span>
                        <span class="boxSubHeader orange">{"Członkini Zarządu"}</span>
                      </div>
                      <span class="boxText black">
                        <b>{"Kiedyś w ZHR: "}</b> {"drużynowa, wicehufcowa, komendantka obozów i kursów metodycznych"}
                        <br/>
                        <b>{"Poza ZHR: "}</b> {"koordynatorka projektów, animatorka społeczna, studentka Zarządzania Instytucjami Artystycznymi na UG"}
                        <br/>
                        <b>{"Ceni sobie harcerstwo za: "}</b> {"ideę, metodę, relacje i wspólnotę"}
                      </span>
                    </div>
                  </div>
                </div>
                <div class="boxWrapper zespolBox">
                  <img class="boxImage" src="../graphics/szachownica.png" />
                  <div class="boxTextWrapperOuter">
                    <div class="boxTextWrapperInner">
                      <div class="boxHeadingWrapper">
                        <span class="boxHeader black">{"Nel Krysiak"}</span>
                        <span class="boxSubHeader orange">{"Członkini Zarządu"}</span>
                      </div>
                      <span class="boxText black">
                        <b>{"Kiedyś w ZHR: "}</b> {"drużynowa, członkini komendy hufca, organizatorka kursów metodycznych, współorganizatorka obozów letnich"}
                        <br/>
                        <b>{"Poza ZHR: "}</b> {"magistra zarządzania, studentka pedagogiki"}
                        <br/>
                        <b>{"Ceni sobie harcerstwo za: "}</b> {"długotrwałe relacje, wspólnotę, bliskość z naturą, przestrzeń do samorozwoju"}
                      </span>
                    </div>
                  </div>
                </div>
              </div>

              <div class="textWrapper horizontal" style="align-self: flex-end;">
                  <h4 class="white">{"Poznaj resztę zespołu"}</h4>
                  <img src="../graphics/arrow-right-white.svg" />
              </div>
            </div>
          </section>
        </div>
        <Footer />
        </section>
    </>
  )
}