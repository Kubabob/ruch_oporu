use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct FaqItem {
    question: String,
    answer: String,
}

#[function_component(FaqComponent)]
fn faq_component(props: &FaqItem) -> Html {
    let expanded = use_state(|| false);

    let toggle = {
        let expanded = expanded.clone();
        Callback::from(move |_| {
            expanded.set(!*expanded);
        })
    };

    html! {
        <div class="faq-item">
            <button class="faq-question" onclick={toggle}>
                <div class="plus-icon">
                    <svg class={classes!(expanded.then(|| "rotated"))}
                        viewBox="0 0 24 24"
                        width="24"
                        height="24"
                    >
                        <path fill="currentColor" d="M19 13H13V19H11V13H5V11H11V5H13V11H19V13Z"/>
                    </svg>
                </div>
                <h3>{&props.question}</h3>
            </button>

            {if *expanded {
                html! {
                    <div class="faq-answer">
                        <p>{&props.answer}</p>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}

#[function_component(FaqSection)]
pub fn faq_section() -> Html {
    let faqs1 = vec![
        FaqItem {
            question:
            "Jakie jest obecnie stanowisko ZHR w sprawie osób LGBT+? Co mówią dokumenty?".into(),

            answer:
            "Na moment pisania tej odpowiedzi (15.12.2024) w dokumentach i instrukcjach ZHR nie ma zapisów dotyczących stosunku ZHR do osób LGBT+, wchodzenia w związki z osobami tej samej płci oraz wynikających z tego tytułu potencjalnych konsekwencji.
            W kwestii stosunku ZHR do osób LGBT+ panuje chaos informacyjny wywołany m.in. zeszłorocznymi zmianami w Zasadach wychowania religijnego. Zmiany te dotyczyły punktu 14., głoszącego, że działalność ZHR opiera się na katolickiej nauce społecznej w kwestiach takich jak m.in. tzw. „praktyki homoseksualne”. W 2023 r. zapis ten uległ znaczącej modyfikacji. W podpunkcie, który wcześniej brzmiał:
            „W swojej działalności ZHR opiera się na katolickiej nauce społecznej. Z uwagi na specyficzny, wychowawczy charakter organizacji, ZHR w sposób szczególny w całym szerokim spektrum katolickiej nauki społecznej podkreśla konieczność zgodności z tą nauką w kwestiach istotnych dla prawidłowego i pełnego rozwoju człowieka, takich jak:
            [...] rola rodziny i życia intymnego, w tym m.in. istota i nienaruszalność sakramentu małżeństwa, czystość małżeńska i przedmałżeńska, małżeństwo jako sakramentalny związek mężczyzny i kobiety, wspólne mieszkanie osób niebędących małżeństwem, praktyki homoseksualne”,
            teraz, w punkcie 5 Zasad wychowania religijnego w ZHR [przyjętych uchwałą Rady Naczelnej nr 161/1 w dniu 24.06.2023], znajduje się bardziej ogólne sformułowanie:
            „rola rodziny opartej na małżeństwie, w tym m.in. nierozerwalność małżeństwa, czystość przedmałżeńska i małżeńska, małżeństwo jako (w Kościołach katolickim i prawosławnym sakramentalny) związek kobiety i mężczyzny”.
            Powyższa zmiana wywołała szereg problemów praktycznych: osoby pełniące funkcje wychowawcze nie są świadome nowych zapisów lub podczas kursów korzystają ze starego regulaminu z 2014 r., który wciąż jest łatwo dostępny w Internecie (pojawia się jako pierwszy wynik w wyszukiwarce Google).

            Jednocześnie, pragniemy jednoznacznie podkreślić, że zarówno poprzednie, jak i obecnie obowiązujące zapisy dot.opierania się ZHR na nauczaniu społecznym kościoła katolickiego, nie wykluczają przynależności osób LGBT+ (będących i niebędących w związkach z osobami tej samej płci) do organizacji czy pełnienia przez nie funkcji instruktorskich. Co więcej, naszym zdaniem, obecne praktyki i stosunek władz ZHR wobec osób nieheteronormatywnych, stoją w sprzeczności z chrześcijańskimi wartościami.
            W efekcie braku jednoznacznych zapisów i instrukcji, zakazujących osobom LGBT+ (zarówno tym będącym jak i nie będącym w związkach z osobami tej samej płci) przynależności do organizacji i grona instruktorskiego, stanowiskiem ZHR stają się opinie konkretnych instruktorek i instruktorów oraz motywowana uprzedzeniami interpretacja istniejących zapisów.
            W ramach naszego projektu chcemy stanowczo zaznaczyć, że bycie osobą homoseksualną, niezależnie od tego, czy tworzy się związek z osobą tej samej płci, nie stoi w sprzeczności ani z Prawem i Przyrzeczeniem Harcerskim, ani z wartościami chrześcijańskimi, na które powołują się dokumenty ZHR. Wierzymy, że prawdziwa spójność, do której wzywają nas wszystkich dokumenty naszej organizacji, polega na życiu w zgodzie z samym sobą, w tym na akceptacji własnej seksualności. Wymaganie spójności opartej na zewnętrznych oczekiwaniach, które zmuszają do zaprzeczania swoim naturalnym potrzebom miłości i tworzenia rodziny, uważamy za głęboko niesprawiedliwe i sprzeczne z pełnym szacunkiem dla godności człowieka. Takie podejście stoi w sprzeczności z wartościami harcerskimi, które opierają się na autentyczności, wzajemnym szacunku i wspieraniu indywidualnego rozwoju każdego członka wspólnoty.
            Z naszych doświadczeń wynika, że jednym z najczęściej podnoszonych argumentów przeciwko obecności osób LGBT+ w ZHR jest domniemana sprzeczność między związkami osób tej samej płci a wynikającą z Prawa Harcerskiego zasadą czystości w myśli, mowie i uczynkach. Osoby, które podzielają ten pogląd, lecz jednocześnie uznają wykluczanie nas wyłącznie ze względu na orientację za niesprawiedliwe, często wskazują na celibat jako jedyną drogę do zachowania czystości oraz pozostania w harcerstwie. Chcemy jasno zaznaczyć, że czystość i celibat to dwa zupełnie różne pojęcia. Wszyscy – zarówno osoby heteroseksualne, jak i homoseksualne – jesteśmy wezwani do życia w czystości. Oznacza to, że nie możemy traktować innych osób przedmiotowo ani wykorzystywać ich w jakikolwiek sposób. Czystość polega na budowaniu relacji opartych na wzajemnym szacunku, empatii i trosce o dobro drugiej osoby. Twierdzenie, że osoby nieheteronormatywne nie są zdolne do budowania takich relacji, a ich związki z definicji nie spełniają tych kryteriów, jest głęboko krzywdzące i niebezpieczne ponieważ zaprzecza naszemu człowieczeństwu.
            Zauważamy, że obecne, niejasne stanowisko ZHR wobec osób LGBT+, skutkuje przenoszeniem odpowiedzialności za wyznaczanie granic na przełożonych oraz na osoby bezpośrednio dotknięte tym zagadnieniem. Taki stan rzeczy nie tylko przysparza wielu niepotrzebnych cierpień, ale także nie rozwiązuje problemu, stwarzając przestrzeń dla nadużyć i podejmowania sprzecznych decyzji w różnych chorągwiach. W związku z tym zachęcamy organizację do stworzenia stosownych instrukcji, szerokiego upowszechnienia istniejących już materiałów (jeśli takowe istnieją) lub do podjęcia rewizji zawartych w nich założeń.
            ".into(),
        },
        FaqItem {
            question: "ZHR opiera się na wartościach chrześcijańskich - jak chcecie to połączyć z akceptacją dla nieheteronormatywności?".into(),
            answer: "Choć temat obowiązku wiary chrześcijańskiej wśród instruktorek i instruktorów, nieustannie od jego wprowadzenia w 2004 r., wzbudza ożywione dyskusje w wielu środowiskach, debatowanie na ten temat nie jest przedmiotem tego projektu. Jednocześnie podkreślamy, że dyskusję na ten temat uznajemy za słuszną i uzasadnioną. Na potrzeby tego projektu i zgodnie z obowiązującymi na dzień dzisiejszy dokumentami ZHR, argumentujemy w narzuconym przez nie kontekście, zakładającym że instruktorzy i instruktorki należą do jednego z obrządków chrześcijańskich.  Przypominamy jednocześnie, że ZHR deklaruje, że nie jest organizacją konfesyjną.
            Jako osoby zaangażowane w działalność ZHR doskonale rozumiemy, jak istotnym elementem harcerskiej metody są wartości chrześcijańskie. Wśród nas znajdują się zarówno ateiści, agnostycy, osoby wierzące, które nie identyfikują się z żadną wspólnotą religijną, jak i osoby aktywnie uczestniczące w życiu wspólnot katolickich. Ci z nas, którzy pielęgnują swoją relację z Bogiem, szczególnie podkreślają wagę prymatu sumienia jako kluczowego wyznacznika życiowych decyzji. Prymat sumienia, jedno z fundamentalnych pojęć w katolicyzmie, zakłada, że każdy wierzący ma obowiązek korzystać z daru rozumu, aby rozróżniać moralnie złożone sytuacje, i że to indywidualne sumienie powinno być ostatecznym arbitrem w podejmowaniu decyzji. Sumienie jest nieodzownym elementem świadomego, etycznego życia, zarówno dla katolików, jak i dla każdego człowieka. Dla katolików nie oznacza to odrzucenia nauczania Kościoła (które, przypominamy, nie nakazuje wykluczania osób LGBT+ ze wspólnoty), lecz uznanie, że nikt nie może być zmuszony do działania, które w zgodzie ze swoim sumieniem uznaje za moralnie niewłaściwe. Przyjaciele osób LGBT+, postępując zgodnie ze swoim sumieniem, nie mogą odwracać wzroku od ich cierpienia ani milczeniem lub zaniechaniem przyczyniać się do niesprawiedliwości popełnianych w imię błędnie pojmowanego interesu ZHR. Osoby LGBT+ mają prawo do budowania swoich rodzin, życia w zgodzie ze swoją naturą i odrzucenia fałszu – ponieważ według ich sumienia rezygnacja z tych wartości byłaby równoznaczna z czynieniem zła.

            Wracając do wartości chrześcijańskich - wiemy, że są wśród was osoby, wedle których do podstawowych wartości chrześcijańskich należy wartość małżeństwa jako nierozerwalnego związku kobiety i mężczyzny. Nie podważamy tej wartości. Zabiegamy jedynie o dostrzeżenie wartości w naszych relacjach, nie odrzucanie ich jako moralnie złych czy istniejących wbrew Bożej woli. Znamy nauczanie kościoła katolickiego w tej materii. Osoby zainteresowane odsyłamy do Katechizmu Kościoła Katolickiego, punkty od 2357 do 2359.

            Pragniemy przypomnieć, że Kościół katolicki na przestrzeni dziejów wielokrotnie rewidował swoje stanowiska i nauczanie, uznając błędy przeszłości i przepraszając za wyrządzone krzywdy. Przykładami takich zmian są dawne przyzwolenie na niewolnictwo i podboje kolonialne, uzasadniane niegdyś papieskimi bullami, przypisywanie Żydom zbiorowej odpowiedzialności za śmierć Jezusa czy wykluczanie osób, które zmarły w wyniku samobójstwa, z możliwości pochówku na cmentarzach. Przywołując te przykłady, podkreślamy fundamentalne znaczenie indywidualnego sumienia, które pozwala każdemu człowiekowi rozróżniać dobro i zło, nawet jeśli stoi to w sprzeczności z obowiązującym w danym czasie nauczaniem Kościoła.

            Serdecznie zachęcamy do zapoznania się również z tymi artykułami:
            https://magazynkontakt.pl/w-sprawie-osob-homoseksualnych-kosciol-zapomina-o-wlasnym-nauczaniu/
            https://magazynkontakt.pl/z-nich-zas-najwieksza-jest-milosc/
            https://magazynkontakt.pl/pendergast-my-tez-jestesmy-kosciolem/
            https://magazynkontakt.pl/milosc-twoja-byla-mi-rozkoszniejsza-niz-milosc-kobiety-biblijne-inspiracje-dla-osob-lgbt/
            https://magazynkontakt.pl/gawrys-czysta-milosc-homoseksualna/
            ".into(),
        },
        FaqItem {
            question:
            "W ZHR wychowujemy przez przykład własny. Jak instruktor/instruktorka LGBT+ mają dawać przykład?".into(),
            answer:
            "Przykład własny to jeden z fundamentów metody harcerskiej. Poprzez nasze działania staramy się ukazywać harcerkom, harcerzom, a także samym sobie, że wartości, które przekazujemy, nie są jedynie pustymi słowami zapisanymi na papierze, lecz stanowią fundament codziennego życia. Prawdomówność, rzetelność, odpowiedzialność za rodzinę i wspólnotę, życzliwość, optymizm, gotowość do niesienia pomocy oraz rozwijanie duchowości to postawy uniwersalne, do których nie ma monopolu żadna grupa czy jednostka. Osoby LGBT+ aktywnie działają na rzecz naszego kraju i wspólnoty pracując, angażując się w życie społeczne jako obywatele, zakładając rodziny, rozwijając swoją duchowość i budując relację z Bogiem. Szczerze wierzymy, że związki i rodziny, które tworzymy, są przykładem wzajemnego wsparcia, wzorem relacji pełnych miłości i bezpieczeństwa.

            Według danych z raportu “Sytuacja społeczna osób LGBTA w Polsce” za lata 2019-2020:
            trzy czwarte nastoletnich osób LGBT+ czuje się osamotnione (74,29%),
            trzy czwarte młodzieży szkolnej i 60% osób studiujących deklaruje, że czasem miewa myśli samobójcze,
            ponad jedna czwarta młodzieży szkolnej i jedna piąta studiującej ma silne objawy depresji, a łącznie aż 55% uczniów ma umiarkowanie nasilone lub silne objawy depresji.
            brak objawów depresji deklaruje poniżej 10% osób uczących się lub studiujących,

            Dlatego z pełnym przekonaniem podkreślamy nieocenioną wartość obecności osób LGBT+ w gronie instruktorskim i przykładu własnego oraz aktywnie dążymy do wprowadzenia zmian, które zapewnią bezpieczną przestrzeń dla dzieci i młodzieży LGBT+ w harcerstwie. Równocześnie apelujemy o zachowanie zdrowego dystansu wobec nadmiernego akcentowania wpływu osobistego przykładu w kontekście seksualności i relacji romantycznych. Homoseksualność nie jest czymś, czym można się „zarazić”. Drużynowa będąca w związku z inną kobietą nie wpłynie na orientację swoich harcerek, tak samo jak drużynowe pozostające w związkach z mężczyznami nie wpłynęły na nasze. Co więcej, harcerki i harcerze obserwujący swoich drużynowych tworzących szczęśliwe związki z osobami tej samej płci mogą poczuć się pewniej, bezpieczniej, mniej samotnie. Jest to szczególnie istotne w kontekście dobrostanu oraz trudnej sytuacji dzieci i młodzieży LGBT+ w naszym kraju.
            ".into()
        },
        FaqItem {
            question: "Takich spraw jest bardzo mało, to są tylko pojedyncze przypadki. Po co tracić na to tyle czasu? Mamy ważniejsze sprawy.".into(),
            answer:
            "Według badań przeprowadzonych w 2016 r. w dziewięciu europejskich krajach przez ośrodek badania opinii publicznej Dalia, odsetek osób LGBT w Polsce wyniósł 4,9 proc. Przyjmując taką statystykę za punkt odniesienia, możemy z łatwością wyliczyć, że:
            w dwudziestoosobowej drużynie jest przynajmniej jedna nieheteronormatywna harcerka lub harcerz,
            w liczącej 100 instruktorek chorągwi około 5 instruktorek jest nieheteronormatywnych,
            w liczącym około 20 tysięcy harcerek i harcerzy ZHR, około tysiąca z nich to osoby nieheteronormatywne.

            Jest nas zatem więcej niż wszystkich rudych i zielonookich razem wziętych (łącznie między 2% a 4%). To, ile osób LGBT+ jest w ZHR nie ma dla nas jednak najmniejszego znaczenia. Jeśli nasze doświadczenia miały być udziałem tylko jednej osoby w całej organizacji, to sens naszych działań pozostałby niezmienny. Każdy człowiek ma wartość, za każdego warto walczyć i nikt nie zasługuje na tak przejmujące poczucie samotności i odrzucenia.

            Ktoś z pewnością zarzuci nam przeszacowanie liczby osób nieheteronormatywnych ponieważ “w naszym środowisku nikogo takiego nie ma, może jedna osoba…”. Dla osób, które odnajdują się w tym zdaniu pragniemy wyjaśnić - to, że nie wiecie świadczy w znacznej mierze nie o braku osób nieheteronormatywnych w waszych środowiskach, a o atmosferze wrogości i potencjalnych konsekwencjach, których obawiają się te osoby.

            Według badania przeprowadzonego wśród instruktorek i instruktorów na potrzeby tworzonej w 2022r. strategii ZHR, głównym motywatorem pozostawania w organizacji są relacje. W takim wypadku nie powinno dziwić, z jakim dyskomfortem i strachem wiąże się sugerowanie czy nakłanianie do “dobrowolnego odejścia”. Dla wielu z nas, wychowywanych od dziecka w ZHR, stanowi on główną rówieśniczą siatkę społeczną oraz jeden z trzonów tożsamości. Umniejszanie uczuciom osób wykluczonych z organizacji jest nie tylko okrutne, ale stanowi kolejny poziom wykluczenia, polegający na zaprzeczeniu, że osoby LGBT+ kiedykolwiek były “prawdziwymi” harcerkami i harcerzami należącymi do wspólnoty. Tworzenie figury “innego” służy projektowaniu nań lęków i uprzedzeń, np. łączenia homoseksualności z pedofilią czy rozwiązłością seksualną a co za tym idzie, tłumieniu wyrzutów sumienia wynikających z dyskryminowania tych osób. Zabieramy głos w ramach tego projektu i dzielimy się swoimi doświadczeniami również w celu pokazania, że nie jesteśmy “innym”, a waszymi koleżankami i kolegami, osobami działającymi razem z Wami w myśl ideałów harcerskich - postawy bezinteresownej służby, braterstwa ponad podziałami, nieustannej pracy nad sobą.
            ".into(),
        },
        FaqItem {
            question: "Czemu niszczycie dobre imię ZHR?".into(),
            answer: "Naszą intencją nie jest niszczenie dobrego imienia ZHR. Działamy na rzecz organizacji                   i należących do niej osób, jednakże ze względu na brak możliwości działania wewnątrz niej, zdecydowałyśmy się na powołanie niezależnego stowarzyszenia.
            To właśnie w ramach ZHR mogłyśmy się rozwijać, pomagać innym, nawiązywać przyjaźnie. Nasze zaangażowanie jest wobec tego wyrazem troski i zaangażowania w ruch bliski naszym sercom. To prawda - naszym zdaniem ZHR nie domaga w obszarze stosunku do osób LGBT+, jednak nie zmazuje to fantastycznego dorobku tej organizacji, którą miałyśmy zaszczyt i przyjemność współtworzyć.

            Nauczono nas, że zepsute rzeczy się naprawia, a nie wyrzuca w kąt. Nauczono nas, że o trudnych sprawach należy rozmawiać. Wewnątrz organizacji, na poziomie rozmów w chorągwiach oraz podczas konferencji czy kuźnic instruktorskich ta możliwość jest niestety znacznie ograniczona ze względu na prowadzenie rozmów “pod tezę”. Przedstawienie zdania przeciwnego do przełożonego wiąże się z nieprzyjemnościami, podważaniem kompetencji do pełnienia funkcji wychowawczej lub temat jest ucinany. Nie mając przestrzeni na działanie w ramach organizacji, część z nas zdecydowała się działać na rzecz zmian już z zewnątrz. Mamy świadomość, że nasze działania mogą przyciągnąć uwagę mediów - nie jest to jednak naszym celem. Jednocześnie, chcemy dotrzeć do szerszego niż krąg naszych znajomych grona odbiorców, a co za tym idzie korzystamy z dostępnych kanałów komunikacji takich jak media społecznościowe czy strona internetowa. Obserwujemy, że wiedza na temat sytuacji osób LGBT+ w ZHR zarówno wśród instruktorek, instruktorów, kadry wspomagającej, rodziców i sympatyków ZHR jest bardzo niska, dlatego efekty naszych działań będą ogólnodostępne w Internecie, tak aby każdy w spokoju mógł zapoznać się z naszym projektem.
            ".into(),
        },
        FaqItem {
            question: "Jeśli nie jesteście już w ZHR to po co dalej w tym siedzicie? Nie lepiej zająć się czymś innym, a nie walczyć o coś co ma małą szansę powodzenia?".into(),
            answer: "Osoby działające w ramach “Gry na Orientację” były i są częścią ZHR. To w tej organizacji się wychowywaliśmy i towarzyszyliśmy w dorastaniu innym. ZHR to nasze środowisko i wspólnota, którą współtworzyłyśmy, a część z nas dalej ją współtworzy.
            Do działania w tej sprawie motywują nas przyjaciele, którzy dalej udzielają się wewnątrz organizacji mimo związanych z tym ogromnych osobistych kosztów. Głównym motorem naszych działań jest jednak świadomość, że bez wprowadzenia niezbędnych zmian, kolejne pokolenia nieheteronormatywnych dzieciaków i młodych dorosłych będą musiały przechodzić dokładnie przez to samo co my. Odejście czy wykluczenie niektórych z nas z organizacji niczego nie zmieni, a cisza i odwracanie się od ZHR plecami nikogo nie ochroni. Kwestia nieprzejrzystych zasad oraz domysłów dotyczących stosunku ZHR do osób LGBT+  nie znikną przecież razem z nami. Będą wracały, przysparzając cierpienia i trudności wszystkim bez względu na położenie i opinie.
            ".into(),
        },
        FaqItem {
            question: "Czy ZHR może “wyrzucić” z organizacji za bycie osobą nieheteronormatywną?".into(),
            answer:
            "Jak pokazuje przykład naszej koleżanki, zdarzają się sytuacje w których przełożony lub przełożona, nakładając na kogoś karę organizacyjną wyklucza oficjalnie z ZHR. Pozbywanie się osób LGBT+ odbywa się jednak przeważnie poprzez nakłanianie do “dobrowolnego” odejścia. Nakłanianie czy też sugerowanie, że nie ma dla nas miejsca w ZHRrze, może przybierać wiele form. Przed częścią z nas, nasi przełożeni postawili ultimatum - twoja relacja z chłopakiem/dziewczyną (bez względu na poziom jej zaawansowania) albo przynależność do organizacji. Samo przedstawienie nam takiego wyboru ma świadczyć o dobrowolności naszej decyzji o odejściu. Danie nam takiego wyboru ma być również dowodem na brak dyskryminacji - nikt nas przecież nie zmusza do odejścia a jedynie przedstawia stojące przed nami opcje. O doświadczeniach osób LGBT+ w ZHR, tym jak wyglądał ich proces odchodzenia z organizacji możecie przeczytać w zakładce “Nasze historie”.
            ".into(),
        },
        FaqItem {
            question: "Nie możecie się pogodzić z tym, że ZHR nie jest dla każdego?".into(),
            answer:
            "Nie :) ZHR to my, osoby które go tworzą i decydują o jego kształcie. Czas pokaże, czy rzeczywiście ZHR nie jest dla nas. Jednak dopiero teraz, kiedy otwarcie zabieramy głos we własnej sprawie rozpoczyna się prawdziwa rozmowa na ten temat. Możliwe, że ZHR nie jest dla nas - przekonamy się o tym dzięki dialogowi, który staramy się wzmocnić poprzez ten projekt. Dialog czy wypracowywanie stanowiska na podstawie deliberacji może zaistnieć dopiero wtedy, gdy obie strony uczestniczą w nim na równych zasadach, a cały proces nie jest prowadzony pod wcześniej określoną tezę. Z naszych doświadczeń wynika, że zbyt otwarte sprzeciwianie się narracji niechętnej przynależności osób LGBT+ do ZHR prowadzi do niezadowolenia przełożonych, wyśmiewania, zarzutów o zapalczywość, sugerowania odejścia. Wiele z nas, dopiero teraz, po opuszczeniu organizacji, ma szansę mówić we własnym imieniu, bez obaw o ostracyzm czy wykluczenie, które już się dokonało.
            ".into(),
        },
        FaqItem {
            question: "Kwestia nieheteronormatywności w ZHR, to temat dotyczący wyłącznie grona instruktorskiego. Dlaczego poruszacie go na forum przez co może dotrzeć do młodszych harcerek i harcerzy albo osób niebędących w organizacji?".into(),
            answer:
            "Kwestia osób LGBT+ nie dotyczy wyłącznie grona instruktorskiego. My również byłyśmy nastolatkami, marzącymi o przejmowaniu i zakładaniu drużyn, zastanawiającymi się nad dalszymi krokami na harcerskiej ścieżce. Stosunek organizacji do osób LGBT+, niejasności związane z tym tematem, były i dalej są czynnikami wpływającymi na decyzję o pozostaniu w organizacji i wejściu do kręgu instruktorskiego. Zasłyszane w kuluarach lub na kursach opinie głoszące że osoby LGBT+ nie są mile widziane bo “tak mówi kościół”, “takie są zasady” budzą sprzeciw młodzieży, która co do zasady ma trudność z przyjmowaniem argumentów z kategorii “bo tak”. Jak już zdążyłyśmy ustalić ani kościół katolicki ani dokumenty ZHR nie stanowią podstawy do wykluczania  nas z organizacji. Dzieje się to za sprawą nieżyczliwej interpretacji oraz uprzedzeń a nie niepodważalnych zapisów czy chrześcijańskiej moralności.

            W kwestii docierania do osób z zewnątrz. Rodzice mają prawo wiedzieć jaki stosunek do osób nieheteronormatywnych ma organizacja do której posyłają swoje dzieci. Jak pokazuje doświadczenie, bardzo niewielu z nich ma świadomość z zakresu różnic między organizacjami harcerskimi, ich idei i programów. Rodzice posyłają dziecko do najbliższej drużyny - tej która przyszła na nabór w szkole lub ich parafii. O wielu rzeczach, z którymi mogą się osobiście głęboko nie zgadzać dowiadują się dopiero znacznie później, czasami dopiero gdy ich dziecko realizując próbę instruktorską dowiaduje się o zapiskach z organizacyjnych dokumentów. Tak było w przypadku dużej części naszych rodziców, którzy o obowiązku chrześcijaństwa wśród instruktorów, bazowaniu na katolickiej nauce społecznej czy niechęci do osób LGBT+ dowiadywali się właśnie od nas, swoich już dorosłych dzieci.

            Przejrzyste zasady i procedury przyniosą korzyści nie tylko dla osób działających już w ZHR, ale również pozwolą rodzicom na świadomy wybór organizacji harcerskiej dla swoich dzieci.
            ".into(),
        },
        // Add more FAQ items
    ];

    let faqs2 = vec![
        FaqItem {
            question: "Czym jest ZHR?".into(),
            answer:
            "ZHR, czyli Związek Harcerstwa Rzeczypospolitej jest drugą największą organizacją harcerską w Polsce. Według spisu z 2022 r. liczy przynależy do niego ponad 20 tysięcy osób.
            Według opisu z zhr.pl:
            Związek Harcerstwa Rzeczypospolitej to wspólnota przyjaciół – dzieci, młodzieży i dorosłych, która w oparciu o wartości chrześcijańskie, poprzez przykład własny instruktorek i instruktorów harcerskich, pracę nad sobą, służbę, przygodę oraz inne elementy metody harcerskiej wychowuje człowieka pełnego radości życia, odpowiedzialnego za Polskę i gotowego podjąć wyzwania współczesności.
            ".into(),
        },
        FaqItem {
            question: "Czy wasz projekt dotyczy tylko ZHR? Co z innymi organizacjami harcerskimi?".into(),
            answer:
            "Stowarzyszenie “Gra na orientację” zrzesza byłe i obecne instruktorki, instruktorów, harcerki, harcerzy oraz inne osoby należące do Związku Harcerstwa Rzeczypospolitej, a w swoich działaniach skupia się głównie przybliżaniu perspektywy osób z tej organizacji. Jesteśmy jednak w pełni świadomi, że podobne problemy i sytuacje nie są jedynie naszym udziałem. Z tego względu serdecznie zapraszamy do dzielenia się swoimi doświadczeniami poprzez udział w naszych działaniach osoby z ZHP, “Zawiszy” i innych organizacji harcerskich.
            ".into(),
        },
        FaqItem {
            question: "Dlaczego postanowiliście zorganizować te wszystkie działania? Dlaczego teraz?".into(),
            answer:
            "Wydarzeniem, które zainicjowało nasze działania było nałożenie kary wykluczenia z organizacji na naszą koleżankę, phm. Julię Raczyńską. Należy jednak wspomnieć, że rozmowy na temat otwartego zabrania głosu we własnej sprawie toczyły się w naszym gronie już znacznie wcześniej. Jako harcerki i instruktorki latami brałyśmy udział w dyskusjach toczonych na konferencjach, zjazdach, kursach czy wyprawach. Z niepokojem obserwowałyśmy ich poziom oraz otaczającą je atmosferę wrogości. Otwarte mówienie o swoich doświadczeniach nie wchodziło w grę, ponieważ wiązało się z ryzykiem ostracyzmu i sugerowaniem odejścia, a nam zależało na pozostaniu w organizacji i działaniu na rzecz naszych jednostek.

            Z naszych doświadczeń wynika, że najgroźniejszą bronią w arsenale ZHR jest właśnie wzbudzanie w nas paraliżującego poczucia wstydu, który wymaga od nas milczenia. Wstyd to lęk przed odrzuceniem, przed zerwaniem więzi. My nie musimy się wstydzić, już zostaliśmy odrzuceni.
            ".into(),
        },
        FaqItem {
            question: "Jeśli wezmę udział w projekcie to kto będzie miał dostęp do moich danych?".into(),
            answer:
            "Wiemy, że powierzenie komuś swoich danych jest bardzo wrażliwą sprawą, szczególnie w tematyce ujawnienia swojej tożsamości, jeśli wciąż pozostaje się w ukryciu. Dlatego stawiamy bardzo duży nacisk na to, aby w tym procesie otoczyć wszystkie osoby, które będą tego potrzebować jak największą troską. Dostęp do danych osób, które biorą udział w naszych projektach mają wyłącznie oficjalne członkinie oraz członkowie stowarzyszenia. Dbamy o to, aby wszystkie osoby, które postanawiają nam zaufać wiedziały z kim się kontaktują i komu powierzają swoje dane, dlatego więcej o nas możesz przeczytać w zakładce “Nasz Zespół”.
            ".into(),
        },
        // Add more FAQ items
    ];

    html! {
        <section class="faq-section">
            <h2>{"Najczęściej zadawanie pytania przez harcerki/harcerzy"}</h2>
            <div class="faq-container">
                {faqs1.into_iter().map(|faq| {
                    html! { <FaqComponent question={faq.question} answer={faq.answer} /> }
                }).collect::<Html>()}
            </div>

            <h2>{"Najczęściej zadawanie pytania przez osoby nie będące w harcerstwie"}</h2>
            <div class="faq-container">
                {faqs2.into_iter().map(|faq| {
                    html! { <FaqComponent question={faq.question} answer={faq.answer} /> }
                }).collect::<Html>()}
            </div>
        </section>
    }
}
