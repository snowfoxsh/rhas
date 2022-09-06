use std::{str::Chars, collections::binary_heap::Iter};

#[allow(dead_code)]
const OPERATORS: [&str; 23] = [
    "(", ")",
    "[", "]",
    "{", "}",

    "`",
    ",",
    "\"",
    "\\",
    ";",
    "=",

    "+", "-",
    "*", "/",

    "%",
    "^",
    "~",
    "!",
    "&",
    "|",
    ":",
    
    // "==",
    // "!=",
    // "<",
    // ">",
    // "<=",
    // ">=",
    
    // "<<",
    // ">>",
    // ">>>",
    // "<<<",
];



enum TokenType {
    Null,
    Delimiter,
    Number,
    Symbol,
    Unknown,
}


struct SyntaxTree {
        
}

impl SyntaxTree {
    fn new(input: String) -> Self {
        SyntaxTree {  

        }
    }
}

fn syntax_tree(input: String) -> SyntaxTree {
    SyntaxTree::new(input)
}


struct TreeParser {
    // exp: &'a str;
    exp: &'static str,
    it:  Chars<'static>,
}

impl TreeParser {
    fn new(input: &'static str ) -> Self {
        TreeParser {
            exp: input,
            it: input.chars()
        }
    }

    fn parse(mut self) {
    println!("{:?}", self.it.filter(|&x| x == 'l').collect::<String>());
    
    // println!("{:?}", self.exp.chars().filter(|&x| x == 'l'))
    // println!("{:?}", self.it.next());
    }
}

impl Iterator for TreeParser {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }
}


#[test]
fn testTreeParse () {
    let mut walker = TreeParser::new("string");

    walker.next();

}