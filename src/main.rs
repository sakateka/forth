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

pub fn parse_keyword(keyword: &str) -> Option<Keyword> {
    KEYWORDS.get(keyword).cloned()
}

pub struct Evaluator {
    stack: Vec<i64>,
    definitions: HashMap<String, Vec<String>>,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            stack: Vec::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn process(row: impl AsRef<str>) -> Result<Vec<i64>, anyhow::Error> {
        Ok(Vec::new())
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
