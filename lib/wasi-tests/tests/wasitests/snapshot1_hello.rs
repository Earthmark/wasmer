// !!! THIS IS A GENERATED FILE !!!
// ANY MANUAL EDITS MAY BE OVERWRITTEN AT ANY TIME
// Files autogenerated with cargo build (build/wasitests.rs).

#[test]
fn test_snapshot1_hello() {
    assert_wasi_output!(
        "../../wasitests/snapshot1/hello.wasm",
        "snapshot1_hello",
        vec![],
        vec![],
        vec![],
        "../../wasitests/hello.out"
    );
}
