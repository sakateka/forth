use anyhow::Result;
use std::collections::HashMap;

use phf::phf_map;

#[derive(Clone)]
pub enum Keyword {
    Plus,
    Minus,
    Mul,
    Div,
    Dup,
    Over,
    Drop,
    Swap,
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
    stack: Vec<i64>,
    definitions: HashMap<String, Vec<Keyword>>,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            stack: Vec::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn process(row: impl AsRef<str>) -> Result<Vec<i64>> {
        Ok(Vec::new())
    }

    pub fn parse_word(&self, word: &str) -> Result<Keyword> {
        match self.definitions.contains_key(word) {
            true => Ok(Keyword::UserCmd(word.into())),
            false => match KEYWORDS.get(word) {
                Some(kw) => Ok(kw.clone()),
                None => word
                    .parse::<i64>()
                    .map(Keyword::Number)
                    .map_err(|er| er.into()),
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
