using extension edgeql_http;

module default {
	# 正常营业、新开业（一年以内的店铺）、停止营业、装修中
	scalar type OperationState extending enum<Normal, New, Close, Building>;

	# 店铺信息状态: 审核中、已审核、被拒绝
	scalar type ShopStatus extending enum<Auditing, Verified, Rejected>;

	type Location {
		required property name -> str;
		required property address -> str;
		required property province -> str;
		required property city -> str;
		required property district -> str;

		property longitude -> float32;
		property latitude -> float32;
	}

	type Tag {
		required property name -> str;
		link parent -> Tag;
	}

	type Shop {
		required property name -> str;
		required property status -> ShopStatus {
			default := ShopStatus.Auditing;
		};
		property address -> str;
		property operation_state -> OperationState {
			default := OperationState.Normal;
		};
		property open_time -> str;
		property logo -> str;
		property telephone -> str;
		property intro -> str;
		property start_at -> cal::local_date;	# 开业时间
		property end_at -> cal::local_date;		# 停业时间

		link location -> Location;
		multi link tags -> Tag;
		multi link photos -> ShopPhoto;
		multi link supporters -> ShopSupporter;
	}

	type Bookshop extending Shop {

	}

	type PublisingHouse extending Shop {

	}

	type PersonalShelf extending Shop {

	}

	type ShopSupporter {
		required property avatar_url -> str;
		required property nickname -> str;
		required property joined_at -> datetime;
		property wx_openid -> str;

	}

	type ShopPhoto {
		required property url -> str;
		property intro -> str;
		required property created_at -> datetime;

		link creator -> ShopSupporter;
	}


	# ShopCompany Info
	# ShopOwner
} 
