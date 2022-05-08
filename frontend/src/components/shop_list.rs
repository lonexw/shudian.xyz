use yew::prelude::*;

use crate::types::Shop;
use crate::components::ShopItem;

#[derive(Clone, Properties, PartialEq)]
pub struct ShopListProps {
    pub shops: Vec<Shop>,
    pub on_click: Callback<Shop>,
}

#[function_component(ShopList)]
pub fn shop_list(ShopListProps { shops, on_click } : &ShopListProps) -> Html {
	let on_click = on_click.clone();

    shops.iter().map(|shop| {
        let on_shop_select = {
            let on_click = on_click.clone();
            let shop = shop.clone();
            Callback::from(move |_| {
                on_click.emit(shop.clone());
            })
        };

        html! {
            <div onclick={on_shop_select}>
                <ShopItem shop={shop.clone()} />
            </div>
        }
    }).collect()
}