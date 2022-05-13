
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Supporter {
	pub nickname: String,
	pub avatar_url: String,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Shop {
	pub id: String,
	pub name: String,
	pub address: String,
	pub open_time: String,
	pub telephone: String,
	pub intro: String,
	pub status: String,
	pub operation_state: String,
	pub supporters: Vec<Supporter>,
}