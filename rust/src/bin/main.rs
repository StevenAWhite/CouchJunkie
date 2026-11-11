extern crate diesel;

use diesel::prelude::*;
use diesel::types::{Text};
use diesel::expression::dsl::sql;
fn main() {
  use diesel::mysql::MysqlConnection;
  let database_url = "mysql://ignitus:dbpass@autoarcade/couchjunkie";
  let conn = MysqlConnection::establish(database_url)
     .expect(&format!("Error connecting to {}", database_url));
     
  println!("Successful Connection!");
  
  let result = sql::<Text>("SHOW databases;")
  .get_result::<String>(&conn).expect(&format!("Failed to execute query"));
  
  println!("{:?}", result);
}