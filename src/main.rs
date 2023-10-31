use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/projects")]
    Projects,
    #[at("/contact")]
    Contact,
}

pub struct Project<'a, 'b, 'c> {
    name: &'a str,
    link: &'b str,
    image: &'c str,
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Projects => html! {
            <Projects />
        },
        Route::Contact => html! {
            <Contact />
        },
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
                <div class="nav-bar-element">
                    <Link<Route> to={Route::Home}>
                        <p>{ "Home" }</p>
                    </Link<Route>>
                </div>
                <div class="nav-bar-element">
                    <Link<Route> to={Route::Contact}>
                        <p>{ "Contact" }</p>
                    </Link<Route>>
                </div>

                <div class="nav-bar-element socials">
                    <a href="https://github.com/Steveplays28/portfolio-website-rs" target="_blank" rel="noopener noreferrer">
                        <img src="/media/github-mark-white.svg" />
                    </a>
                    <a href="https://discord.gg/KbWxgGg" target="_blank" rel="noopener noreferrer">
                        <img src="/media/discord-mark-white.svg" />
                    </a>
                    <a href="https://www.youtube.com/steveplays28" target="_blank" rel="noopener noreferrer">
                        <img src="/media/yt_icon_mono_dark.png" />
                    </a>
                    <a href="https://www.patreon.com/steveplays28" target="_blank" rel="noopener noreferrer">
                        <img src="/media/Digital-Patreon-Logo_White.png" />
                    </a>
                    <a href="https://ko-fi.com/steveplays" target="_blank" rel="noopener noreferrer">
                        <img src="/media/kofi_s_logo_nolabel.svg" />
                    </a>
                </div>
            </div>
        </>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let projects: Vec<Project> = vec![
        Project {
            name: "Realistic Sleep",
            link: "https://modrinth.com/mod/realisticsleep",
            image: "https://github.com/Steveplays28/realisticsleep/raw/3420dc9ec26bdbdf3044f066744db67a605fb8f1/docs/media/realistic_sleep.gif",
        },
        Project {
            name: "Path Under Fence Gates",
            link: "https://modrinth.com/mod/pathunderfencegates",
            image: "https://github.com/Steveplays28/pathunderfencegates/raw/4c2c9c086a9d85f47e369fd2ef8f958e2b37650d/docs/media/all_fixes.png",
        },
        Project {
            name: "Restart Server",
            link: "https://modrinth.com/mod/restart-server",
            image: "https://github.com/Steveplays28/restart-server/raw/67bc815074a98eef71f3a49ceb8ea29206f095d1/docs/media/command_preview.png",
        },
        Project {
            name: "Forgero (contributor)",
            link: "https://modrinth.com/mod/forgero",
            image: "https://github.com/SigmundGranaas/forgero/raw/6cece12e6a9f59109470bc375c136da3e1cc3698/assets/banner.png",
        }
    ];
    let mut index: f32 = -0.25;

    html! {
        <>
            <div class="bio-container">
                <a href="https://github.com/Steveplays28" target="_blank" rel="noopener noreferrer" class="profile-picture-container">
                    <img src="/media/steve_profile_picture.png" alt="Profile picture" class="profile-picture" />
                    <img src="/media/github-mark-white-background.svg" alt="GitHub out link" class="github" />
                </a>

                <p class="bio">
                    { "Hi, I'm Steve!" } <br/>
                    { "I like making games in Godot, as well as making Minecraft mods." } <br/><br/>
                    { "I'm currently maintaining 5 Minecraft mods, a Factorio mod, and a Satisfactory mod. I'm also contributing to other Minecraft projects, such as " } <a href="https://modrinth.com/mod/forgero" target="_blank" rel="noopener noreferrer"> { "Forgero" } </a> { " and "} <a href="https://modrinth.com/mod/distanthorizons" target="_blank" rel="noopener noreferrer"> { "Distant Horizons" } </a> { "." }
                </p>
            </div>

            <div class="projects">
            {
                projects.into_iter().map(|project| {
                    index += 0.25;
                    let style = format!("animation-delay: {seconds}s; background-image: url({image});", seconds = index, image = project.image);

                    html! {
                        <a href={project.link} target="_blank" rel="noopener noreferrer" key={project.name} class="project" style={style}>
                            <p class="project-title">{ project.name }</p>
                        </a>
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

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <>
            <div class="contact-text">
                <p class="title">{ "Reach out to me" }</p> <br/>
                <p class="subtitle">{ "I respond most quickly on Discord, GitHub and Patreon, but I try my best to respond on all platforms." }</p>
            </div>

            <div class="socials contact-socials">
                <a href="https://www.patreon.com/steveplays28" target="_blank" rel="noopener noreferrer">
                    <img src="/media/Digital-Patreon-Logo_White.png" />
                </a>
                <a href="https://ko-fi.com/steveplays" target="_blank" rel="noopener noreferrer">
                    <img src="/media/kofi_s_logo_nolabel.svg" />
                </a>
                <div class="gap"></div>

                <a href="https://github.com/Steveplays28/portfolio-website-rs" target="_blank" rel="noopener noreferrer">
                    <img src="/media/github-mark-white.svg" />
                </a>
                <a href="https://gitlab.com/steveplays" target="_blank" rel="noopener noreferrer">
                    <img src="/media/gitlab-logo-700.svg" />
                </a>
                <div class="gap"></div>

                <a href="https://steveplays.itch.io" target="_blank" rel="noopener noreferrer">
                    <img src="/media/itchio-logo-textless-white.svg" />
                </a>
                <a href="https://modrinth.com/user/Steveplays" target="_blank" rel="noopener noreferrer">
                    <img src="/media/modrinth-logo-white.svg" />
                </a>
                <a href="https://www.curseforge.com/members/steveplays28" target="_blank" rel="noopener noreferrer">
                    <img src="/media/curseforge-logo-white.svg" />
                </a>
                <div class="gap"></div>

                <a href="https://www.youtube.com/steveplays28" target="_blank" rel="noopener noreferrer">
                    <img src="/media/yt_icon_mono_dark.png" />
                </a>
                <a href="https://twitch.tv/steveplays28" target="_blank" rel="noopener noreferrer">
                    <img src="/media/TwitchGlitchWhite.svg" />
                </a>
                <div class="gap"></div>

                <a href="https://discord.gg/KbWxgGg" target="_blank" rel="noopener noreferrer">
                    <img src="/media/discord-mark-white.svg" />
                </a>
                <a href="https://rvlt.gg/gYXfebk5" target="_blank" rel="noopener noreferrer">
                    <img src="/media/revolt_chat_logo_white.png" />
                </a>
                <a href="https://mastodon.gamedev.place/@steveplays" target="_blank" rel="noopener noreferrer">
                    <img src="/media/mastodon-logo-white.svg" />
                </a>
                <a href="https://twitter.com/steveplays28" target="_blank" rel="noopener noreferrer">
                    <img src="/media/twitter-logo-white.svg" />
                </a>
            </div>
        </>
    }
}
