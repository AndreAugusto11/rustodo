use crate::args::{
  TodoCommand,
  TodoSubcommand,
  CreateTodo,
  DeleteTodo,
  CompleteTodo,
  ShowTodos
};

use crate::models::{NewTodo, Todo};
use crate::db::establish_connection;
use diesel::prelude::*;

pub fn handle_todo_command(todo: TodoCommand) {
  let command = todo.command;
  match command {
      TodoSubcommand::Create(todo) => {
          create_todo(todo);
      }
      TodoSubcommand::Delete(todo) => {
          delete_todo(todo);
      }
      TodoSubcommand::Complete(todo) => {
        complete_todo(todo);
      },
      TodoSubcommand::Show(todo) => {
          show_todos(todo);
      }
  }
}

pub fn create_todo(todo: CreateTodo) {
  println!("Creating todo: {:?}", todo);
  use crate::schema::todos::dsl::*;

  let connection = establish_connection();
  let new_todo = NewTodo {
      title: &todo.title,
      description: &todo.description,
      completed: false,
      user_id: &todo.user_id,
  };

  diesel::insert_into(todos)
      .values(&new_todo)
      .execute(&connection)
      .expect("Error saving new todo");
}

pub fn delete_todo(todo: DeleteTodo) {
  println!("Deleting todo: {:?}", todo);
  use crate::schema::todos::dsl::*;

  let connection = establish_connection();
  diesel::delete(todos.find(todo.id))
      .execute(&connection)
      .expect("Error deleting todo");
}

pub fn show_todos(todo: ShowTodos) {
  println!("Showing todos");
  use crate::schema::todos::dsl::*;

  let connection = establish_connection();
  let results = todos
      .filter(user_id.eq(todo.user_id))
      .load::<Todo>(&connection)
      .expect("Error loading todos");

  println!("Displaying {} todos", results.len());
  for todo in results {
      println!("{:?}", todo);
  }
}

pub fn complete_todo(todo: CompleteTodo) {
    println!("Completing todo: {:?}", todo);
    println!("Not implemented");
    // use crate::schema::todos::dsl::*;

    // let connection = establish_connection();

    // let db_todo = IgnoreNoneFieldsUpdateUser {
    //     completed: true,
    // };
    
    // diesel::update(&db_todo)
    //     .set(completed.eq(true))
    //     .execute(&connection)
    //     .expect("Error updating todo");
}
