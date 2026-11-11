use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
//-------------------------------------------------------------------------------
//             Global Configuations
//-------------------------------------------------------------------------------
    let database_uri_arg = Arg::with_name("CJ_DB_URI")
        .long("database-uri")
        .help("Specifies the database URL to connect to. Falls back to \
               the CJ_DB_URI environment variable if unspecified.")
        .global(true)
        .takes_value(true);

    let database_pass_arg = Arg::with_name("CJ_DB_PASS")
        .long("database-pass")
        .help("Specifies the database password to connect to. Falls back to \
               the CJ_DB_PASS environment variable if unspecified.")
        .global(true)
        .takes_value(true);

    let verbose = Arg::with_name("verbose")
                     .short("v")
                     .multiple(true)
                     .help("Sets the level of verbosity")
                     .global(true)
                     ;
//-------------------------------------------------------------------------------
//             Database Command
//-------------------------------------------------------------------------------
    let init = SubCommand::with_name("initialize")
                   .alias("init")
                   .about("Database administration")
                   .version(env!("CARGO_PKG_VERSION"))
                   .help("Manages the underlying couchjunkie schema")
                   .arg(Arg::with_name("force")
                       .short("f")
                       .required(true)
                       .help("Flag required to ensure you intend to wipe and initialize the database. ")
                       )
                   ;

    let update = SubCommand::with_name("update")
                   .about("Verifies the current schema")
                   .arg(Arg::with_name("force")
                       .short("f")
                       .required(true)
                       .help("Flag required to ensure you intend to wipe and initialize the database. ")
                       )
                   ;

    let truncate = SubCommand::with_name("truncate")
                 .alias("trunc")
                 .about("Truncates the schema")
                 .arg(Arg::with_name("force")
                     .short("f")
                     .required(true)
                     .help("Flag required to ensure you intend to wipe and initialize the database. ")
                     )
                 ;

    let database = SubCommand::with_name("db")
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .about("Database administration")
                    .subcommand(init)
                    .subcommand(update)
                    .subcommand(truncate)
                    ;
//-------------------------------------------------------------------------------
//             Scanner Command
//-------------------------------------------------------------------------------
    let directory = Arg::with_name("dirs")
                 .long("directory")
                 .help("Directory list to search.")
                 .takes_value(true)
                 .multiple(true)
                 .index(1)
                 .required(true)
                 ;

    let regex = Arg::with_name("regex")
                 .long("regex")
                 .short("r")
                 .help("Regex pattern for file searching.")
                 .takes_value(true)
                 ;

    let level = Arg::with_name("max-level")
                .long("max-level")
                .short("l")
                .help("Limits the recursion level of the search.")
                .takes_value(true)
            ;
                
    let scanner = SubCommand::with_name("scanner")
                    .alias("scan")
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .about("Recurses through directories in search of media files.")
                    .arg(directory)
                    .arg(regex)
                    .arg(level)
                    ;
//-------------------------------------------------------------------------------
//              Configuration Command
//-------------------------------------------------------------------------------
    let config = SubCommand::with_name("config")
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .about("Manages couchjunkie settings")
                    ;
//-------------------------------------------------------------------------------
//             CodeJunkie Command
//-------------------------------------------------------------------------------
    App::new("CouchJunkie")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ignitus Boyone <IgnitusBoyone@gmail.com>")
        .about("Data miner for the CouchJunkie framework. 
                 - performs schema initialization, upgrades, and maintenance
                 - manages couchjunkie settings
                 - searches directory trees for media files [video,music,images]
                 - pulls metadata from the internet and populates codejunkie database")
        .arg(verbose)
        .arg(database_uri_arg)
        .arg(database_pass_arg)
        .subcommand(database)
        .subcommand(scanner)
        .subcommand(config)
}
