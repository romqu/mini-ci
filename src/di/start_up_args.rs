use clap::Parser;

#[derive(Parser, Debug)]
#[clap(long_about = None)]
pub struct StartupArgs {
    #[clap(long)]
    pub(crate) ssh_passphrase: String,

    #[clap(long)]
    pub(crate) ssh_key_path: String,
}