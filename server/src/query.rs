use axum::{response::IntoResponse, Json};

pub async fn get_shops() -> impl IntoResponse {
    Json(serde_json::json!([
        { "id": "1", 
            "name": "做書书咖酒馆 北新桥", 
            "cover_image": "https://s1.ax1x.com/2022/05/08/O13V3j.jpg", 
            "address": "北京市东城区后永康胡同16号",
            "open_time": "11:00-20:00",
            "telephone": "010-64176626",
            "tags": "网红、二手书、阅读空间",
            "desc": "PAGEONE的特色是原版艺术设计类书籍、英文原版、港台版还有原版杂志、搭卖的高端文具。"
        }, 
        { "id": "2", 
            "name": "做书西单更新场", 
            "cover_image": "https://s1.ax1x.com/2022/05/08/O12HqP.jpg", 
            "address": "北京市西城区西单北大街180号(西单地铁站A西北口步行290米)",
            "open_time": "11:00-20:00",
            "telephone": "010-64176626",
            "tags": "网红、二手书、阅读空间",
            "desc": "PAGEONE的特色是原版艺术设计类书籍、英文原版、港台版还有原版杂志、搭卖的高端文具。" }]))
}