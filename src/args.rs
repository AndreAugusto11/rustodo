
use clap:: {
  Args,
  Parser,
  Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RustodoArgs {
  #[clap(subcommand)]
  pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
  /// Create, update, delete or show users
  User(UserCommand),

  /// Create, update, delete or show todos
  Todo(TodoCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Args)]
pub struct TodoCommand {
    #[clap(subcommand)]
    pub command: TodoSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// Create a new user
    Create(CreateUser),

    /// Delete a user
    Delete(DeleteUser),

    /// Show all users
    Show,
}

#[derive(Debug, Subcommand)]
pub enum TodoSubcommand {
    /// Create a new todo
    Create(CreateTodo),

    /// Delete a todo
    Delete(DeleteTodo),

    /// Update a todo
    Update(UpdateTodo),

    /// Show all todos
    Show(ShowTodos),
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// The name of the user
    pub name: String,

    /// The email of the user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct DeleteUser {
    /// The id of the entity to delete
    pub id: i32,
}


#[derive(Debug, Args)]
pub struct CreateTodo {
    /// The title of the todo to create
    pub title: String,

    /// The description of the todo to create
    pub description: String,

    /// The user that creates the to do
    pub user_id: i32,
}

#[derive(Debug, Args)]
pub struct DeleteTodo {
    /// The id of the entity to delete
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct ShowTodos {
    pub user_id: i32,
}

#[derive(Debug, Args)]
pub struct UpdateTodo {
    /// The id of the todo to update
    pub id: i32,

    /// The title of the todo to update
    pub title: String,

    /// The description of the todo to update
    pub description: String,

    /// The user that updates the to do
    pub user_id: i32,
}