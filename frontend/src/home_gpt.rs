use yew::prelude::*;


#[function_component(HomeGPT)]
pub fn home_gpt() -> Html {
    
    html! {
        <>
            <header class="hero">
                <nav class="navbar">
                    <a href="#" class="logo">{"🌈 United in Pride"}</a>
                    <ul class="nav-links">
                        <li><a href="#about">{"About Us"}</a></li>
                        <li><a href="#community">{"Community"}</a></li>
                        <li><a href="#resources">{"Resources"}</a></li>
                    </ul>
                </nav>
                <div class="hero-content">
                    <h1>{"United in Pride, Empowered by Love"}</h1>
                    <p>{"Celebrate diversity, advocate for equality, and connect with a supportive community. Together, we can make a difference."}</p>
                    <div class="cta-buttons">
                        <a href="#join" class="btn primary">{"Join the Community"}</a>
                        <a href="#learn-more" class="btn secondary">{"Learn More About Us"}</a>
                    </div>
                </div>
            </header>
        </>
    }
}