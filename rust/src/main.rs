extern crate diesel;
extern crate clap;

mod util;
mod cmd;


// use diesel::prelude::*;
// use diesel::types::{Text};
// use diesel::expression::dsl::sql;

use util::cli;
use cmd::Command;
use cmd::Configure;
use cmd::Scanner;
use cmd::DBAdmin;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
fn main() {  
  let cli = cli::build_cli().get_matches();;
  
  // Gets a value for config if supplied by user, or defaults to "default.conf"
  let dbURI = cli.value_of("database-uri").unwrap_or("mysql://root@localhost/");
  let dbPass = cli.value_of("database-pass").unwrap_or("defaultPassword");
  let V_LEVEL =  cli.occurrences_of("verbose");


  // You can also match on a subcommand's name
  let cmd : Box<cmd::Command> = match cli.subcommand_name()  {
        Some("scanner")     => {
          let scan:Scanner   = Command::create(cli.subcommand_matches("scanner").unwrap().clone()); 
          Box::new(scan)
        }
        ,Some("db")         => {
          let db:DBAdmin     = Command::create(cli.subcommand_matches("db").unwrap().clone()); 
          Box::new(db  )
        }
        ,Some("initialize") => {
          let conf:Configure = Command::create(cli.subcommand_matches("config").unwrap().clone()); 
          Box::new(conf)
        }
        ,None               => panic!("No Subcommand given")
        ,_                  => panic!("Unexpected Subcommand.")
  };
  

  cmd.execute();  
  // use diesel::mysql::MysqlConnection;
  // let database_url = "mysql://ignitus:dbpass@autoarcade/couchjunkie";
  // let conn = MysqlConnection::establish(database_url)
  //    .expect(&format!("Error connecting to {}", database_url));
     
  // println!("Successful Connection!");
  
  // let result = sql::<Text>("SHOW databases;")
  // .get_result::<String>(&conn)
  //   .expect(&format!("Failed to execute query"));
  
  // println!("{:?}", result);

}