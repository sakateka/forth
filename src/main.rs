use anyhow::{bail, Result};
use std::collections::HashMap;

use phf::phf_map;

#[derive(Clone, Debug)]
pub enum Keyword {
    Plus,
    Minus,
    Mul,
    Div,
    Over,
    Swap,
    Dup,
    Drop,
    UserCmd(String),
    Number(i64),
}

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "+" => Keyword::Plus,
    "-" => Keyword::Minus,
    "*" => Keyword::Mul,
    "/" => Keyword::Div,
    "dup" => Keyword::Dup,
    "over" => Keyword::Over,
    "drop" => Keyword::Drop,
    "swap" => Keyword::Swap,
};

pub struct Evaluator {
    definitions: HashMap<String, Vec<Keyword>>,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            definitions: HashMap::new(),
        }
    }

    pub fn process(&mut self, row: impl AsRef<str>) -> Result<Vec<i64>> {
        let mut commands_iter = row.as_ref().split_whitespace();

        let mut stack = Vec::new();
        let mut cmd_stack: Vec<Keyword> = Vec::new();

        loop {
            let Some(cmd) = commands_iter.next() else {
                return Ok(stack);
            };
            let mut keyword = self.parse_word(cmd)?;
            loop {
                match keyword {
                    Keyword::Number(n) => {
                        stack.push(n);
                        break;
                    }
                    Keyword::UserCmd(name) => {
                        if let Some(def) = self.definitions.get(name.as_str()) {
                            cmd_stack.append(def.clone().as_mut());
                        } else {
                            bail!("Unknown command: '{}'", name);
                        }
                    }
                    _ => self.evaluate(&mut stack, keyword)?,
                }
                keyword = if let Some(kw) = cmd_stack.pop() {
                    kw
                } else {
                    break;
                };
            }
        }
    }

    fn evaluate(&mut self, stack: &mut Vec<i64>, keyword: Keyword) -> Result<()> {
        match keyword {
            Keyword::Number(_) | Keyword::UserCmd(_) => bail!("unexpected keyword {:?}", keyword),
            Keyword::Drop => {
                let Some(_) = stack.pop() else {
                    bail!("too few arguments for '{:?}': {:?}", keyword, stack);
                };
            }
            Keyword::Dup => {
                let Some(num) = stack.pop() else {
                    bail!("too few arguments for '{:?}': {:?}", keyword, stack);
                };
                stack.extend_from_slice(&[num, num]);
            }
            _ => {
                if stack.len() < 2 {
                    bail!("too few arguments for '{:?}': {:?}", keyword, stack);
                }
                let arg2 = stack.pop().unwrap();
                let arg1 = stack.pop().unwrap();
                match keyword {
                    Keyword::Plus => stack.push(arg1 + arg2),
                    Keyword::Minus => stack.push(arg1 - arg2),
                    Keyword::Mul => stack.push(arg1 * arg2),
                    Keyword::Div => {
                        if arg2 == 0 {
                            bail!("attempt to divide by zero");
                        }
                        stack.push(arg1 / arg2);
                    }
                    Keyword::Over => stack.extend_from_slice(&[arg1, arg2, arg1]),
                    Keyword::Swap => stack.extend_from_slice(&[arg2, arg1]),
                    _ => unreachable!(),
                }
            }
        }
        Ok(())
    }

    fn parse_word(&self, word: &str) -> Result<Keyword> {
        match self.definitions.contains_key(word) {
            true => Ok(Keyword::UserCmd(word.into())),
            false => match KEYWORDS.get(word) {
                Some(kw) => Ok(kw.clone()),
                None => word
                    .parse::<i64>()
                    .map(Keyword::Number)
                    .map_err(|e| e.into()),
            },
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    println!("Here will be forth");
}

#[cfg(test)]
mod test {
    use crate::Evaluator;

    #[test]
    fn test_num() {
        let mut ev = Evaluator::new();
        let stack = ev.process("10");
        assert_eq!(vec![10], stack.unwrap());

        let stack = ev.process("1 2 3");
        assert_eq!(vec![1, 2, 3], stack.unwrap());

        let stack = ev.process("1 3 -2");
        assert_eq!(vec![1, 3, -2], stack.unwrap());
    }

    #[test]
    fn test_plus() {
        let mut ev = Evaluator::new();
        let stack = ev.process("1 2 +");
        assert_eq!(vec![3], stack.unwrap());

        let mut ev = Evaluator::new();
        let e = ev.process("2 +").unwrap_err();
        assert_eq!(e.to_string(), "too few arguments for 'Plus': [2]");
    }

    #[test]
    fn test_minus() {
        let mut ev = Evaluator::new();
        let stack = ev.process("1 2 -");
        assert_eq!(vec![-1], stack.unwrap());
        let stack = ev.process("10 2 -");
        assert_eq!(vec![8], stack.unwrap());
    }

    #[test]
    fn test_mul() {
        let mut ev = Evaluator::new();
        let stack = ev.process("3 2 *");
        assert_eq!(vec![6], stack.unwrap());
    }

    #[test]
    fn test_div() {
        let mut ev = Evaluator::new();
        let stack = ev.process("3 9 /");
        assert_eq!(vec![0], stack.unwrap());
        let stack = ev.process("9 3 /");
        assert_eq!(vec![3], stack.unwrap());

        let e = ev.process("9 0 /").unwrap_err();
        assert_eq!(e.to_string(), "attempt to divide by zero");
    }

    #[test]
    fn test_dup() {
        let mut ev = Evaluator::new();
        let stack = ev.process("3 dup");
        assert_eq!(vec![3, 3], stack.unwrap());
        let stack = ev.process("9 3 dup");
        assert_eq!(vec![9, 3, 3], stack.unwrap());
    }

    #[test]
    fn test_over() {
        let mut ev = Evaluator::new();
        let stack = ev.process("1 3 over");
        assert_eq!(vec![1, 3, 1], stack.unwrap());
        let stack = ev.process("1 2 3 over");
        assert_eq!(vec![1, 2, 3, 2], stack.unwrap());

        let e = ev.process("9 over").unwrap_err();
        assert_eq!(e.to_string(), "too few arguments for 'Over': [9]");
    }

    #[test]
    fn test_swap() {
        let mut ev = Evaluator::new();
        let stack = ev.process("1 3 swap");
        assert_eq!(vec![3, 1], stack.unwrap());
        let stack = ev.process("1 2 3 swap");
        assert_eq!(vec![1, 3, 2], stack.unwrap());

        let e = ev.process("9 swap").unwrap_err();
        assert_eq!(e.to_string(), "too few arguments for 'Swap': [9]");
    }

    #[test]
    fn test_big() {
        struct Case {
            description: &'static str,
            input: &'static [&'static str],
            expected: &'static [i64],
            is_err: bool,
        }
        let cases = vec![
            Case {
                description: "push numbers",
                input: &["1 2 3 4 5"],
                expected: &[1, 2, 3, 4, 5],
                is_err: false,
            },
            Case {
                description: "add",
                input: &["1 2 +"],
                expected: &[3],
                is_err: false,
            },
            Case {
                description: "nothing to add",
                input: &["+"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "add arity",
                input: &["1 +"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "sub",
                input: &["3 4 -"],
                expected: &[-1],
                is_err: false,
            },
            Case {
                description: "nothing to sub",
                input: &["-"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "sub arity",
                input: &["1 -"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "mul",
                input: &["2 4 *"],
                expected: &[8],
                is_err: false,
            },
            Case {
                description: "nothing to mul",
                input: &["*"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "mul arity",
                input: &["1 *"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "div",
                input: &["12 3 /"],
                expected: &[4],
                is_err: false,
            },
            Case {
                description: "integer division",
                input: &["8 3 /"],
                expected: &[2],
                is_err: false,
            },
            Case {
                description: "division by zero",
                input: &["4 0 /"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "nothing to div",
                input: &["/"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "div arity",
                input: &["1 /"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "add sub",
                input: &["1 2 + 4 -"],
                expected: &[-1],
                is_err: false,
            },
            Case {
                description: "mul div",
                input: &["2 4 * 3 /"],
                expected: &[2],
                is_err: false,
            },
            Case {
                description: "dup",
                input: &["1 dup"],
                expected: &[1, 1],
                is_err: false,
            },
            Case {
                description: "dup top",
                input: &["1 2 dup"],
                expected: &[1, 2, 2],
                is_err: false,
            },
            Case {
                description: "nothing to dup",
                input: &["dup"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "drop",
                input: &["1 drop"],
                expected: &[],
                is_err: false,
            },
            Case {
                description: "drop top",
                input: &["1 2 drop"],
                expected: &[1],
                is_err: false,
            },
            Case {
                description: "nothing to drop",
                input: &["drop"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "swap",
                input: &["1 2 swap"],
                expected: &[2, 1],
                is_err: false,
            },
            Case {
                description: "swap top",
                input: &["1 2 3 swap"],
                expected: &[1, 3, 2],
                is_err: false,
            },
            Case {
                description: "nothing to swap",
                input: &["swap"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "swap arity",
                input: &["1 swap"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "over",
                input: &["1 2 over"],
                expected: &[1, 2, 1],
                is_err: false,
            },
            Case {
                description: "over2",
                input: &["1 2 3 over"],
                expected: &[1, 2, 3, 2],
                is_err: false,
            },
            Case {
                description: "nothing to over",
                input: &["over"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "over arity",
                input: &["1 over"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "user-defined",
                input: &[": dup-twice dup dup ;", "1 dup-twice"],
                expected: &[1, 1, 1],
                is_err: false,
            },
            Case {
                description: "user-defined order",
                input: &[": countup 1 2 3 ;", "countup"],
                expected: &[1, 2, 3],
                is_err: false,
            },
            Case {
                description: "user-defined override",
                input: &[": foo dup ;", ": foo dup dup ;", "1 foo"],
                expected: &[1, 1, 1],
                is_err: false,
            },
            Case {
                description: "built-in override",
                input: &[": swap dup ;", "1 swap"],
                expected: &[1, 1],
                is_err: false,
            },
            Case {
                description: "built-in operator override",
                input: &[": + * ;", "3 4 +"],
                expected: &[12],
                is_err: false,
            },
            Case {
                description: "no redefinition",
                input: &[": foo 5 ;", ": bar foo ;", ": foo 6 ;", "bar foo"],
                expected: &[5, 6],
                is_err: false,
            },
            Case {
                description: "reuse in definition",
                input: &[": foo 10 ;", ": foo foo 1 + ;", "foo"],
                expected: &[11],
                is_err: false,
            },
            Case {
                description: "redefine numbers",
                input: &[": 1 2 ;"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "non-existent word",
                input: &["foo"],
                is_err: true,
                expected: &[],
            },
            Case {
                description: "DUP case insensitivity",
                input: &["1 DUP Dup dup"],
                expected: &[1, 1, 1, 1],
                is_err: false,
            },
            Case {
                description: "DROP case insensitivity",
                input: &["1 2 3 4 DROP Drop drop"],
                expected: &[1],
                is_err: false,
            },
            Case {
                description: "SWAP case insensitivity",
                input: &["1 2 SWAP 3 Swap 4 swap"],
                expected: &[2, 3, 4, 1],
                is_err: false,
            },
            Case {
                description: "OVER case insensitivity",
                input: &["1 2 OVER Over over"],
                expected: &[1, 2, 1, 2, 1],
                is_err: false,
            },
            Case {
                description: "user-defined case insensitivity",
                input: &[": foo dup ;", "1 FOO Foo foo"],
                expected: &[1, 1, 1, 1],
                is_err: false,
            },
            Case {
                description: "definition case insensitivity",
                input: &[": SWAP DUP Dup dup ;", "1 swap"],
                expected: &[1, 1, 1, 1],
                is_err: false,
            },
            Case {
                description: "redefine of builtin after define user function on it",
                input: &[": foo dup ;", ": dup 1 ;", "2 foo"],
                expected: &[2, 2],
                is_err: false,
            },
        ];

        use anyhow::{anyhow, Result};
        for case in cases {
            eprintln!("Run test for {}", case.description);
            let mut ev = Evaluator::new();
            let resp = || -> Result<Vec<i64>> {
                let mut ret = Err(anyhow!("empty input"));
                for row in case.input {
                    ret = Ok(ev.process(row)?);
                }
                ret
            }();
            if case.is_err {
                assert_eq!(case.is_err, resp.is_err());
            } else {
                assert_eq!(case.expected, resp.unwrap());
            }
        }
    }
}
