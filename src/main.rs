use abs_admin::controller::{
    img_controller, sys_auth_controller, sys_dict_controller, sys_res_controller,
    sys_role_controller, sys_user_controller,
};
use abs_admin::middleware::auth_actix::Auth;
use abs_admin::service::CONTEXT;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .insert_header(("Cache-Control", "no-cache"))
        .body("[abs_admin] Hello !")
}

/// use tokio,because Rbatis specifies the runtime-tokio
#[tokio::main]
async fn main() -> std::io::Result<()> {
    //日志追加器
    abs_admin::config::log::init_log();
    //连接数据库
    CONTEXT.link_db().await;
    //路由
    HttpServer::new(|| {
        App::new()
            .wrap(Auth {})
            .route("/", web::get().to(index))
            .configure(sys_user_controller::user_config)
            .configure(sys_role_controller::role_config)
            .configure(sys_res_controller::res_config)
            .configure(sys_dict_controller::dict_config)
            .configure(sys_auth_controller::auth_config)
            .configure(img_controller::img_config)
    })
    .bind(&CONTEXT.config.server_url)?
    .run()
    .await
}
