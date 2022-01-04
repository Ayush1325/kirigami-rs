const LIB_NAME: &str = "Kirigam2";

fn main() {
    eprintln!("cargo:warning={:?}", std::env::vars().collect::<Vec<_>>());

    kde_frameworks::link_lib(LIB_NAME).unwrap();
}
