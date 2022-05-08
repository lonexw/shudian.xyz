using extension edgeql_http;

module default {
	type Shop {
		required property name -> str;
		required property open_time -> str;
		required property address -> str;
		required property cover_image -> str;
		required property telephone -> str;
		required property tags -> str;
		required property desc -> str;
	}
}
