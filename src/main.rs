use clap::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args{
    //Prueba
    #[clap(short, long)]
    name: String,

    //Ejemplo
    #[clap(short, log, devault_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    for _ in 0..args.count(){
        println!("{}", args.name);
    }
}
