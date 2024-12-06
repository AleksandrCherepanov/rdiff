pub fn common(
    src_count: usize,
    dst_count: usize,
    diff_count: usize,
    compared_count: usize
) {
    println!("Number of files in the first dir: {}", src_count);
    println!("Number of files in the second dir: {}", dst_count);
    println!("Number of different files: {}", diff_count);
    println!("Number of compared files: {}", compared_count);
}

pub fn compared(list: &Vec<String>) {
    println!();
    println!("Compared files:");
    for f in list {
        println!("{}", f);
    }
}
