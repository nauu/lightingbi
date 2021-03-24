use crate::handler::default::{favicon, response_body, welcome, with_param};
use actix_files as fs;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{error, web, HttpRequest, HttpResponse};
use std::io;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(favicon)
        // register simple route, handle all methods
        .service(welcome)
        // with path parameters
        .service(web::resource("/user/{name}").route(web::get().to(with_param)))
        // async response body
        .service(web::resource("/async-body/{name}").route(web::get().to(response_body)))
        .service(
            web::resource("/test").to(|req: HttpRequest| match *req.method() {
                Method::GET => HttpResponse::Ok(),
                Method::POST => HttpResponse::MethodNotAllowed(),
                _ => HttpResponse::NotFound(),
            }),
        )
        .service(web::resource("/error").to(|| async {
            error::InternalError::new(
                io::Error::new(io::ErrorKind::Other, "test"),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }))
        // static files
        .service(fs::Files::new("/static", "static").show_files_listing())
        .service(fs::Files::new("/assets", "static/dist/assets").show_files_listing())
        .service(fs::Files::new("/resource", "static/dist/resource").show_files_listing())
        .service(fs::Files::new(
            "/_app.config.js",
            "static/dist/_app.config.js",
        ))
        // redirect
        .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
            println!("{:?}", req);
            HttpResponse::Found()
                //.header(header::LOCATION, "static/welcome.html")
                .header(header::LOCATION, "static/dist/index.html")
                .finish()
        })));
}
