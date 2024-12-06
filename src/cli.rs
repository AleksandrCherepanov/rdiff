use clap_derive::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    ///The first folder to compare.
    #[arg(short, long)]
    pub src: String,

    ///The second folder to compare.
    #[arg(short, long)]
    pub dst: String,

    ///If passed, the list of files that matched will be shown.
    #[arg(long)]
    pub compared: bool,

    ///If passed, all files that is compared will be deleted from <DST>
    #[arg(long)]
    pub delete: bool,

    ///If passed, all files that is compared will be rename as <file_name>_deleted in <DST>
    #[arg(long)]
    pub postfix: bool,
}
