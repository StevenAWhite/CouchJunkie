use cmd::Command;
use clap::ArgMatches;

#[allow(dead_code)]
#[allow(unused_variables)]
pub struct Configure { 
  args : ArgMatches<'static>
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Command for Configure  {
  fn create(p_args: ArgMatches<'static>) -> Self
  {
    Configure {
      args: p_args
    }
  }
  //Information
  fn rtti(&self) -> &'static str
  {
    "InitCommand"
  }
  
  //Execute
  fn execute(&self)
  {
    let dirs : Vec<&str> = self.args.values_of("dirs").unwrap().collect();
    println!("{:?}", dirs); 
  }

}