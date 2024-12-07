# rdiff
Compare two folders and do some manipulations with files

# Installation
## Debian/Ubuntu
 - Download the package from [releases list](https://github.com/AleksandrCherepanov/rdiff/releases).
 - Install the package running 
    ```bash 
    sudo dpkg -i <downloaded_package>
    ```
## From source
 To compile the code you need to have `rustc`. [How to install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) 
 - Clone repository
 ```bash
 git clone git@github.com:AleksandrCherepanov/rdiff.git
 ```
 - Go to cloned directory and build it
 ```bash
 cd rdiff && cargo build --release
 ```
 - The binary file will be available in `target/release/rdiff` 
