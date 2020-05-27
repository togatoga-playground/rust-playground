use std::os::raw::{c_char, c_int, c_uchar};
#[link(name = "git2")]
extern "C" {
    pub fn git_libgit2_init() -> c_int;
}
