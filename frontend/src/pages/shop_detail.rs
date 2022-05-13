use yew::prelude::*;

use crate::types::Shop;
use crate::components::ShopItem;

#[derive(Clone, Properties, PartialEq)]
pub struct ShopDetailProps {
	pub shop: Shop,
}

#[function_component(ShopDetail)]
pub fn shop_detail(ShopDetailProps { shop } : &ShopDetailProps) -> Html {
	html! {
		<div class="shop-detail">
			<div class="shop-detail-header">
				<ShopItem shop={shop.clone()} show_more={true} />
			</div>
			<div class="shop-detail-images">
				<img src="gh_51d25f9a18eb_258.jpeg" />
			</div>
			<div class="shop-detail-container">
				<div class="shop-nav-tabs">
					<div class="tab">{"📚 精选书单"}</div>
					// <div class="tab">{"图书市集"}</div>
				</div>
				<div class="shop-detail-content">
					<div class="tips">{"书店精选书单，即将开启"}</div>
				</div>
			</div>
		</div>
	}
}