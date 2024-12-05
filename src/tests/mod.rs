#[cfg(test)]
mod tests {
    #[test]
    fn test_macros() {
        use crate::prelude::*;
        lexer_builder!(
            DefaultSystem {
                number: true,
                symbol: true,
                keyword: true,
                string: true,
                whitespace: {
                    allow_them: true,
                    use_system: true,
                },
            },
            Symbols {
                Single {
                    '.' => Dot,
                },
                Either {
                    ':' => ':' => DoubleColon, Colon
                }
            },
            Keyword { },
            Number {
                trailing {"_i8" => i8 => I8},
                float: true,
                u_int: true,
                int: true
            }
        );
    }
}
