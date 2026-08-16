fn main() {
    // Rebuild when migration files are added/edited so the embedded
    // migrator stays in sync.
    println!("cargo:rerun-if-changed=migrations");
}
