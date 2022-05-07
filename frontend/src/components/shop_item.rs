use yew::prelude::*;

use crate::types::Shop;

#[derive(Properties, PartialEq)]
pub struct ShopItemProps {
	pub shop: Shop
}

#[function_component(ShopItem)]
pub fn shop_item(ShopItemProps { shop } : &ShopItemProps) -> Html {
	html! {
		<p>{format!("{}: {}", shop.id, shop.name)}</p>
	}
}