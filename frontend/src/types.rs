
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Shop {
	pub id: String,
	pub name: String,
	pub cover_image: String,
	pub address: String,
}