use std::str::Chars;

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