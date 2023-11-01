use yew::prelude::*;

#[function_component(Banner)]
pub fn banner() -> Html {
    html! {
        <div class="banner">
            <div class="container">
                <h1 class="logo-font">
                    { "Rust Web Application" }
                </h1>
                <p>{ "A language empowering everyone
    to build reliable and efficient software." }</p>
            </div>
        </div>
    }
}
