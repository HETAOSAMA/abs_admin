use actix_web::{web::{self}, post, Responder};
use crate::domain::dto::auth::SysAuthDTO;
use crate::domain::vo::RespVO;
use crate::service::CONTEXT;

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin/auth")
        .service(check)
    );
}

///检测token以及path 是否有效且允许访问
#[post("/check")]
pub async fn check(arg: web::Json<SysAuthDTO>) -> impl Responder {
    let r = CONTEXT.sys_auth_service.check_auth(arg.0).await;
    RespVO::from_result(&r).resp_json()
}
