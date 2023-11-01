use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class="container">
                <Link<AppRoute> to={AppRoute::Home} classes="logo-font">{ "Demo" }</Link<AppRoute>>
                <span class="attribution">
                    { "© 2023. An interactive learning project from" }
                    <a href="https://thinkster.io"> { "Thinkster" } </a>
                    { ". Code licensed under MIT." }
                </span>
            </div>
        </footer>
    }
}
