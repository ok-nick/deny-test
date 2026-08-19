// This crate exists only to exercise the cargo-deny dependency-audit workflow.
// It is deliberately never built or run in CI.
fn main() {
    println!("deny-test: a fixture for the dependency-audit workflow");
}
