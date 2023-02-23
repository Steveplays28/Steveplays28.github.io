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

pub struct ProjectsList {
    names: Vec<String>,
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

                // Must be child of <BrowserRouter>
                <Switch<Route> render={switch} />
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
    let names: Vec<&str> = vec!["Project 1", "Project 2", "Project 3", "Project 4", "Project 5", "Project 6", "Project 7", "Project 8", "Project 9"];
    let mut index: f32 = -0.25;
    let navigator = use_navigator().unwrap();

    html! {
        <>
            <p class="bio">
                { "Hi, I'm Steve!" } <br/>
                { "I like making games in Godot and Minecraft mods." } <br/><br/>
                { "Currently maintaining 3 MC mods (+more) https://github.com/Steveplays28" }
            </p>

            <div class="projects">
            {
                names.into_iter().map(|name| {
                    index += 0.25;
                    let animation_delay = format!("animation-delay: {}s", index);
                    let navigator = navigator.clone();
                    let onclick = Callback::from(move |_| navigator.push(&Route::Projects));

                    html! {
                        <button {onclick} key={name} class="project" style={animation_delay}>{ name }</button>
                    }
                }).collect::<Html>()
            }
            </div>
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
