mod token;

fn its_true() -> bool {
    true
}

#[test]
fn it_works() {
    let result = its_true();
    assert!(result);
}