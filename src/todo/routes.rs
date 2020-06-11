use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::todo::{Todo, TodoRequest};

#[get("/todos")]
async fn find_all(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Todo::find_all(db_pool.get_ref()).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        _ => HttpResponse::BadRequest().body("Error getting todos from database"),
    }
}

#[get("/todo/{id}")]
async fn find(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Todo::find_by_id(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        _ => HttpResponse::BadRequest().body("Todo not found"),
    }
}

#[post("/todo")]
async fn create(todo: web::Json<TodoRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    info!("Creating new Todo");
    let result = Todo::create(todo.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        _ => HttpResponse::BadRequest().body("Error trying to create new todo"),
    }
}

#[delete("/todo/{id}")]
async fn delete(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Todo::delete(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(deleted) => HttpResponse::Ok().json(deleted),
        _ => HttpResponse::BadRequest().body("Error deleting resource"),
    }
}

#[put("/todo/{id}")]
async fn update(
    id: web::Path<i32>,
    todo: web::Json<TodoRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = Todo::update(id.into_inner(), todo.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        _ => HttpResponse::BadRequest().body("Error updating resource"),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(delete);
    cfg.service(update);
}
