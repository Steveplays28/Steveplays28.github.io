use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <p class="bio">{ "Hi, I'm Steve!"} <br/>
                {"I like making games in Godot and Minecraft mods."} <br/><br/>
                {"Currently maintaining 3 MC mods (+more) https://github.com/Steveplays28"}
            </p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
