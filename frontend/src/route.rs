use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Home, ShopDetail};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/shop/{id}")]
    ShopDetail,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::ShopDetail => html! { <ShopDetail />	},
    }
}