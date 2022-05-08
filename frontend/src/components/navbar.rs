use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
	html! { 
		<div class="navbar">
			<div class="nav-header">
				<img class="logo" src="logo.png" alt="书店指南" />
				<div class="city-select">
					<span>{"北京"}</span>
				</div>
			</div>
			<div class="nav-map"></div>
		</div>

	}
}