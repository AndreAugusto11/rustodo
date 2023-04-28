use crate::args::{
  UserCommand,
  UserSubcommand,
  CreateUser,
  DeleteUser
};

use crate::models::{NewUser, User};
use crate::db::establish_connection;
use diesel::prelude::*;

pub fn handle_user_command(todo: UserCommand) {
  let command = todo.command;
  match command {
	  UserSubcommand::Create(todo) => {
		create_user(todo);
	  }
	  UserSubcommand::Delete(todo) => {
        delete_user(todo);
	  }
	  UserSubcommand::Show => {
	    show_users();
	  }
  }
}

pub fn create_user(user: CreateUser) {
  println!("Creating user: {:?}", user);
  use crate::schema::users::dsl::*;

  let connection = establish_connection();
  let new_user = NewUser {
	  name: &user.name,
	  email: &user.email,
  };

  diesel::insert_into(users)
	  .values(&new_user)
	  .execute(&connection)
	  .expect("Error saving new user");
}

pub fn delete_user(user: DeleteUser) {
    println!("Deleting user: {:?}", user);
    use crate::schema::users::dsl::*;
  
    let connection = establish_connection();
  
    diesel::delete(users.find(user.id))
        .execute(&connection)
        .expect("Error deleting new user");
}

pub fn show_users() {
    println!("Showing users");
    use crate::schema::users::dsl::*;
  
    let connection = establish_connection();
    let results = users
        .load::<User>(&connection)
        .expect("Error loading users");
  
    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:?}", user);
    }
  }