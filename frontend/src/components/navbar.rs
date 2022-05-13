use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::api::get_cities;


#[derive(Clone, Properties, PartialEq)]
pub struct NavbarProps {
	pub on_changed: Callback<String>,
}

#[function_component(Navbar)]
pub fn navbar(NavbarProps { on_changed } : &NavbarProps) -> Html {
	let is_changing = use_state(|| false);
	let selected_city = use_state(|| "北京市".to_string());
	let cities = use_state(|| None);

	// Request cities
    {
        let cities = cities.clone();
        use_effect(move || {
            if cities.is_none() {
                spawn_local(async move {
                    let result = get_cities().await;
                    cities.set(Some(result));
                });
            }

            || {}
        })
    }

	let on_change_city = {
		let is_changing = is_changing.clone();
		Callback::from(move |_| {
			is_changing.set(!*is_changing);
		})
	};

	html! { 
		<div class="navbar">
			<div class="nav-header">
				<img class="logo" src="logo.png" alt="书店指南" />
				<div class="city-select" onclick={on_change_city}>
					{
						match *is_changing {
							false => html! {
								<span>{format!("{} ", *selected_city)} </span>
							},
							true => html! {
								<span>{format!("收起")} </span>
							}
						}
					}
				</div>
			</div>
			if *is_changing {
				<div class="nav-map">
				{
					match cities.as_ref() {
						None => {
				            html! {
				                <div>{"获取城市列表数据出错！"}</div>
				            }
				        }
				        Some(Ok(response)) => {
				        	response.data.clone().into_iter().map(|city| {
				        		let change_city = {
									let is_changing = is_changing.clone();
									let selected_city = selected_city.clone();
									let on_changed = on_changed.clone();
									let city = city.clone();
									Callback::from(move |_| {
										is_changing.set(false);
										selected_city.set(city.clone());
										on_changed.emit(city.clone());
									})
								};

								html! {
									if *selected_city == city {
										<span class="city-name city-name-selected">{format!("{}", city)}</span>
									} else {
										<span class="city-name" onclick={change_city}>{format!("{}", city)}</span>
									}
								}
							}).collect::<Html>()
				        }
				        Some(Err(err)) => {
				            html! {
				                <div>{"Error requesting data from server: "}{err}</div>
				            }
				        }
					}
				}
				</div>
			}
		</div>

	}
}