extern crate cc;
use walkdir::WalkDir;

fn main() {
    compile_and_link_c_files();
}

fn compile_and_link_c_files() {
    let mut build = cc::Build::new();

    // Recursively traverse all files add them to the compiler.
    for entry in WalkDir::new("c_lib")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .filter(|e| {
            let fname = e.file_name().to_str().unwrap_or_default();
            fname.ends_with(".c")
        })
    {
        build.file(entry.path());
    }

    build.include("c_lib").static_flag(true);

    // Finally compile all c files
    build.compile("c.lib");
}
