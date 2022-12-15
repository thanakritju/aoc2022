#[derive(Debug, Clone)]
pub enum GrammarItem {
    Comma,
    Number(u8),
    Paren,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub value: Option<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexItem {
    Paren(char),
    Comma(char),
    Num(u8),
}

impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            value: None,
        }
    }
}

pub fn parse(input: &String) -> Result<ParseNode, String> {
    let tokens = lex(input)?;
    parse_expr(&tokens, 0).and_then(|(n, i)| {
        if i == tokens.len() {
            Ok(n)
        } else {
            Err(format!(
                "Expected end of input, found {:?} at {}",
                tokens[i], i
            ))
        }
    })
}

fn parse_expr(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (mut node, next_pos) = parse_expr(tokens, pos)?;
    let c = tokens.get(pos);
    match c {
        Some(lexItem) => match lexItem {
            LexItem::Paren('[') => {
                let (rhs, i) = parse_expr(tokens, next_pos + 1)?;
                node.children.push(rhs);
                Ok((node, i))
            }
            LexItem::Paren(']') => Ok((node, next_pos)),
            LexItem::Comma(',') => Ok((node, next_pos + 1)),
            LexItem::Num(n) => {
                let mut child = ParseNode::new();
                child.value = Some(*n);
                node.children.push(child);
                Ok((node, next_pos))
            }
            _ => panic!("unregonized data"),
        },
        None => Ok((node, next_pos)),
    }
}

fn lex(input: &String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                let n = get_number(c);
                result.push(LexItem::Num(n));
            }
            '[' | ']' => {
                result.push(LexItem::Paren(c));
                it.next();
            }
            ',' => {
                result.push(LexItem::Comma(c));
                it.next();
            }
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }
    Ok(result)
}

fn get_number(c: char) -> u8 {
    let mut number = c
        .to_string()
        .parse::<u8>()
        .expect("The caller should have passed a digit.");
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number() {
        assert_eq!(get_number('1'), 1);
    }

    #[test]
    fn test_parse() {
        let string = String::from("[]");
        assert_eq!(parse(&string).unwrap(), ParseNode::new());
        let string = String::from("[4]");
        let mut n = ParseNode::new();
        n.value = Some(4);
        assert_eq!(parse(&string).unwrap(), n);
    }

    #[test]
    fn test_lex() {
        let string = String::from("[[1],[2,3,4]]");
        let mut out = lex(&string).unwrap().into_iter();
        assert!(out.next().unwrap() == LexItem::Paren('['));
        assert!(out.next().unwrap() == LexItem::Paren('['));
        assert!(out.next().unwrap() == LexItem::Num(1));
        assert!(out.next().unwrap() == LexItem::Paren(']'));
        assert!(out.next().unwrap() == LexItem::Comma(','));
        assert!(out.next().unwrap() == LexItem::Paren('['));
        assert!(out.next().unwrap() == LexItem::Num(2));
        assert!(out.next().unwrap() == LexItem::Comma(','));
        assert!(out.next().unwrap() == LexItem::Num(3));
        assert!(out.next().unwrap() == LexItem::Comma(','));
        assert!(out.next().unwrap() == LexItem::Num(4));
        assert!(out.next().unwrap() == LexItem::Paren(']'));
        assert!(out.next().unwrap() == LexItem::Paren(']'));
    }
}
