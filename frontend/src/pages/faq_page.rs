use yew::prelude::*;
use crate::components::navbar::Navbar;

#[function_component(FAQ)]
pub fn faq() -> Html {
    html!(
        <>
            <section class="subPage">
                <Navbar color="black" />

                <section class="oNas textBlocksWrapper">
                    <div class="textBlockWrapper">
                        <h2 class="black">{"FAQ"}</h2>
                        <p class="black">
                            {"Cieszymy się, że chcesz dowiedzieć się więcej o naszej inicjatywie i podjętych działaniach."}
                            <br/>
                            <br/>
                            {"W poniższym FAQ chcemy przedstawić nasz punkt widzenia oraz nasze spojrzenie na sprawę i szczerze odpowiedzieć na pojawiające się wątpliwości i pytania."}
                            <br/>
                            <br/>
                            {"Nie rozumiesz do końca naszej inciajtywy? Zapraszamy poniżej do przeczytania przygotowanych przez nas odpowiedzi."}
                        </p>
                    </div>

                    <div class="textBlockWrapper">
                        <h2 class="black">{"Pytania ideowe"}</h2>
                        <details>
                            <summary class="black">{"Jakie jest obecnie stanowisko ZHR w sprawie osób LGBT+? Co mówią dokumenty?"}</summary>
                            <p class="black">
                            {r#"W momencie pisania tej odpowiedzi (15.12.2024) dokumenty i instrukcje ZHR nie zawierają zapisów dotyczących stosunku ZHR do osób LGBT+, wchodzenia w związki z osobami tej samej płci oraz wynikających z tego potencjalnych konsekwencji."#}
                            <br/>
                            <br/>
                            {r#"W kwestii stosunku ZHR do osób LGBT+ panuje chaos informacyjny wywołany m.in. zeszłorocznymi zmianami w “Zasadach wychowania religijnego w ZHR”. Zmiany te dotyczyły punktu 14., głoszącego, że działalność ZHR opiera się na katolickiej nauce społecznej w kwestiach takich jak m.in. tzw. „praktyki homoseksualne”. W 2023 r. zapis ten uległ znaczącej modyfikacji. W podpunkcie, który wcześniej brzmiał:"#}
                            <br/>
                            <br/>
                            {r#"W swojej działalności ZHR opiera się na katolickiej nauce społecznej. Z uwagi na specyficzny, wychowawczy charakter organizacji, ZHR w sposób szczególny w całym szerokim spektrum katolickiej nauki społecznej podkreśla konieczność zgodności z tą nauką w kwestiach istotnych dla prawidłowego i pełnego rozwoju człowieka, takich jak:"#}
                            <br/>
                            <br/>
                            <em>{r#"[...] rola rodziny i życia intymnego, w tym m.in. istota i nienaruszalność sakramentu małżeństwa, czystość małżeńska i przedmałżeńska, małżeństwo jako sakramentalny związek mężczyzny i kobiety, wspólne mieszkanie osób niebędących małżeństwem, praktyki homoseksualne,"#}</em>
                            <br/>
                            <br/>
                            {r#"teraz natomiast, w punkcie 5. “Zasad wychowania religijnego w ZHR” [przyjętych uchwałą Rady Naczelnej nr 161/1 w dniu 24.06.2023], znajduje się ogólniejsze sformułowanie:"#}
                            <br/>
                            <br/>
                            <em>{r#"[...] rola rodziny opartej na małżeństwie, w tym m.in. nierozerwalność małżeństwa, czystość przedmałżeńska i małżeńska, małżeństwo jako (w Kościołach katolickim i prawosławnym sakramentalny) związek kobiety i mężczyzny."#}</em>
                            <br/>
                            <br/>
                            {r#"Powyższa zmiana wywołała szereg problemów praktycznych: osoby pełniące funkcje wychowawcze nie są świadome nowych zapisów lub podczas kursów korzystają z regulaminu z 2014 r., który wciąż jest łatwo dostępny w Internecie (pojawia się jako pierwszy wynik w wyszukiwarce Google)."#}
                            <br/>
                            <img src="../graphics/faq1_1.png" />
                            <br/>
                            <br/>
                            {r#"Jednocześnie pragniemy jednoznacznie podkreślić, że zarówno poprzednie, jak i obecnie obowiązujące zapisy dot. opierania się ZHR na nauczaniu społecznym kościoła katolickiego "#}
                            <b><u>{r#"nie wykluczają przynależności osób LGBT+ "#}</u></b>
                            {r#"(będących i niebędących w związkach z osobami tej samej płci) do organizacji czy pełnienia przez nie funkcji instruktorskich. Co więcej, naszym zdaniem obecne praktyki i stosunek władz ZHR wobec osób nieheteronormatywnych stoją w sprzeczności z chrześcijańskimi wartościami."#}
                            <br/>
                            <br/>
                            {r#"Obserwujemy, że wobec braku jednoznacznych zapisów i instrukcji zakazujących osobom LGBT+ (zarówno tym będącym, jak i niebędącym w związkach z osobami tej samej płci) przynależności do organizacji i grona instruktorskiego, stanowiskiem ZHR stają się opinie konkretnych instruktorek i instruktorów oraz motywowana uprzedzeniami interpretacja istniejących zapisów."#}
                            <br/>
                            <br/>
                            <b>{r#"W ramach naszego projektu chcemy stanowczo zaznaczyć, że bycie osobą homoseksualną, niezależnie od tego, czy współtworzy się związek z osobą tej samej płci, nie stoi w sprzeczności ani z Prawem i Przyrzeczeniem Harcerskim, ani z wartościami chrześcijańskimi, na które powołują się dokumenty ZHR. "#}</b>
                            {r#"Wierzymy, że prawdziwa spójność, do której wzywają nas wszystkich dokumenty naszej organizacji, polega na życiu w zgodzie z samym sobą, w tym na akceptacji własnej seksualności. Wymaganie spójności opartej na zewnętrznych oczekiwaniach, zmuszających do zaprzeczania swoim naturalnym potrzebom miłości i tworzenia rodziny uważamy za głęboko niesprawiedliwe i sprzeczne z pełnym szacunkiem dla godności człowieka. Podejście to stoi w sprzeczności z wartościami harcerskimi, które opierają się na autentyczności, wzajemnym szacunku i wspieraniu indywidualnego rozwoju każdego członka wspólnoty. "#}
                            </p>
                        </details>


                        <details>
                            <summary class="black">{"Prawo Harcerskie wymaga od nas czystości w myśli mowie i uczynkach. Jak mielibyście zachować tą czystość w związkach z osobami tej samej płci?"}</summary>
                            <p class="black">
                            {r#"Z naszych doświadczeń wynika, że jednym z najczęściej podnoszonych argumentów przeciwko obecności osób LGBT+ w ZHR jest domniemana sprzeczność między związkami osób tej samej płci a wynikającą z Prawa Harcerskiego zasadą czystości w myśli, mowie i uczynkach. Osoby, które z jednej strony podzielają ten pogląd, a z drugiej uznają wykluczanie nas wyłącznie ze względu na orientację za niesprawiedliwe, często wskazują na celibat jako jedyną drogę do zachowania czystości oraz pozostania w harcerstwie. Chcemy jasno zaznaczyć, że czystość i celibat to dwa zupełnie różne pojęcia. Wszyscy – zarówno osoby heteroseksualne, jak i homoseksualne – jesteśmy wezwani do życia w czystości. Oznacza to, że nie możemy traktować innych osób przedmiotowo ani wykorzystywać ich w jakikolwiek sposób. Czystość polega na budowaniu relacji opartych na wzajemnym szacunku, empatii i trosce o dobro drugiej osoby. Twierdzenie, że osoby nieheteronormatywne nie są zdolne do budowania takich relacji, a ich związki z definicji nie spełniają tych kryteriów, jest głęboko krzywdzące i niebezpieczne – wprowadza do języka aluzje odczłowieczające, prowadzi do pogłębiania uprzedzeń, normalizuje wykluczenia."#}
                            <br/>
                            <br/>
                            {r#"Zauważamy, że obecne, niejasne stanowisko ZHR wobec osób LGBT+ skutkuje przenoszeniem odpowiedzialności za wyznaczanie granic na przełożonych oraz na osoby bezpośrednio dotknięte tym zagadnieniem. Taki stan rzeczy nie tylko przysparza wielu niepotrzebnych cierpień, ale także nie rozwiązuje problemu, stwarzając przestrzeń dla nadużyć i podejmowania sprzecznych decyzji w różnych środowiskach. W związku z tym zachęcamy organizację do stworzenia stosownych instrukcji, szerokiego upowszechnienia istniejących już materiałów (jeśli takowe istnieją) lub do podjęcia rewizji zawartych w nich założeń."#}
                            </p>
                        </details>


                        <details>
                            <summary class="black">{"ZHR opiera się na wartościach chrześcijańskich – jak chcecie to połączyć z akceptacją dla nieheteronormatywności?"}</summary>
                            <p class="black">
                            {r#"Choć temat obowiązku wiary chrześcijańskiej wśród instruktorek i instruktorów, nieustannie od jego wprowadzenia w 2004 r., wzbudza ożywione dyskusje w wielu środowiskach, debatowanie na ten temat nie jest przedmiotem naszego projektu. Jednocześnie podkreślamy, że dyskusję na ten temat uznajemy za słuszną i uzasadnioną. Na potrzeby naszej inicjatywy i zgodnie z obowiązującymi na dzień dzisiejszy dokumentami ZHR argumentujemy w narzuconym przez nie kontekście, zakładającym że instruktorzy i instruktorki należą do jednego z kościołów chrześcijańskich.  Przypominamy jednocześnie, że ZHR deklaruje, iż nie jest organizacją konfesyjną."#}
                            <br/>
                            <br/>
                            {r#"Wśród nas znajdują się zarówno ateiści, agnostycy, osoby wierzące, które nie identyfikują się z żadną wspólnotą religijną, jak i osoby aktywnie uczestniczące w życiu wspólnot katolickich. Ci z nas, którzy pielęgnują swoją relację z Bogiem, szczególnie podkreślają wagę prymatu sumienia jako kluczowego wyznacznika życiowych decyzji. Prymat sumienia, jedno z fundamentalnych pojęć w katolicyzmie, zakłada, że każdy wierzący ma obowiązek korzystać z daru rozumu, aby rozróżniać moralnie złożone sytuacje, i że to indywidualne sumienie powinno być ostatecznym arbitrem w podejmowaniu decyzji. Sumienie jest nieodzownym elementem świadomego, etycznego życia, zarówno dla katolika, jak i dla każdego innego człowieka. Dla katolików nie oznacza to odrzucenia nauczania Kościoła (które, przypominamy, nie nakazuje wykluczania osób LGBT+ ze wspólnoty), lecz uznanie, że nikt nie może być zmuszony do działania, które w zgodzie ze swoim sumieniem uznaje za moralnie niewłaściwe. Przyjaciele osób LGBT+, postępując zgodnie ze swoim sumieniem, nie mogą odwracać wzroku od ich cierpienia ani milczeniem lub zaniechaniem przyczyniać się do niesprawiedliwości popełnianych w imię błędnie pojmowanego interesu ZHR. Osoby LGBT+ mają prawo do budowania swoich rodzin, życia w zgodzie ze swoją naturą i odrzucenia fałszu – ponieważ według ich sumienia rezygnacja z tych wartości byłaby równoznaczna z czynieniem zła."#}
                            <br/>
                            <br/>
                            {r#"Wracając do wartości chrześcijańskich – wiemy, że są wśród was osoby, wedle których do podstawowych wartości chrześcijańskich należy wartość małżeństwa jako nierozerwalnego związku kobiety i mężczyzny. Nie podważamy tej wartości. Zabiegamy jedynie o dostrzeżenie wartości w naszych relacjach oraz nieodrzucanie ich jako moralnie złych czy istniejących wbrew Bożej woli. Znamy nauczanie kościoła katolickiego w tym zakresie. Osoby zainteresowane odsyłamy do Katechizmu Kościoła Katolickiego, punkty od 2357 do 2359."#}
                            <br/>
                            <br/>
                            {r#"Jednocześnie pragniemy przypomnieć, że Kościół katolicki na przestrzeni dziejów wielokrotnie rewidował swoje stanowiska i nauczanie, uznając błędy przeszłości i przepraszając za wyrządzone krzywdy. Przykładami takich zmian są chociażby przyzwolenie na niewolnictwo i podboje kolonialne uzasadniane niegdyś papieskimi bullami, przypisywanie Żydom zbiorowej odpowiedzialności za śmierć Jezusa czy wykluczanie osób, które zmarły w wyniku samobójstwa, poprzez odebranie im możliwości pochówku na cmentarzach. Przywołując te przykłady, podkreślamy fundamentalne znaczenie indywidualnego sumienia, które pozwala każdemu człowiekowi rozróżniać dobro i zło, nawet jeśli stoi to w sprzeczności z obowiązującym w danym czasie nauczaniem Kościoła."#}
                            <br/>
                            <br/>
                            {r#"Serdecznie zachęcamy do zapoznania się z poniższymi artykułami:"#}
                            <br/>
                            <br/>
                            <a href="https://magazynkontakt.pl/w-sprawie-osob-homoseksualnych-kosciol-zapomina-o-wlasnym-nauczaniu/">{"https://magazynkontakt.pl/w-sprawie-osob-homoseksualnych-kosciol-zapomina-o-wlasnym-nauczaniu/"}</a>
                            <br/>
                            <br/>
                            <a href="https://magazynkontakt.pl/z-nich-zas-najwieksza-jest-milosc/">{"https://magazynkontakt.pl/z-nich-zas-najwieksza-jest-milosc/"}</a>
                            <br/>
                            <br/>
                            <a href="https://magazynkontakt.pl/pendergast-my-tez-jestesmy-kosciolem/">{"https://magazynkontakt.pl/pendergast-my-tez-jestesmy-kosciolem/"}</a>
                            <br/>
                            <br/>
                            <a href="https://magazynkontakt.pl/milosc-twoja-byla-mi-rozkoszniejsza-niz-milosc-kobiety-biblijne-inspiracje-dla-osob-lgbt/">{"https://magazynkontakt.pl/milosc-twoja-byla-mi-rozkoszniejsza-niz-milosc-kobiety-biblijne-inspiracje-dla-osob-lgbt/"}</a>
                            <br/>
                            <br/>
                            <a href="https://magazynkontakt.pl/gawrys-czysta-milosc-homoseksualna/">{"https://magazynkontakt.pl/gawrys-czysta-milosc-homoseksualna/"}</a>
                            </p>
                        </details>


                        <details>
                            <summary class="black">{"W ZHR wychowujemy przez przykład własny. Jak instruktor/instruktorka LGBT+ mają dawać przykład?"}</summary>
                            <p class="black">
                            {r#"Przykład własny to jeden z fundamentów metody harcerskiej. Poprzez nasze działania staramy się ukazywać harcerkom, harcerzom, a także samym sobie, że wartości, które przekazujemy, nie są jedynie pustymi słowami zapisanymi na papierze, lecz stanowią fundament codziennego życia. Prawdomówność, rzetelność, odpowiedzialność za rodzinę i wspólnotę, życzliwość, optymizm, gotowość do niesienia pomocy oraz rozwijanie duchowości to postawy uniwersalne, na które monopolu nie ma żadna grupa czy jednostka. Osoby LGBT+ aktywnie działają na rzecz naszego kraju i wspólnoty poprzez wykonywaną pracę, obywatelskie zaangażowanie w życie społeczne, zakładanie rodzin, rozwój swojej duchowości. "#}
                            <br/>
                            <br/>
                            {r#"Szczerze wierzymy, że związki i rodziny, które tworzymy, są przykładem wzajemnego wsparcia oraz wzorem relacji pełnych miłości i bezpieczeństwa. "#}
                            <br/>
                            <br/>
                            {r#"Według danych z raportu “Sytuacja społeczna osób LGBTA w Polsce” za lata 2019-2020:"#}
                            <br/>
                            <ul>
                                <li>{"trzy czwarte nastoletnich osób LGBT+ czuje się osamotnione (74,29%), "}</li>
                                <li>{"trzy czwarte młodzieży szkolnej i 60% osób studiujących deklaruje, że czasem miewa myśli samobójcze, "}</li>
                                <li>{"ponad jedna czwarta młodzieży szkolnej i jedna piąta studiującej ma silne objawy depresji, a łącznie aż 55% uczniów ma umiarkowanie nasilone lub silne objawy depresji,"}</li>
                                <li>{"brak objawów depresji deklaruje poniżej 10% osób uczących się lub studiujących."}</li>
                            </ul>
                            <br/>
                            {r#"Dlatego z pełnym przekonaniem podkreślamy nieocenioną wartość obecności osób LGBT+ w gronie instruktorskim i przykładu własnego oraz aktywnie dążymy do wprowadzenia zmian, które zapewnią bezpieczną przestrzeń dla dzieci i młodzieży LGBT+ w harcerstwie. Równocześnie apelujemy o zachowanie zdrowego dystansu wobec nadmiernego akcentowania wpływu przykładu osobistego w kontekście orientacji seksualnej. Homoseksualność nie jest czymś, czym można się „zarazić”. Drużynowa będąca w związku z inną kobietą nie wpłynie na orientację swoich harcerek, tak samo jak drużynowe pozostające w związkach z mężczyznami nie wpłynęły na nasze. Co więcej, harcerki i harcerze obserwujący swoich drużynowych, którzy tworzą szczęśliwe związki z osobami tej samej płci, mogą poczuć się pewniej, bezpieczniej, mniej samotnie. Jest to szczególnie istotne w kontekście dobrostanu oraz trudnej sytuacji dzieci i młodzieży LGBT+ w naszym kraju."#}
                            </p>
                        </details>


                        <details>
                            <summary class="black">{"Takich spraw jest bardzo mało, to są tylko pojedyncze przypadki. Po co tracić na to tyle czasu? Mamy ważniejsze sprawy."}</summary>
                            <p class="black">
                            {r#"Według badań przeprowadzonych w 2016 r. w dziewięciu europejskich krajach przez ośrodek badania opinii publicznej Dalia, odsetek osób LGBT w Polsce wyniósł 4,9 proc. Przyjmując taką statystykę za punkt odniesienia, możemy z łatwością wyliczyć, że:"#}
                            <ul>
                                <li>{"w dwudziestoosobowej drużynie jest przynajmniej jedna nieheteronormatywna harcerka lub harcerz,"}</li>
                                <li>{"w liczącej 100 instruktorek chorągwi około 5 instruktorek jest nieheteronormatywnych,"}</li>
                                <li>{"w liczącym około 20 tysięcy harcerek i harcerzy ZHR, około tysiąc z nich to osoby nieheteronormatywne. "}</li>
                            </ul>
                            {r#"Jest nas zatem więcej niż wszystkich rudych i zielonookich razem wziętych (łącznie między 2% a 4%). To, ile osób LGBT+ jest w ZHR, nie ma dla nas jednak najmniejszego znaczenia. "#}
                            <br/>
                            <u>{"Jeśli nasze doświadczenia miały być udziałem tylko jednej osoby w całej organizacji, to sens naszych działań pozostałby niezmienny. Każdy człowiek ma wartość, za każdego warto walczyć i nikt nie zasługuje na tak przejmujące poczucie samotności i odrzucenia. "}</u>
                            {r#"Ktoś z pewnością zarzuci nam przeszacowanie liczby osób nieheteronormatywnych ponieważ “w naszym środowisku nikogo takiego nie ma, może jedna osoba…”. Osobom, które zgadzają się z tym stwierdzeniem, pragniemy wyjaśnić – brak waszej wiedzy świadczy w znacznej mierze nie o braku osób nieheteronormatywnych w waszych środowiskach, a o atmosferze wrogości i potencjalnych konsekwencjach, których obawiają się te osoby. "#}
                            <br/>
                            <br/>
                            {r#"Według badania przeprowadzonego wśród instruktorek i instruktorów na potrzeby tworzonej w 2022 r. strategii ZHR jednym z głównych motywatorów do pozostania w organizacji są relacje. Dla wielu z nas, wychowywanych od dziecka w ZHR, stanowi on główną rówieśniczą siatkę społeczną oraz jeden z trzonów tożsamości. W takim wypadku nie powinno dziwić, że sugerowanie czy nakłanianie do “dobrowolnego odejścia” wiążę się z ogromnym dyskomfortem i strachem. To okropne, niszczące relacje i zaufanie doświadczenie. "#}
                            <br/>
                            <br/>
                            {r#"Umniejszanie uczuciom osób wykluczonych z organizacji jest zatem nie tylko okrutne, ale stanowi kolejny poziom wykluczenia, polegający na zaprzeczeniu, że osoby LGBT+ kiedykolwiek były “prawdziwymi” harcerkami i harcerzami należącymi do wspólnoty. Tworzenie figury “innego” służy projektowaniu nań lęków i uprzedzeń, np. łączenia homoseksualności z pedofilią czy rozwiązłością seksualną, a co za tym idzie, tłumieniu wyrzutów sumienia wynikających z dyskryminowania tych osób. "#}
                            <br/>
                            <br/>
                            {r#"Zabieramy głos w ramach tego projektu i dzielimy się swoimi doświadczeniami, by pokazać, że nie jesteśmy “innym”, a waszymi koleżankami i kolegami, osobami działającymi razem z Wami w myśl ideałów harcerskich – postawy bezinteresownej służby, braterstwa ponad podziałami, nieustannej pracy nad sobą."#}
                            </p>
                        </details>


                        <details>
                            <summary class="black">{"Czemu niszczycie dobre imię ZHR?"}</summary>
                            <p class="black">
                            {r#"Naszą intencją nie jest niszczenie dobrego imienia ZHR. Działamy na rzecz organizacji                   i należących do niej osób, jednakże ze względu na brak możliwości działania od wewnątrz zdecydowałyśmy się na powołanie niezależnego stowarzyszenia. To właśnie w ramach ZHR mogłyśmy się rozwijać, pomagać innym, nawiązywać przyjaźnie. Nasze działanie jest wobec tego wyrazem troski i zaangażowania w ruch bliski naszym sercom. To prawda – naszym zdaniem ZHR niedomaga w obszarze stosunku do osób LGBT+, jednak nie skreśla to fantastycznego dorobku tej organizacji, którą miałyśmy zaszczyt i przyjemność współtworzyć. "#}
                            <br/>
                            <br/>
                            {r#"Nauczono nas, że zepsute rzeczy się naprawia, a nie wyrzuca w kąt. Nauczono nas, że o trudnych sprawach należy rozmawiać. Wewnątrz organizacji, na poziomie rozmów w chorągwiach oraz podczas konferencji czy kuźnic instruktorskich ta możliwość jest niestety znacznie ograniczona ze względu na prowadzenie rozmów “pod tezę”. Przedstawienie zdania przeciwnego do przełożonego wiąże się z nieprzyjemnościami, ryzykiem podważenia kompetencji do pełnienia funkcji wychowawczej lub temat bywa po prostu ucinany. Nie mając przestrzeni na działanie w ramach organizacji, część z nas zdecydowała się działać na rzecz wprowadzania w niej zmian już z zewnątrz. "#}
                            <br/>
                            <br/>
                            {r#"Mamy świadomość, że nasze inicjatywy mogą przyciągnąć uwagę mediów – nie jest to jednak naszym głównym celem. Jednocześnie chcemy dotrzeć do szerszego niż krąg naszych znajomych grona odbiorców, w związku z czym korzystamy z dostępnych kanałów komunikacji, takich jak media społecznościowe czy strona internetowa. Obserwujemy, że wiedza na temat sytuacji osób LGBT+ w ZHR zarówno wśród instruktorek, instruktorów, kadry wspomagającej, rodziców i sympatyków ZHR jest bardzo niska, dlatego efekty naszych działań będą ogólnodostępne w Internecie, tak aby każdy mógł zapoznać się z naszym projektem."#}
                            </p>
                            </details>

                        <details>
                            <summary class="black">{"Jeśli nie jesteście już w ZHR to po co dalej w tym siedzicie? Nie lepiej zająć się czymś innym, a nie walczyć o coś co ma małą szansę powodzenia?"}</summary>
                            <p class="black">
                            {r#"Osoby działające w ramach “Gry na Orientację” były i są częścią ZHR. To w tej organizacji się wychowywaliśmy i towarzyszyliśmy w dorastaniu innym. ZHR to nasze środowisko i wspólnota, którą współtworzyłyśmy, a część z nas dalej ją współtworzy. Do działania w tej sprawie motywują nas przyjaciele, którzy dalej udzielają się wewnątrz organizacji mimo związanych z tym ogromnych kosztów osobistych. "#}
                            <br/>
                            <br/>
                            {r#"Głównym motorem naszych działań jest świadomość, że bez wprowadzenia niezbędnych zmian, kolejne pokolenia nieheteronormatywnych dzieciaków i młodych dorosłych będą musiały przechodzić dokładnie to, czego my dziś doświadczamy. Odejście czy wykluczenie niektórych z nas z organizacji niczego nie zmieni, a cisza i odwracanie się od ZHR plecami nikogo nie ochroni. Kwestia nieprzejrzystych zasad oraz domysłów dotyczących stosunku ZHR do osób LGBT+  nie znikną przecież razem z nami. Będą wracały, przysparzając cierpienia i trudności wszystkim – bez względu na położenie i opinie. "#}
                            </p>
                        </details>


                        <details>
                            <summary class="black">{"Czy ZHR może “wyrzucić” z organizacji za bycie osobą nieheteronormatywną?"}</summary>
                            <p class="black">
                            {r#"Jak pokazuje przykład naszej koleżanki, zdarzają się sytuacje w których przełożony lub przełożona, nakładając na kogoś karę organizacyjną, wyklucza oficjalnie z ZHR. Pozbywanie się osób LGBT+ odbywa się jednak przeważnie poprzez nakłanianie do “dobrowolnego” odejścia. Nakłanianie czy też sugerowanie, że nie ma dla nas miejsca w ZHRrze, że nie jesteśmy tu mile widziani, może przybierać wiele form. Przed częścią z nas nasi przełożeni postawili ultimatum – twoja relacja z chłopakiem/dziewczyną (bez względu na jej długość, zaangażowanie emocjonalne i plany na przyszłość) albo przynależność do organizacji. Samo przedstawienie nam takiego wyboru ma świadczyć o dobrowolności naszej decyzji o odejściu oraz ma być dowodem na brak dyskryminacji – nikt nas przecież nie zmusza do odejścia, a jedynie przedstawia stojące przed nami opcje. O doświadczeniach osób LGBT+ w ZHR i jak wyglądał ich proces odchodzenia z organizacji możecie przeczytać w zakładce “Nasze historie”. "#}
                            </p>
                        </details>


                        <details>
                            <summary class="black">{"Nie możecie się pogodzić z tym, że ZHR nie jest dla każdego?"}</summary>
                            <p class="black">{r#"Nie :). ZHR to my, osoby które go tworzą i decydują o jego kształcie. Czas pokaże, czy rzeczywiście ZHR nie jest dla nas. Jednak dopiero teraz, kiedy otwarcie zabieramy głos we własnej sprawie rozpoczyna się prawdziwa rozmowa na ten temat. Możliwe, że ZHR nie jest dla nas – przekonamy się o tym dzięki dialogowi, który staramy się wzmocnić poprzez ten projekt. Wypracowywanie wartościowego stanowiska na podstawie deliberacji może zaistnieć dopiero wtedy, gdy obie strony uczestniczą w nim na równych zasadach, a cały proces nie jest prowadzony pod wcześniej określoną tezę. Z naszych doświadczeń wynika, że zbyt otwarte sprzeciwianie się narracji niechętnej przynależności osób LGBT+ do ZHR prowadzi do niezadowolenia przełożonych, wyśmiewania, zarzutów o zapalczywość, sugerowania odejścia, co w efekcie uniemożliwia prowadzenie jakiegokolwiek dialogu. Wiele z nas dopiero teraz, po opuszczeniu organizacji, ma szansę mówić we własnym imieniu, bez obaw o ostracyzm czy wykluczenie, bo już się ono dokonało. "#}</p>
                        </details>


                        <details>
                            <summary class="black">{"Kwestia nieheteronormatywności w ZHR, to temat dotyczący wyłącznie grona instruktorskiego. Dlaczego poruszacie go na forum przez co może dotrzeć do młodszych harcerek i harcerzy albo osób niebędących w organizacji?"}</summary>
                            <p class="black">
                            {r#"Kwestia osób LGBT+ nie dotyczy wyłącznie grona instruktorskiego. My również byłyśmy nastolatkami marzącymi o przejmowaniu i zakładaniu drużyn, zastanawiającymi się nad dalszymi krokami na harcerskiej ścieżce. Stosunek organizacji do osób LGBT+, niejasności związane z tym tematem, były i dalej są czynnikami wpływającymi na decyzję o pozostaniu w organizacji i wejściu do kręgu instruktorskiego. Zasłyszane w kuluarach lub na kursach opinie głoszące, że osoby LGBT+ nie są mile widziane, bo “tak mówi kościół”, “takie są zasady” budzą sprzeciw młodzieży, która co do zasady ma trudność z przyjmowaniem argumentów z kategorii “bo tak”. Jak już zdążyłyśmy ustalić, ani kościół katolicki ani dokumenty ZHR nie stanowią podstawy do wykluczania  nas z organizacji. Dzieje się to za sprawą nieżyczliwej interpretacji oraz uprzedzeń, a nie niepodważalnych zapisów w związkowych dokumentach czy chrześcijańskiej moralności."#}
                            <br/>
                            <br/>
                            {r#"W kwestii docierania do osób z zewnątrz. Rodzice mają prawo wiedzieć, jaki stosunek do osób nieheteronormatywnych ma organizacja, do której posyłają swoje dzieci. Jak pokazuje doświadczenie, bardzo niewielu z nich ma świadomość co do różnic między organizacjami harcerskimi, ich idei i programów. Rodzice posyłają dziecko do najbliższej drużyny – tej, która przyszła na nabór w szkole lub ich parafii. O wielu rzeczach, z którymi mogą się osobiście głęboko nie zgadzać, dowiadują się znacznie później – czasami dopiero gdy ich dziecko realizuje próbę instruktorską i dowiaduje się o jakichś zapiskach z organizacyjnych dokumentów. Tak było w przypadku dużej części naszych rodziców, którzy o obowiązku chrześcijaństwa wśród instruktorów, bazowaniu na katolickiej nauce społecznej czy niechęci do osób LGBT+ dowiadywali się właśnie od nas, swoich już dorosłych dzieci. "#}
                            <br/>
                            <br/>
                            {r#"Przejrzyste zasady i procedury przyniosą korzyści nie tylko osobom działającym już w ZHR, ale również pozwolą rodzicom na świadomy wybór organizacji harcerskiej dla swoich dzieci."#}
                            </p>
                        </details>
                    </div>

                    <div class="textBlockWrapper">
                        <h2 class="black">{"Pytania podstawowe"}</h2>
                        <details>
                            <summary class="black">{"Czym jest ZHR?"}</summary>
                            <p class="black">
                            {"ZHR, czyli Związek Harcerstwa Rzeczypospolitej jest drugą największą organizacją harcerską w Polsce. Według spisu z 2022 r. liczy przynależy do niego ponad 20 tysięcy osób. "}
                            <br/>
                            {"Według opisu z zhr.pl:"}
                            <br/>
                            <em>{"Związek Harcerstwa Rzeczypospolitej to wspólnota przyjaciół – dzieci, młodzieży i dorosłych, która w oparciu o wartości chrześcijańskie, poprzez przykład własny instruktorek i instruktorów harcerskich, pracę nad sobą, służbę, przygodę oraz inne elementy metody harcerskiej wychowuje człowieka pełnego radości życia, odpowiedzialnego za Polskę i gotowego podjąć wyzwania współczesności."}</em>
                            </p>
                        </details>

                        <details>
                            <summary class="black">{"Czy wasz projekt dotyczy tylko ZHR? Co z innymi organizacjami harcerskimi?"}</summary>
                            <p class="black">
                            {r#"Stowarzyszenie “Gra na Orientację” zrzesza byłe i obecne instruktorki, instruktorów, harcerki, harcerzy oraz inne osoby należące do Związku Harcerstwa Rzeczypospolitej, a w swoich działaniach skupia się głównie na przybliżaniu perspektywy osób z tej organizacji. Jesteśmy jednak w pełni świadomi, że podobne problemy i sytuacje nie są jedynie naszym doświadczeniem. Z tego względu serdecznie zapraszamy do dzielenia się swoimi historiami i zaangażowania się w nasz projekt osoby z ZHP, “Zawiszy” i innych organizacji harcerskich."#}
                            </p>
                        </details>

                        <details>
                            <summary class="black">{"Dlaczego postanowiliście zorganizować te wszystkie działania? Dlaczego teraz?"}</summary>
                            <p class="black">
                            {"Wydarzeniem, które zainicjowało nasze działania, było nałożenie kary wykluczenia z organizacji na naszą koleżankę, phm. Julię Raczyńską. Należy jednak wspomnieć, że rozmowy na temat otwartego zabrania głosu we własnej sprawie toczyły się w naszym gronie już znacznie wcześniej. Jako harcerki i instruktorki latami brałyśmy udział w dyskusjach toczonych na konferencjach, zjazdach, kursach czy wyprawach. Z niepokojem obserwowałyśmy ich poziom oraz otaczającą je atmosferę wrogości. Otwarte mówienie o swoich doświadczeniach nie wchodziło w grę, ponieważ wiązało się z ryzykiem ostracyzmu i sugerowaniem odejścia, a nam zależało na pozostaniu w organizacji i działaniu na rzecz naszych jednostek. Z naszych doświadczeń wynika, że główną metodą działania wobec osób LGBT+ jest wzbudzanie w nas paraliżującego poczucia wstydu i wymóg milczenia. Wstyd to lęk przed odrzuceniem, przed zerwaniem więzi. My nie musimy się wstydzić, skoro już zostaliśmy odrzuceni. Korzystając z tego położenia, zabieramy głos we własnej sprawie. "}
                            </p>
                        </details>

                        <details>
                            <summary class="black">{"Jeśli wezmę udział w projekcie to kto będzie miał dostęp do moich danych?"}</summary>
                            <p class="black">
                            {r#"Wiemy, że powierzenie komuś danych osobistych jest bardzo wrażliwą sprawą, zwłaszcza tych związanych z tożsamością czy wartościami. Dlatego stawiamy bardzo duży nacisk na to, aby w tym procesie otoczyć największą troską wszystkie osoby, które będą tego potrzebować. Dostęp do danych osób, które biorą udział w naszych projektach, mają wyłącznie oficjalne członkinie oraz członkowie stowarzyszenia. Możesz przeczytać o nas w zakładce “Nasz Zespół”."#}
                            </p>
                        </details>
                    </div>
                </section>

                
            </section>
        </>
    )
} 

// <details>
// <summary class="black">{""}</summary>
// <p class="black">
// {""}
// </p>
// </details>