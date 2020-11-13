use structopt::StructOpt;
#[path = "lib/util.rs"] mod util;

fn main() {
    let opts = util::FirePhasers::from_args();

    println!("args: {:?}", opts.cmd);
    //let parsed_opts = util::parse_opts(&opts.cmd);

    match opts.cmd {
        util::CommandList::Add { alias, command, path } => util::add_task(&alias, &command, path.unwrap_or(String::from(""))),
        util::CommandList::Remove { alias } => println!("{}", alias),
        util::CommandList::List {} => println!("Listing tasks..."),
    }

    // ADD
    //

}
