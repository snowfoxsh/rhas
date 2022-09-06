mod token;
mod lexer;
mod tree_parser;

fn its_true() -> bool {
    true
}

#[test]
fn it_works() {
    let result = its_true();
    assert!(result);
}