use yew::prelude::*;
use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::components::team_story_box::TeamStoryBox;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
  html!(
    <>
      <section class="landingPage">
        <Navbar color="black" />
        <div class="teeth">
          <img src="../graphics/lampion_cze_zeby.png" class="tooth" />
          <img src="../graphics/lampion_cze_zeby.png" class="tooth" />
          <img src="../graphics/lampion_cze_zeby.png" class="tooth" />
          <img src="../graphics/lampion_cze_zeby.png" class="tooth" />
          <img src="../graphics/lampion_cze_zeby.png" class="tooth" />
          <img src="../graphics/lampion_cze_zeby.png" class="tooth" />
          <img src="../graphics/lampion_cze_zeby.png" class="tooth" />
          <img src="../graphics/lampion_cze_zeby.png" class="tooth" />
          <img src="../graphics/lampion_cze_zeby.png" class="tooth" />
        </div>


        <div class="landingPageContentWrapper">
          <section class="hero" id="hero">
            <h1 class="black" id="gra-na-orientacje">{"GRA NA ORIENTACJĘ"}</h1>
            <h3 class="black" id="prosze-nie-zrywac">{"Proszę nie zrywać"}</h3>
          </section>

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

          <section class="colorfulButtons boxesWrapper">
            <a href="nasze-historie" id="historieBox" class="boxWrapper colorfulBoxWrapper">
              <div class="boxHeadingWrapper">
                <span class="boxHeader black">{"Nasze historie"}</span>
              </div>
              <div class="textWrapper horizontal">
                  <p class="black">{"Przeczytaj"}</p>
                <img src="../graphics/arrow-right-black.svg" />
              </div>
            </a>
            <a href="projekty" id="projektyBox" class="boxWrapper colorfulBoxWrapper">
              <div class="boxHeadingWrapper">
                <span class="boxHeader black">{"Nasze projekty"}</span>
              </div>
              <div class="textWrapper horizontal">
                  <p class="black">{"Zobacz więcej"}</p>
                <img src="../graphics/arrow-right-black.svg" />
              </div>
            </a>
            <a href="faq" id="faqBox" class="boxWrapper colorfulBoxWrapper">
              <div class="boxHeadingWrapper">
                <span class="boxHeader black">{"Pytania i odpowiedzi"}</span>
              </div>
              <div class="textWrapper horizontal">
                  <p class="black">{"Zobacz więcej"}</p>
                <img src="../graphics/arrow-right-black.svg" />
              </div>
            </a>
          </section>

          <section class="naszZespol">
            <div class="textBlocksWrapper">
              <div class="textBlockWrapper">
                <h2 class="black">{"Nasz zespół"}</h2>
                <p class="black">{r#"Kto stoi za Stowarzyszeniem Gra na Orientacje? Poznaj nasz zarząd oraz wszystkie jawne i tajne osoby członkowskie. To właśnie one budują przestrzeń do rozmowy i torują drogę do zmian. To dzięki nim inicjatywa przerodziła się w czyny. Poniżej przedstawiamy Ci nasze historie i przebyte ścieżki: te harcerskie dawne lub aktualne, a także prywatne i zawodowe."#}</p>
              </div>
            </div>

            <div class="boxesLinkWrapper">
              <div class="zespolBoxes">
                <TeamStoryBox
                  name="Julia"
                  position="Członkini Zarządu"
                  kiedys="drużynowa harcerek i wędrowniczek, hufcowa, referentka ds harcerek; komendantka 5 akcji zimowych i  7 letnich, działała w Pomorskiej Szkole Instruktorek"
                  poza="studiuje Psychologię oraz Zarządzanie Instytucjami Artystycznymi, animatorka lokalnej społeczności, wspólnie z narzeczoną prowadzi Klub Sąsiedzki"
                  ceni="obserwowanie rozwoju ludzi, z którymi działała, działanie z konkretnymi celami, wyczyn oraz wspólnotę"
                  zdjecie=true
                />

                <TeamStoryBox
                  name="Natalia"
                  position="Członkini Zarządu"
                  kiedys="drużynowa, wicehufcowa, komendantka obozów i kursów metodycznych"
                  poza="koordynatorka projektów, animatorka społeczna, studentka Zarządzania Instytucjami Artystycznymi na UG"
                  ceni="ideę, metodę, relacje i wspólnotę"
                  zdjecie=true
                />

                <TeamStoryBox
                  name="Nel"
                  position="Członkini Zarządu"
                  kiedys="drużynowa, członkini komendy hufca, organizatorka kursów metodycznych, współorganizatorka obozów letnich"
                  poza="magistra zarządzania, studentka pedagogiki"
                  ceni="długotrwałe relacje, wspólnotę, bliskość z naturą, przestrzeń do samorozwoju"
                  zdjecie=true
                />

                <TeamStoryBox
                  name="Kacper"
                  kiedys="drużynowy, czynnie angażuje się w pracę chorągwi i Okręgu"
                  poza="ukończył szkołę muzyczną, studiuje prawo i pracuje w NGO, jego pasją jest praca na rzecz innych"
                  ceni="ludzi, których w nim poznał oraz relacje, które z nimi nawiązał, dzięki niemu zdobył mnóstwo przydatnych życiowo kompetencji i odkrył chęć niesienia pomocy innym"
                  zdjecie=true
                />
              </div>



              <a href="nasz-zespol" class="textWrapper horizontal" style="align-self: end; padding-right: 15%">
                  <h4 class="black">{"Poznaj resztę zespołu"}</h4>
                  <img src="../graphics/arrow-right-black.svg" />
              </a>
            </div>
          </section>
        </div>
        <Footer />
        </section>
    </>
  )
}