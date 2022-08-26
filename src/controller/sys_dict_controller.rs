use crate::domain::dto::{DictAddDTO, DictEditDTO, DictPageDTO, IdDTO};
use crate::domain::table::SysDict;
use crate::domain::vo::RespVO;
use crate::service::CONTEXT;
use actix_web::{web, Responder,post};
use rbatis::rbdc::datetime::FastDateTime;

pub fn dict_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
        .service(page)
        .service(add)
        .service(update)
        .service(remove)
    );
}

/// 字典分页(json请求)
#[post("/sys_dict_page")]
pub async fn page(page: web::Json<DictPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_dict_service.page(&page.0).await;
    RespVO::from_result(&data).resp_json()
}

//字典添加
#[post("/sys_dict_add")]
pub async fn add(mut arg: web::Json<DictAddDTO>) -> impl Responder {
    if arg.name.is_none() {
        return RespVO::<u64>::from_error_info("", "字典名字不能为空!").resp_json();
    }
    if arg.code.is_none() {
        return RespVO::<u64>::from_error_info("", "字典code不能为空!").resp_json();
    }
    if arg.state.is_none() {
        arg.state = Some(1);
    }
    let res = SysDict {
        id: arg.name.clone().into(),
        name: arg.name.clone(),
        code: arg.code.clone(),
        state: arg.state.clone(),
        create_date: FastDateTime::now().set_micro(0).into(),
    };
    let data = CONTEXT.sys_dict_service.add(&res).await;
    CONTEXT.sys_dict_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

///字典修改
#[post("/sys_dict_update")]
pub async fn update(arg: web::Json<DictEditDTO>) -> impl Responder {
    let data = CONTEXT.sys_dict_service.edit(&arg.0).await;
    CONTEXT.sys_dict_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}

///字典删除
#[post("/sys_dict_remove")]
pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let data = CONTEXT
        .sys_dict_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    CONTEXT.sys_dict_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
}
