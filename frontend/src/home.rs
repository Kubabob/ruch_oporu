use yew::prelude::*;

use crate::navigator::NavBar;
use crate::survey_counter::SurveyCounter;


#[function_component(Home)]
pub fn home() -> Html {
    
    html! {
        <>
            <header>
                <h1>{ "Witamy w aplikacji" }</h1>
            </header>
            
            <NavBar />

            <section>
                <SurveyCounter />
            </section>

            <footer>
                <p>{ "Wszelkie prawa zastrzeżone © 2024" }</p>
            </footer>

            /*<div class="main-container">
                <div class="desktop-vip">
                    <div class="m-image"></div>
                    <div class="content">
                        <div class="logo">
                            <div class="i"></div>
                            <span class="t-company">{ "Ruch oporu" }</span>
                        </div>
                        <span class="t-title">{ "Gra na orientacje" }</span>
                        <span class="t-subtitle">
                            { "Czujemy się zobowiązanx do zabrania głosu i podjęcia próby zmiany organizacji poprzez stworzenie bezpiecznej przestrzeni wypowiedzi" }
                        </span>
                        <button class="j">
                            <span class="t-subscribe">{ "Dołącz do nas" }</span>
                        </button>
                    </div>
                </div>
                <div class="desktop-vip-1">
                    <button class="j-2">
                        <div class="i-3">
                            <div class="group"></div>
                        </div>
                        <span class="t-google-podcasts">{ "Google Podcasts" }</span>
                    </button>
                    <div class="flex-row-b">
                        <span class="t-detail">
                            { "Kolektyw/Stowarzyszenie  „Gra na Orientacje” to wspólnota przyjaciół, założona jako odpowiedź na doświadczenie dyskryminacji /ze względu na orientację seksualną/ w ZHR. Inicjatywa jest tworzona przez byłe i obecne osoby członkowskie i osoby instruktorskie ZHR, w  myśl wartości, w których w tej organizacji zostałyśmy wychowane – odpowiedzialności za społeczeństwo, braterstwa i wzajemnego szacunku." }
                        </span>
                        <span class="t-title-4">{ "Znajdź nas" }</span>
                        <button class="j-5">
                            <div class="i-6">
                                <div class="group-7"></div>
                            </div>
                            <span class="span-t-apple-podcasts">{ "Apple Podcasts" }</span>
                        </button>
                    </div>
                    <button class="button-j">
                        <div class="div-i">
                            <div class="group-8">
                                <div class="group-9">
                                    <div class="group-a">
                                        <div class="change"></div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <span class="span-t-spotify">{ "Spotify" }</span>
                    </button>
                </div>
                <div class="size-desktop-layout-targetuser-vip">
                    <div class="flex-column-fd">
                        <div class="m-image-b"></div>
                        <div class="m-image-c"></div>
                    </div>
                    <span class="span-t-title">{ "Poznaj nas" }</span>
                    <div class="flex-column">
                        <span class="span-t-name">{ "Marta Roszak" }</span>
                        <span class="span-t-tag">{ "Szefowa 1" }</span>
                        <span class="span-t-detail">
                            { "Jestem Marta" }
                        </span>
                        <span class="span-t-name-d">{ "Lola Klemenska" }</span>
                        <span class="span-t-tag-e">{ "Szefowa 2" }</span>
                        <span class="span-t-detail-f">
                            { "Jestem Lola" }
                        </span>
                    </div>
                </div>
                <div class="size-desktop">
                    <span class="t-title-10">{ "Ważne cytaty" }</span>
                    <div class="flex-row-fd">
                        <div class="j-11">
                            <div class="group-12">
                                <div class="vector"></div>
                            </div>
                            <span class="text"></span>
                        </div>
                        <div class="j-13">
                            <div class="group-14">
                                <div class="vector-15"></div>
                            </div>
                            <span class="text-16"></span>
                        </div>
                        <div class="j-17">
                            <div class="group-18">
                                <div class="vector-19"></div>
                            </div>
                            <span class="text-1a"></span>
                        </div>
                        <span class="t-detail-1b">
                            { "Mam nadzieję, ze nikt nie obrazi sie za czcionkę bez polskich znakow" }
                        </span>
                        <span class="t-detail-1c">{ "Lorem ipsum" }</span>
                        <span class="t-detail-1d">
                            { "Niech mi ktos powie nastepnym razem ze zabieranie sie za rzeczy bez zerowego pojecia o nich to zly pomysl" }
                        </span>
                        <span class="t-name">{ "- Jakub Bozek" }</span>
                        <span class="t-name-1e">{ "- Albert Einstein" }</span>
                        <span class="t-name-1f">{ "- Jakub Bozek" }</span>
                    </div>
                </div>
                <div class="size-desktop-20">
                    <div class="flex-row">
                        <div class="h">
                            <div class="m-image-21"></div>
                            <span class="t-tag">{ "Current Episode" }</span>
                            <span class="t-title-22">{ "Essential Foundations" }</span>
                            <span class="t-detail-23">
                                { "Explore the insights of this episode. Interested listeners are curious to learn, so share a brief episode summary here." }
                            </span>
                        </div>
                        <div class="h-24">
                            <div class="m-image-25"></div>
                            <div class="flex-column-ae">
                                <span class="t-tag-26">{ "Edition 37" }</span>
                                <span class="t-title-27">{ "Self-Care and Earth-Care" }</span>
                                <span class="t-detail-28">
                                    { "Dive into the latest happenings of this episode. Inquisitive audiences are eager to hear, so provide a brief overview here." }
                                </span>
                            </div>
                        </div>
                        <div class="h-29">
                            <div class="flex-column-abf">
                                <span class="t-tag-2a">{ "Edition 35" }</span>
                                <span class="t-title-2b">{ "Green Cleaning 101" }</span>
                                <span class="t-detail-2c">
                                    { "Find out what's new in this episode. Curious listeners are waiting to discover, so present a quick episode overview here." }
                                </span>
                            </div>
                            <div class="flex-column-ec">
                                <div class="m-image-2d"></div>
                                <span class="t-tips">{ "Automated video subtitles enabled." }</span>
                            </div>
                        </div>
                    </div>
                    <div class="h-2e">
                        <div class="m-image-2f"></div>
                        <div class="flex-column-a">
                            <span class="t-tag-30">{ "Edition 36" }</span>
                            <span class="t-title-31">{ "Community Contributions" }</span>
                            <span class="t-detail-32">
                                { "Discover the highlights of this episode. Intrigued listeners are eager to hear, so offer a concise episode summary here." }
                            </span>
                        </div>
                    </div>
                </div>
            </div>*/
        </>
    }
}