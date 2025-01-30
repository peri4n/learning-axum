pub mod requests;
pub mod postgres;

use crate::model::Person;
use crate::persistence::requests::*;

#[derive(Debug, Default, serde::Deserialize, PartialEq, Eq)]
pub struct DbConfig {
    user: String,
    password: String,
    host: String,
    port: String,
    name: String,
}

impl DbConfig {
    pub fn connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}

pub trait PersonRepo {
    async fn create_person(&self, req: &CreatePersonRequest) -> Result<Person, CreatePersonRequest>;
    fn update_person(&self, req: &UpdatePersonRequest) -> Result<Person, UpdatePersonRequest>;
    fn delete_person(&self, req: &DeletePersonRequest) -> Result<Person, DeletePersonRequest>;
}
