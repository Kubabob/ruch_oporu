use yew::prelude::*;

use crate::components::navbar::Navbar;

#[function_component(MoviePage)]
pub fn movie_page() -> Html {
    html!(
        <section class="subPage">
            <Navbar color="black" />
            <section class="oNas textBlocksWrapper">
                <div class="textBlockWrapper">
                    <h2 class="black">{r#"Film “Też kiedyś byłam harcerką...”"#}</h2>
                    <div class="imageTextBlockWrapper">
                        <img class="movieImage" src="../graphics/movie/1.png" />
                        <p class="black">
                            <b>{r#"Film “Też kiedyś byłam harcerką - czyli nasze i wasze historie opowiedziane z miłością” "#}</b>{"jest nietypowym połączeniem fabuły i dokumentu."}
                            <br/>
                            <br/>
                            {"Scenariusz opiera się na wywiadach, które udało nam się przeprowadzić z nieheteronormatywnymi harcerkami i harcerzami. Opowiadają oni o swoim doświadczeniu bycia w organizacji ZHR jako osoby nieheteronormatywne. Przedstawiają swoje historie, które są trudne, zabawne, smutne i piękne. Na podstawie wszystkich zebranych wywiadów tworzymy narratora, który będzie towarzyszył nam w trakcie historii rozgrywającej się na ekranie. Sam film będzie opowiadał o miłości. Miłości między dwiema kobietami, miłości do organizacji, miłości do ludzi i do natury. Bo właśnie o prawo do tej miłości staramy się walczyć.  "}
                            <br/>
                            <br/>
                            {"Mimo, że fabuła filmu opowiada historie dwóch kobiet chcemy zaznaczyć że ten problem również dotyka mężczyzn i osób niebinarnch."}
                            <br/>
                            <br/>
                            {r#"Projekt filmu powstał w odpowiedzi na dyskryminujące zachowania wobec osób LGBTQ w środowisku ZHR. Większość osób, które tworzą ten projekt, to osoby, które albo same doświadczyły różnego typu krzywdzących zachowań albo po prostu nie zgadzają się na wykluczanie ludzi ze środowiska ze względu na ich orientację.  "#}
                            <br/>
                            <br/>
                            {r#"Film nie jest akcją mającą na celu upokorzenie lub likwidację tej organizacji. Jego celem jest  naświetlenie problemu i pokazanie, jak ważne jest ponowne przedyskutowanie podejścia, jakie ZHR promuje wobec osób nieheteronormatywnych. Równocześnie chcemy, aby stał się wsparciem i przestrzenią wolną od tabu dla osób, które nadal doświadczają takiej dyskryminacji lub doświadczyły jej w przeszłości.  "#}
                            <br/>
                            <br/>
                            {r#"Ekipa tworząca projekt składa się głównie z byłych i obecnych harcerek oraz osób, dla których ta sprawa jest ważna i warta ponownego przemyślenia. Ponieważ większość z nas należała do organizacji ZHR, ta sprawa nie jest nam obojętna. Robimy to, ponieważ głęboko wierzymy, że otwarcie się ZHR na osoby nieheteronormatywne polepszy jego funkcjonowanie. A wiadomo, że jeśli na czymś naprawdę Ci zależy, starasz się to ulepszyć."#}
                        </p>
                    </div>
                </div>
            </section>
        </section>
    )
}