
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Shop {
	pub id: String,
	pub name: String,
	pub cover_image: String,
	pub address: String,
	pub open_time: String,
	pub telephone: String,
	pub tags: String,
	pub desc: String,
}