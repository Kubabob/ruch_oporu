use yew::prelude::*;

use crate::components::navbar::Navbar;
use crate::components::story_box::StoryBox;

#[function_component(HistoriesPage)]
pub fn histories_page() -> Html {
    html!(
        <section class="subPage">
                <Navbar color="black" />

                <section class="histories textBlocksWrapper">
                    <div class="textBlockWrapper">
                    <h2 class="black">{"Poznaj nasze historie"}</h2>
                    <p class="black">
                    {r#"Nasza inicjatywa powstała z przekonania, że osoby nieheteronormatywne związane z ZHR zasługują na przestrzeń, w której mogą dzielić się swoimi doświadczeniami bez lęku i obawy przed wykluczeniem."#}
                    <br/>
                    <br/>
                    {r#"Poprzez nasze działania pragniemy przywrócić osobom LGBT+ poczucie sprawczości, godności i wspólnoty oraz stworzyć możliwość lepszego zrozumienia naszej perspektywy i doświadczeń wśród innych członków ZHR. "#}
                    <br/>
                    <br/>
                    {r#"Wierzymy, że dzielenie się tymi historiami jest kluczem do budowania mostów porozumienia i silniejszej, bardziej wspierającej wspólnoty."#}
                    </p>
                    </div>
                </section>

                <section class="naszeHistorie textBlocksWrapper">
                    <div class="boxesLinkWrapper">
                        <div class="zespolBoxes">
                            <StoryBox
                                index=1
                                title="Nazywam się Sebastian..."
                                text="i jestem nieheteronormatywny. Swoją działalność w harcerstwie zakończyłem po około ośmiu latach, na końcu jako przyboczny. Przez ten cały czas nie czułem się na tyle komfortowo, aby być sobą wśród moich najbliższych..."
                            />

                            <StoryBox
                                index=2
                                title="TAKIE osoby nie mogą być przykładem dla innych"
                                text="Dołączając do harcerstwa nie byłam świadoma mojej orientacji seksualnej. Miałam 12 lat - mało kto jest jej wtedy świadomy. Zaczęło to do mnie docierać dopiero w wieku 15/16 lat..."
                            />

                            <StoryBox
                                index=3
                                title=r#"“Ludzie boją się tego, czego nie znają”"#
                                text="Szczerze chciałam wierzyć, że odpowiedzią na niechęć, wrogość czy obojętność na krzywdę jest rozmowa, przekonywanie kolejnych osób krok po kroku. Nie liczyłam na cuda, wiedziałam że to bardzo idealistyczne podejście..."
                            />

                            <StoryBox
                                index=4
                                title="Jestem drużynową i jestem osobą nieheteronormatywną"
                                text="W moim bliskim otoczeniu (harcerskim jak i nie harcerskim) wszyscy wiedzą o mojej orientacji. Nigdy się nie wstydziłam tego, kim jestem i jak ktoś się mnie o to pytał to z chęcią..."
                            />
                        </div>
                    </div>
                </section>
        </section>
    )
}