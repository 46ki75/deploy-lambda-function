use clap::Parser;

fn main() {
    let cli = deploy_lambda_function::cli::Cli::parse();

    println!("region: {:?}", cli.region);
}
