fn main() {
    println!("cargo:rerun-if-changed=src/sql.lalrpop");
    lalrpop::process_root().unwrap();
}
