use clap::{ArgMatches};

pub trait Command {
    //Constructor
    fn create(ArgMatches<'static>) -> Self where Self:Sized;
    
    //Information
    fn rtti(&self) -> &'static str ;
    
    //Execute
    fn execute(&self);
}