use yew::prelude::*;

use crate::components::navbar::Navbar;
use crate::components::team_story_box::TeamStoryBox;

#[function_component(TeamStoriesPage)]
pub fn team_stories_page() -> Html {
    html!(
        <section class="subPage">
            <Navbar color="black" />

            <section class="naszZespol textBlocksWrapper">
                <div class="textBlockWrapper">
                    <h2 class="black">{"Nasz zespół"}</h2>
                    <p class="black">
                    <b>{r#"Kto stoi za Stowarzyszeniem Gra na Orientację? "#}</b>
                    <br/>
                    <br/>
                    {"Poznaj nasz zarząd oraz wszystkie jawne i tajne osoby członkowskie. To właśnie one budują przestrzeń do rozmowy i torują drogę do zmian. To dzięki nim inicjatywa przerodziła się w czyny."}
                    <br/>
                    <br/>
                    {"Poniżej przedstawiamy Ci nasze historie i przebyte ścieżki: te harcerskie – dawne lub aktualne, a także prywatne i zawodowe."}
                    </p>
                </div>
            </section>

            <section class="naszZespol textBlocksWrapper">
                <div class="boxesLinkWrapper">
                    <div class="zespolBoxes">
                        <TeamStoryBox
                            name="Julia Raczyńska"
                            position="Członkini Zarządu"
                            kiedys="drużynowa harcerek i wędrowniczek, hufcowa, referentka ds harcerek; komendantka 5 akcji zimowych i  7 letnich, działała w Pomorskiej Szkole Instruktorek"
                            poza="studiuje Psychologię oraz Zarządzanie Instytucjami Artystycznymi, animatorka lokalnej społeczności, wspólnie z narzeczoną prowadzi Klub Sąsiedzki"
                            ceni="obserwowanie rozwoju ludzi, z którymi działała, działanie z konkretnymi celami, wyczyn oraz wspólnotę"
                        />

                        <TeamStoryBox
                            name="Natalia Zarębska"
                            position="Członkini Zarządu"
                            kiedys="drużynowa, wicehufcowa, komendantka obozów i kursów metodycznych"
                            poza="koordynatorka projektów, animatorka społeczna, studentka Zarządzania Instytucjami Artystycznymi na UG"
                            ceni="ideę, metodę, relacje i wspólnotę"
                        />

                        <TeamStoryBox
                            name="Nel Krysiak"
                            position="Członkini Zarządu"
                            kiedys="drużynowa, członkini komendy hufca, organizatorka kursów metodycznych, współorganizatorka obozów letnich"
                            poza="magistra zarządzania, studentka pedagogiki"
                            ceni="długotrwałe relacje, wspólnotę, bliskość z naturą, przestrzeń do samorozwoju"
                        />

                        <TeamStoryBox
                            name="Anonim"
                            teraz="drużynowa, nieheteronormatywna"
                            zachowuje="nie chce narażać drużyny na potencjalne konsekwencje; planuje dalej się rozwijać harcersko w stopniach instruktorskich "
                        />

                        <TeamStoryBox
                            name="Jagoda Jankowska"
                            kiedys="aktywna instruktorka, prowadziła gdyńską drużynę harcerek, działała w Zarządzie Okręgu i prowadziła Zespół Medialny na Pomorzu"
                            poza="logopedka, dba o prawidłowy rozwój mowy dzieci i młodych dorosłych, w pracy kieruje się zasadą, że nic lepiej nie rozgrzewa aparatu artykulacyjnego niż pożądana dawka śmiechu"
                            ceni="lata spędzone wśród swoich leśnych przyjaciół; uważa, że ZHR wyposażył ją w najpiękniejsze wspomnienia i przyjaźnie jakie mogła kiedykolwiek zdobyć"
                        />

                        <TeamStoryBox
                            name="Anonim"
                            teraz="drużynowa, podharcmistrzyni z zajawką na prowadzenie kursów, jej ulubiony element ręki metody to ,,dobrowolność”, na mundurze nosi wyszytą tęczę"
                            zachowuje="chce zachować spokój ducha; z obawy, sama do końca nie wiedząc przed czym"
                        />

                        <TeamStoryBox
                            name="Barbara Dowgiałło"
                            kiedys="przyboczna w 25 Drużynie Harcerek im. Danuty Siedzikówny Inki"
                            poza="artystka, muzyczka, kompozytorka, copywriterka, producentka wydarzeń artystycznych, animatorka-społeczna"
                            ceni="nauke tego jak stworzyć coś z niczego, szkołę budowania relacji oraz za danie jej możliwości do życia w bliskości z naturą"
                        />

                        <TeamStoryBox
                            name="Anonim"
                            teraz="była drużynowa, prowadziła wszelkiego rodzaju kursy metodyczne"
                            zachowuje="bez niej straciłaby zaufanie i wiarygodność ze strony członków i członkiń ZHRu, uniemożliwiłoby jej to swobodne działanie w organizacji"
                        />

                        <TeamStoryBox
                            name="Katarzyna Rusin"
                            kiedys="przyboczna, kuchmistrzyni"
                            poza="studentka filologii germańskiej, entuzjastka języków i rozwoju wokalnego"
                            ceni="nawiązane więzi, możliwość wyjścia ze strefy komfortu i poznanie świata z innej perspektywy"
                        />

                        <TeamStoryBox
                            name="Anonim"
                        />

                        <TeamStoryBox
                            name="Anonim"
                        />

                        <TeamStoryBox
                            name="Klaudia Hołosiewicz"
                            kiedys="instruktorka, drużynowa wędrowniczek przygotowująca następczynię, podejmuje dyskusje na trudne tematy w środowisku"
                            poza="studentka psychologii udzielająca się społecznie i artystycznie"
                            ceni="bycie dla niej bezpieczną przestrzenią do pogłębiania relacji, wsparciem w samorozwoju oraz dobrym przygotowaniem do dorosłości"
                        />

                        <TeamStoryBox
                            name="Anonim"
                            teraz="była drużynowa, organizatorka kursów zastępowych i metodycznych, kiedyś również zaangażowana instruktorka"
                            zachowuje="nie chce, aby jej działania zmieniły postrzeganie jednostki, którą prowadziła"
                        />

                        <TeamStoryBox
                            name="Zofia Pinkiewicz"
                            kiedys="drużynowa, przewodnicząca kapituły wędrowniczki"
                            poza="początkująca reżyserka, dramaturżka i dyrygentka; większość swojego czasu spędza w teatrze"
                            ceni="wychowanie w poczuciu odpowiedzialności za otaczającą nas rzeczywistość, zbudowanie wiary w swoją sprawczość i przygodę!"
                        />

                        <TeamStoryBox
                            name="Kacper Romanowski"
                            kiedys="drużynowy, czynnie angażuje się w pracę chorągwi i Okręgu"
                            poza="ukończył szkołę muzyczną, studiuje prawo i pracuje w NGO, jego pasją jest praca na rzecz innych"
                            ceni="ludzi, których w nim poznał oraz relacje, które z nimi nawiązał, dzięki niemu zdobył mnóstwo przydatnych życiowo kompetencji i odkrył chęć niesienia pomocy innym"
                        />

                        <TeamStoryBox
                            name="Maria Unterschuetz"
                            kiedys="była przyboczna harcerek i wędrowniczka, angażuje się w ruch wodny jako niezależna harcerka"
                            poza="filozofuje na uczelni i poza nią, pracuje z dziećmi i podróżuje, uczy się języka niemieckiego (nazwisko zobowiązuje)"
                            ceni="porządny warsztat pracy w grupie i umiejętności społecznych,to właśnie harcerstwo zaraziło ją pasją do żeglowania i zamiłowaniem do dzikiej przyrody"
                        />

                    </div>
                </div>
            </section>
        </section>
    )

}
