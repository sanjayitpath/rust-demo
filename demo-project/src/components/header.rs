use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav class="navbar navbar-light">
            <div class="container">
                <Link<AppRoute> to={AppRoute::Home} classes="navbar-brand">
                    { "Demo" }
                </Link<AppRoute>>
            </div>
        </nav>
    }
}
