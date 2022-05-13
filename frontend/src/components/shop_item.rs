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
            <div class="shop-card-content">
                <p class="shop-card-name">
                	<div class="shop-card-name-wrap">
                		<span>{format!("{}", shop.name)}</span>
                		// <span>{"🌟"}</span >
                	</div>
                </p>
        		if shop.status != "Auditing" {
        			<p><span class="shop-card-state info-wanring-text">{"核实中"}</span></p>
        		}
        		else 
        		{
            		if shop.operation_state == "Normal" {
						<p><span class="shop-card-state">{"🙌 正常营业"}</span></p>
					}

					if shop.operation_state == "New" {
						<p><span class="shop-card-state info-success">{"🎉 新开业"}</span></p>
					}

					if shop.operation_state == "Building" {
						<p><span class="shop-card-state info-secondary">{"🏗 装修中"}</span></p>
					}

					if shop.operation_state == "Close" {
						<p><span class="shop-card-state info-danger">{"🤚 停止营业"}</span></p>
					}
				}
                <p>{format!("🏷️ 暂无标签")}</p>
                <p>{format!("地址: {}", shop.address)}</p>
                if *show_more {
	                <p>{format!("营业时间: {}", shop.open_time)}</p>
	                <p>{format!("联系方式: {}", shop.telephone)}</p>
	                <p class="shop-card-desc">{format!("简介: {}", shop.intro)}</p>
	                <p class="tips">{"本信息由他们提供维护"}</p>
	                <div class="shop-detail-supporters">
                		{
                			shop.supporters.clone().into_iter().map(|supporter| {
                				html! {
                					<div class="supporter-card">
                						<img src={supporter.avatar_url} />
                						<p class="nickname">{format!("{}",supporter.nickname)}</p>
                					</div>
                				}
                			}).collect::<Html>()
                		}
                	</div>
            	}	
            </div>
        </div>
	}
}