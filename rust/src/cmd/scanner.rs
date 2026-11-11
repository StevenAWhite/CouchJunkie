use cmd::Command;
use clap::ArgMatches;

use std::fs;

pub struct Scanner{
  args : ArgMatches<'static>
}

impl Scanner {
  fn scan_dir(&self, dir:&str)
  {
    let paths =  fs::read_dir(dir)
      .expect(format!("Unable to read directory {:?}", dir).as_str());
    
    for p in paths 
    {
      let path = match p {
        Err(why) => {println!("! {:?}", why.kind());break;}
        ,Ok(p)   => p.path()
        ,
      };

      let s_path =  match path.to_str() {
          None    => break
         ,Some(s_path) => s_path
      }; 
      
      if path.is_dir() {
          self.scan_dir(s_path);
      } else {
        println!("{:?}", s_path);
      }
    }       
  }
}

impl Command for Scanner  {
  fn create(p_args: ArgMatches<'static>) -> Self
  {
    Scanner {
        args:p_args
    }
  }
  //Information
  fn rtti(&self) -> &'static str
  {
    "Scanner"
  }
  
  //Execute
  #[allow(unused_parens)]
  fn execute(&self)
  {
    let dirs : Vec<&str> = self.args.values_of("dirs").unwrap().collect();
    for dir in dirs {
      self.scan_dir(dir);
    }
  }
}