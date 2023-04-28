use crate::schema::users;
use crate::schema::todos;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
}

#[derive(Queryable, Debug, AsChangeset)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub completed: bool,
    pub user_id: &'a i32
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub user_id: i32,
}
