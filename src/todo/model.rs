use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, HttpRequest, Responder, Error};
use futures::future::{ready, Ready};
use sqlx::{PgPool, Row};
use sqlx::postgres::PgRow;
use anyhow::Result;

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoRequest {
    pub description: String,
    pub done: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: i32,
}

impl Responder for Todo {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

impl Todo {
    pub async fn create(todo: TodoRequest, pool: &PgPool) -> Result<Todo> {
        let mut tx = pool.begin().await?;
        let todo = sqlx::query(
            "INSERT INTO todos (description, done) VALUES ($1, $2) RETURNING id, description, done",
        )
        .bind(&todo.description)
        .bind(todo.done)
        .map(|row: PgRow| Todo {
            id: row.get(0),
            description: row.get(1),
            done: row.get(2),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(todo)
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Todo>> {
        let mut todos = vec![];
        let recs = sqlx::query(r#" SELECT * FROM todos ORDER BY id"#)
            .map(|row: PgRow| Todo {
                id: row.get(0),
                description: row.get(1),
                done: row.get(2),
            })
            .fetch_all(pool)
            .await?;

        for rec in recs {
            todos.push(Todo {
                id: rec.id,
                description: rec.description,
                done: rec.done,
            });
        }

        Ok(todos)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<Todo> {
        let mut tx = pool.begin().await?;
        let todo = sqlx::query(
            r#"
                SELECT * FROM todos WHERE id = $1
            "#,
        )
        .bind(id)
        .map(|row: PgRow| Todo {
            id: row.get(0),
            description: row.get(1),
            done: row.get(2),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await.unwrap();
        Ok(todo)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<i32> {
        let mut tx = pool.begin().await?;
        let deleted = sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(deleted as i32)
    }

    pub async fn update(id: i32, todo: TodoRequest, pool: &PgPool) -> Result<Todo> {
        let mut tx = pool.begin().await.unwrap();
        let todo = sqlx::query("UPDATE todos SET description = $1, done = $2 WHERE id = $3 RETURNING id, description, done")
            .bind(&todo.description)
            .bind(todo.done)
            .bind(id)
            .map(|row: PgRow| {
                Todo {
                    id: row.get(0),
                    description: row.get(1),
                    done: row.get(2)
                }
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await.unwrap();
        Ok(todo)
    }
}
