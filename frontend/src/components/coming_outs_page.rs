use yew::prelude::*;
use super::form_box::LinkedButton;
use super::floating_box::WyznanieFloatingBox;

#[function_component(ComingOutsPage)]
pub fn coming_outs_page() -> Html {
    html!(
        <section class="coming-outs">
            <section class="page-description">
                <div class="text-wrapper" id="description-text-wrapper">
                    <h3 class="description-header">{"Anonimowe coming outy"}</h3>
                    <div class="description-wrapper">
                        <p>
                            {"Nasza inicjatywa powstała z przekonania, że osoby nieheteronormatywne związane z ZHR zasługują na przestrzeń, w której mogą dzielić się swoimi doświadczeniami bez lęku i obawy przed wykluczeniem."}
                        </p>
                        <p>
                            {"Poprzez nasze działania pragniemy przywrócić osobom LGBT+ poczucie sprawczości, godności i wspólnoty oraz stworzyć możliwość lepszego zrozumienia naszej perspektywy i doświadczeń wśród innych członków ZHR."}
                        </p>
                        <p>
                            {"Wierzymy, że dzielenie się tymi historiami jest kluczem do budowania mostów porozumienia i silniejszej, bardziej wspierającej wspólnoty."}
                        </p>
                    </div>
                </div>
                
                <div class="text-wrapper">
                    <h3 class="description-header" id="wypelnij-formularz">{"Chcesz aby Twoja historia się tutaj pojawiła?"}</h3>
                    <LinkedButton label="Wypełnij anonimowy formularz" href="podziel-sie" />
                </div>

            </section>

            <section class="floating-boxes-flex-grid">
                <WyznanieFloatingBox 
                    img_src="../graphics/anonymous.png" 
                    heading="Nazywam się Sebastian..." 
                    orange_text="" 
                    black_text="i jestem nieheteronormatywny. Swoją diałalność w harcerstwie zakończyłem po okołu ośmiu latach, na końcu jako przyboczny. Przez ten cały czas nie czułem się na tyle komfortowo, aby być sobą wśród moich najbliższych..."
                    green_text="Przeczytaj"
                    href="wyznania/1"
                />
                <WyznanieFloatingBox
                    img_src="../graphics/anonymous.png" 
                    heading="Nazywam się Sebastian..." 
                    orange_text="" 
                    black_text="i jestem nieheteronormatywny. Swoją diałalność w harcerstwie zakończyłem po okołu ośmiu latach, na końcu jako przyboczny. Przez ten cały czas nie czułem się na tyle komfortowo, aby być sobą wśród moich najbliższych..."
                    green_text="Przeczytaj"
                    href="wyznania/1"
                />
                <WyznanieFloatingBox
                    img_src="../graphics/anonymous.png" 
                    heading="Nazywam się Sebastian..." 
                    orange_text="" 
                    black_text="i jestem nieheteronormatywny. Swoją diałalność w harcerstwie zakończyłem po okołu ośmiu latach, na końcu jako przyboczny. Przez ten cały czas nie czułem się na tyle komfortowo, aby być sobą wśród moich najbliższych..."
                    green_text="Przeczytaj"
                    href="wyznania/1"
                />
                <WyznanieFloatingBox
                    img_src="../graphics/anonymous.png" 
                    heading="Nazywam się Sebastian..." 
                    orange_text="" 
                    black_text="i jestem nieheteronormatywny. Swoją diałalność w harcerstwie zakończyłem po okołu ośmiu latach, na końcu jako przyboczny. Przez ten cały czas nie czułem się na tyle komfortowo, aby być sobą wśród moich najbliższych..."
                    green_text="Przeczytaj"
                    href="wyznania/1"
                />
            </section>
        </section>
    )
}