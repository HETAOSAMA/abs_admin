use actix_web::{web, Responder,post};
use rbatis::plugin::object_id::ObjectId;
use rbatis::rbdc::types::datetime::FastDateTime;

use crate::domain::dto::{IdDTO, ResAddDTO, ResEditDTO, ResPageDTO};
use crate::domain::table::SysRes;
use crate::domain::vo::RespVO;
use crate::service::CONTEXT;

pub fn res_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
        .service(page)
        .service(all)
        .service(layer_top)
        .service(add)
        .service(update)
        .service(remove)
    );
}

/// 资源分页(json请求)
#[post("/sys_res_page")]
pub async fn page(page: web::Json<ResPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_res_service.page(&page.0).await;
    RespVO::from_result(&data).resp_json()
}

/// 资源全部(json请求)
#[post("/sys_res_all")]
pub async fn all() -> impl Responder {
    let data = CONTEXT.sys_res_service.finds_all().await;
    RespVO::from_result(&data).resp_json()
}

/// 顶层权限
#[post("/sys_res_layer_top")]
pub async fn layer_top() -> impl Responder {
    let data = CONTEXT.sys_res_service.finds_layer_top().await;
    RespVO::from_result(&data).resp_json()
}

///资源添加
#[post("/sys_res_add")]
pub async fn add(mut arg: web::Json<ResAddDTO>) -> impl Responder {
    if arg.name.is_none() {
        return RespVO::<u64>::from_error_info("", "资源名字不能为空!").resp_json();
    }
    if arg.permission.is_none() {
        return RespVO::<u64>::from_error_info("", "资源permission不能为空!").resp_json();
    }
    if arg.path.is_none() {
        arg.path = Some("".to_string());
    }
    let res = SysRes {
        id: ObjectId::new().to_string().into(),
        parent_id: arg.parent_id.clone(),
        name: arg.name.clone(),
        permission: arg.permission.clone(),
        path: arg.path.clone(),
        del: 0.into(),
        create_date: FastDateTime::now().set_micro(0).into(),
    };
    let data = CONTEXT.sys_res_service.add(&res).await;
    CONTEXT.sys_res_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

///资源修改
#[post("/sys_res_update")]
pub async fn update(arg: web::Json<ResEditDTO>) -> impl Responder {
    let data = CONTEXT.sys_res_service.edit(&arg.0).await;
    CONTEXT.sys_res_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

///资源删除
#[post("/sys_res_remove")]
pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let data = CONTEXT
        .sys_res_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    CONTEXT.sys_res_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}
