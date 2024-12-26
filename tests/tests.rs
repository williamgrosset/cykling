use std::collections::HashMap;

use cykling::*;

#[test]
fn test_regular_grammar() {
    let mut grammar = HashMap::new();

    grammar.insert("S".to_string(), vec![
        Rule::NonTerminal("A".to_string(), "B".to_string()),
    ]);
    grammar.insert("B".to_string(), vec![
        Rule::NonTerminal("B".to_string(), "C".to_string()),
        Rule::Terminal("b".to_string()),
    ]);
    grammar.insert("A".to_string(), vec![Rule::Terminal("a".to_string())]);
    grammar.insert("C".to_string(), vec![Rule::Terminal("c".to_string())]);

    assert!(cyk("abc", &grammar, "S"));
    assert!(!cyk("acb", &grammar, "S"));
}

#[test]
fn test_empty_grammar() {
    let mut empty_grammar = HashMap::new();
    empty_grammar.insert("S".to_string(), vec![Rule::Terminal("".to_string())]);
    assert!(cyk("", &empty_grammar, "S"));
}
