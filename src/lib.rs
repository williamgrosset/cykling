use std::collections::{HashMap, HashSet};

// Grammar rule in Chomsky Normal Form
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Rule {
    Terminal(String),
    NonTerminal(String, String),
}

// Determine if a string can be derived from a grammar
pub fn cyk(input: &str, grammar: &HashMap<String, Vec<Rule>>, start_symbol: &str) -> bool {
    let n = input.len();

    if n == 0 {
        if let Some(rules) = grammar.get(start_symbol) {
            for rule in rules {
                if let Rule::Terminal(term) = rule {
                    if term.is_empty() {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    let mut table: Vec<Vec<HashSet<String>>> = vec![vec![HashSet::new(); n]; n];

    for (i, symbol) in input.chars().enumerate() {
        for (non_terminal, rules) in grammar {
            for rule in rules {
                if let Rule::Terminal(term) = rule {
                    if term == &symbol.to_string() {
                        table[i][i].insert(non_terminal.clone());
                    }
                }
            }
        }
    }

    for length in 2..=n {
        for start in 0..=n - length {
            let end = start + length - 1;

            for split in start..end {
                for (non_terminal, rules) in grammar {
                    for rule in rules {
                        if let Rule::NonTerminal(lhs, rhs) = rule {
                            if table[start][split].contains(lhs) && table[split + 1][end].contains(rhs) {
                                table[start][end].insert(non_terminal.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    table[0][n - 1].contains(start_symbol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cyk_algorithm() {
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

        let mut empty_grammar = HashMap::new();
        empty_grammar.insert("S".to_string(), vec![Rule::Terminal("".to_string())]);
        assert!(cyk("", &empty_grammar, "S"));
    }
}
