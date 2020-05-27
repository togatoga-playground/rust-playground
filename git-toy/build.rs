fn main() {
    let path = std::env::current_dir().unwrap();
    println!(
        "cargo:rustc-link-search=native={}/libgit2/build",
        path.display()
    );
}
