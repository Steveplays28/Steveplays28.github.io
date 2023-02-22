use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Projects => html! {
            <Projects />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <NavBar />

                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </>
    }
}

#[function_component(NavBar)]
fn nav_bar() -> Html {
    html! {
        <>
            <div class="nav-bar">
                <Link<Route> to={Route::Home}>
                    <p class="nav-bar-element">{ "Home" }</p>
                </Link<Route>>
                <Link<Route> to={Route::Projects}>
                    <p class="nav-bar-element">{ "Projects" }</p>
                </Link<Route>>
            </div>
        </>
    }
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <>
            <p class="bio">
                { "Hi, I'm Steve!" } <br/>
                { "I like making games in Godot and Minecraft mods." } <br/><br/>
                { "Currently maintaining 3 MC mods (+more) https://github.com/Steveplays28" }
            </p>
        </>
    }
}

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <>
            <h1 class="title">{"Projects"}</h1>
        </>
    }
}
