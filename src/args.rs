#[derive(clap::Parser, Debug)]
#[command(version, about, author)]
pub(crate) struct Args {
    #[clap(env)]
    pub(crate) database_url: String,
}
