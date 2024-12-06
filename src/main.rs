mod output;
mod file_system;
mod cli;

use std::collections::HashMap;
use clap::Parser;
use crate::file_system::Flags;


fn main() {
    let args = cli::Cli::parse();
    let flags = Flags::from_cli(&args);

    let src = file_system::parse_path(&args.src);
    let dst = file_system::parse_path(&args.dst);

    let mut src_files: HashMap<String, bool> = HashMap::new();
    let exceptions: Vec<&str> = vec![dst.to_str().unwrap()];

    let src_traverser = file_system::Traverser::new(&exceptions, false);
    let src_count = src_traverser.dir_traverse(src.to_str().unwrap(), &mut src_files);

    let mut dst_files: HashMap<String, bool> = HashMap::new();
    let exceptions: Vec<&str> = vec![];

    let dst_traverser = file_system::Traverser::new(&exceptions, true);
    let dst_count = dst_traverser.dir_traverse(dst.to_str().unwrap(), &mut dst_files);

    let (diff, compared) = file_system::dir_compare(&src_files, &dst_files, &flags);

    output::common(src_count, dst_count, diff, compared.len());

    if flags.compared {
        output::compared(&compared);
    }
}
