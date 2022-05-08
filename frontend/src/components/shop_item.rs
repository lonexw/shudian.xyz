use yew::prelude::*;

use crate::types::Shop;

#[derive(Properties, PartialEq)]
pub struct ShopItemProps {
	pub shop: Shop
}

#[function_component(ShopItem)]
pub fn shop_item(ShopItemProps { shop } : &ShopItemProps) -> Html {
	html! {
		<div class="shop-card">
            <div class="shop-card-images">
                <img src={shop.cover_image.clone()} alt={shop.name.clone()} />
            </div>
            <div class="shop-card-content">
                <p class="shop-card-name">
                	<span>{format!("{}", shop.name)}</span>
                	<span class="shop-mark-button">{"📷 拍照打卡 🌟"}</span>
                </p>
                <p class="shop-card-address">{format!("地址：{}", shop.address)}</p>
            </div>
        </div>
	}
}