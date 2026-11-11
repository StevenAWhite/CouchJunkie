use cmd::Command;
use clap::ArgMatches;

#[allow(dead_code)]
#[allow(unused_variables)]
pub struct DBAdmin { 
  args : ArgMatches<'static>
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Command for DBAdmin  {
  fn create(p_args: ArgMatches<'static>) -> Self
  {
    DBAdmin {
      args : p_args
    }
  }
  //Information
  fn rtti(&self) -> &'static str
  {
       "SQLCommand" 
  }
  
  //Execute
  fn execute(&self)
  {
     let dirs : Vec<&str> = self.args.values_of("dirs").unwrap().collect();
     println!("{:?}", dirs);
  }

}