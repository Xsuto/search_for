# search_for
multithreaded find command written in rust.
# How to build
cargo build --release
add ./target/release/search_for to $PATH or run directly
# How to run
search_for DIR Options

# Options
-n --name files that program will look for example "main.cpp,*.rs"
-e --excluded_dirs program won't waste time on those dirs for example node_modules
