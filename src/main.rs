use structopt::StructOpt;
#[path = "lib/util.rs"] mod util;

fn main() {
    let opts = util::FirePhasers::from_args();
    println!("args: {:?}", opts);
}
