use actix_web::{web, HttpRequest, Responder,post};

use crate::domain::dto::{IdDTO, SignInDTO, UserAddDTO, UserEditDTO, UserRolePageDTO};
use crate::domain::vo::{JWTToken, RespVO};
use crate::service::CONTEXT;

pub fn user_config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/admin")
        .service(login)
        .service(info)
        .service(add)
        .service(page)
        .service(detail)
        .service(update)
        .service(remove)
    );
}

/// 用户登陆
#[post("/login")]
pub async fn login(arg: web::Json<SignInDTO>) -> impl Responder {
    log::info!("login:{:?}", arg.0);
    let vo = CONTEXT.sys_user_service.sign_in(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 用户信息
#[post("/sys_user_info")]
pub async fn info(req: HttpRequest) -> impl Responder {
    let token = req.headers().get("access_token");
    return match token {
        Some(token) => {
            let token = token.to_str().unwrap_or("");
            let token = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
            if token.is_err() {
                return RespVO::from_result(&token).resp_json();
            }
            let user_data = CONTEXT
                .sys_user_service
                .get_user_info_by_token(&token.unwrap())
                .await;
            RespVO::from_result(&user_data).resp_json()
        }
        _ => RespVO::<String>::from_error_info("access_token is empty!", "").resp_json(),
    };
}

/// 用户添加
#[post("/sys_user_add")]
pub async fn add(arg: web::Json<UserAddDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_service.add(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///用户分页
#[post("/sys_user_page")]
pub async fn page(arg: web::Json<UserRolePageDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_role_service.page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///用户详情
#[post("/sys_user_detail")]
pub async fn detail(arg: web::Json<IdDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_service.detail(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///用户修改
#[post("/sys_user_update")]
pub async fn update(arg: web::Json<UserEditDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_service.edit(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

///用户删除
#[post("/sys_user_remove")]
pub async fn remove(arg: web::Json<IdDTO>) -> impl Responder {
    let vo = CONTEXT
        .sys_user_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    return RespVO::from_result(&vo).resp_json();
}
