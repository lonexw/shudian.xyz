use yew::prelude::*;
use crate::types::Shop;

#[derive(Clone, Properties, PartialEq)]
pub struct ShopDetailProps {
	pub shop: Shop,
}

#[function_component(ShopDetail)]
pub fn shop_detail(ShopDetailProps { shop } : &ShopDetailProps) -> Html {
	html! {
		<p>{format!("{}: {}", shop.id, shop.name)}</p>
	}
}