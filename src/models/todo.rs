use poem_openapi::Object;

#[derive(Object)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: bool,
}
