use yew::prelude::*;

use crate::types::Shop;

#[derive(Properties, PartialEq)]
pub struct ShopItemProps {
	pub shop: Shop,
	pub show_more: bool,
}

#[function_component(ShopItem)]
pub fn shop_item(ShopItemProps { shop, show_more } : &ShopItemProps) -> Html {
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
                <p>{format!("地址: {}", shop.address)}</p>
                if *show_more {
	                <p>{format!("营业时间: {}", shop.open_time)}</p>
	                <p>{format!("联系方式: {}", shop.telephone)}</p>
	                <p>{format!("标签: {}", shop.tags)}</p>
	                <p class="shop-card-desc">{format!("简介: {}", shop.desc)}</p>
            	}
            </div>
        </div>
	}
}