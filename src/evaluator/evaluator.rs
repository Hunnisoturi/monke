#[cfg(test)]
mod test {
    use crate::lexer::lexer::*;
    use crate::object::object::*;

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![("5", 5), ("10", 10)];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            assert!(test_integer_object(&evaluated, expected));
        }
    }

    fn test_integer_object(obj: &Object, expected: i64) -> bool {
        match obj {
            Object::Integer(integer) => {
                if integer.value != expected {
                    eprintln!(
                        "object has wrong value. got={}, want={}",
                        integer.value, expected
                    );
                    return false;
                }
                true
            }
            _ => {
                eprintln!("object is not Integer. got={:?}", obj.inspect());
                false
            }
        }
    }

    fn test_eval(input: &str) -> Object {
        let _lexer = Lexer::new(input.into());
        Object::Null
    }
}

//TODO: Separate Lexer and Parser, make Token include literal
//      otherwise this shit won't work
