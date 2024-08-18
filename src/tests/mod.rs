#[cfg(test)]
mod tests {
    #[test]
    fn hehe() {
        use crate::prelude::*;
        lexer_builder!(ignore_space: true);
        symbols!();
        keywords!("and");
        let mut lexer = AtlasLexer::default();
        lexer
            .set_path("<stdin>")
            .set_source(String::from("256245.325 , ; 25 { } and"));
        match lexer.tokenize() {
            Ok(toks) => {
                for t in toks {
                    println!("{:?}", t);
                }
            }
            Err(e) => {
                println!("Doesn't work lil bro {:?}", e);
            }
        }
    }
}
