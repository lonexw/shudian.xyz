use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Home};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    // #[at("/shop/:id")]
    // ShopDetail { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        // Route::ShopDetail { id } => html! { <ShopDetail shop_id={id} />	},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}