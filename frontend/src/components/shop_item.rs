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
		<div class="shop-card" id={shop.id.clone()}>
			if false {
	            <div class="shop-card-images">
	                <img src={shop.cover_image.clone()} alt={shop.name.clone()} />
	            </div>
        	}
            <div class="shop-card-content">
                <p class="shop-card-name">
                	<div class="shop-card-name-wrap">
                		<span>{format!("{}", shop.name)}</span>
                		<span>{"🌟"}</span>
                	</div>
                	<div class="shop-mark-button">
                		<img src="https://s1.ax1x.com/2022/05/10/OYX2Zt.jpg" />
                	</div>
                </p>
                <p><span class="shop-card-state">{"正常营业"}</span></p>
                <p>{format!("🏷️ {}", shop.tags)}</p>
                <p>{format!("地址: {}", shop.address)}</p>
                if *show_more {
	                <p>{format!("营业时间: {}", shop.open_time)}</p>
	                <p>{format!("联系方式: {}", shop.telephone)}</p>
	                <p class="shop-card-desc">{format!("简介: {}", shop.desc)}</p>
            	}
            </div>
        </div>
	}
}