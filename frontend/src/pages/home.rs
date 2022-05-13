use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::api::get_shops;
use crate::types::Shop;
use crate::components::Navbar;
use crate::components::ShopList;
use crate::pages::ShopDetail;

#[function_component(Home)]
pub fn home() -> Html {
    let shops = use_state(|| None);

    // Request `/api/shops` once
    {
        let shops = shops.clone();
        use_effect(move || {
            if shops.is_none() {
                spawn_local(async move {
                    let result = get_shops("北京市".to_string()).await;
                    shops.set(Some(result))
                });
            }

            || {}
        });
    }

    let on_city_changed = {
        let new_shops = shops.clone();
        Callback::from(move |city| {
            let new_shops = new_shops.clone();
            spawn_local(async move {
                    let result = get_shops(city).await;
                    new_shops.set(Some(result));
                });

        })
    };
  

    match shops.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(Ok(response)) => {

            let selected_shop = use_state(|| None);
            let on_shop_select = {
                let selected_shop = selected_shop.clone();
                Callback::from(move |shop: Shop| {
                    selected_shop.set(Some(shop))
                })
            };
            let detail = selected_shop.as_ref().map(|shop| html! {
                <ShopDetail shop={shop.clone()} />
            });


            let close_shop_detail = {
                Callback::from(move |_| {
                    selected_shop.set(None)
                })
            };

            html! {
                <div class="main-content">
                    <div class="shop-list">
                        <Navbar on_changed={on_city_changed} />
                        <div class="container">
                            <ShopList shops={response.data.clone()} on_click={on_shop_select.clone()} />
                            <p class="guide-to-pc">{"电脑访问：https://shudian.xyz | "}</p>
                            <p class="guide-to-pc">{"🪧 京ICP备16050972号-19"}</p>
                        </div>
                    </div>

                    if detail.is_some() {
                        <button class="shop-detail-close" onclick={close_shop_detail}>{"关闭"}</button>
                        { for detail }
                    }
                </div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
    }
}