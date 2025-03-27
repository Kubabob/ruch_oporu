use crate::movie_components::movie_page::MoviePage;
use yew::prelude::*;

#[function_component(Movie)]
pub fn movie() -> Html {
    html! {
        <>
            <MoviePage />
        </>
    }
}
