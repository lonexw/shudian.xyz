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
				<ShopItem shop={shop.clone()} />
			</div>
			<div class="shop-detail-container">
				<div class="shop-nav-tabs">
					<div class="tab">{"推荐书单"}</div>
				</div>
				<div class="shop-detail-content">
					
				</div>
			</div>
		</div>
	}
}