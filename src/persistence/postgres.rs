use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Row};

use crate::model::*;
use super::*;

#[derive(Debug, Clone)]
pub struct Pg {
    pool: PgPool,
}

impl Pg {
    pub async fn new(url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .unwrap();

        Self { pool }
    }
}

impl PersonRepo for Pg {
    async fn create_person(
        &self,
        req: &CreatePersonRequest,
    ) -> Result<Person, CreatePersonRequest> {
        let mut tx = self
            .pool
            .begin()
            .await
            .unwrap_or_else(|e| panic!("failed to start Postgres transaction: {}", e));

        let person_id: i32 = sqlx::query::<Postgres>("INSERT INTO people (name, age) VALUES ($1, $2) RETURNING id")
            .bind(req.name.clone())
            .bind(req.age as i32)
            .fetch_one(&mut *tx)
            .await
            .map(|res| res.get(0))
            .unwrap();

        tx.commit().await;

        Ok(Person { id: Some(person_id as u32), name: req.name.clone(), age: req.age })
    }

    fn update_person(
        &self,
        req: &UpdatePersonRequest,
    ) -> Result<crate::model::Person, super::UpdatePersonRequest> {
        todo!()
    }

    fn delete_person(
        &self,
        req: &DeletePersonRequest,
    ) -> Result<crate::model::Person, super::DeletePersonRequest> {
        todo!()
    }
}
