use std::collections::{HashMap, HashSet};

/// Grammar rule
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Rule {
    Terminal(String),
    NonTerminal(String, String),
}

/// CYK algorithm determines if a string can be derived from a context-free grammar in
/// Chomsky normal form.
///
/// # Arguments
///
/// * `input` - The string to derive.
/// * `grammar` - The hash map of non-terminal symbols mapped to vectors of production rules
///   in Chomsky normal form. A production rule can be either a:
///     - Terminal rule (e.g., `A -> 'a'`)
///     - Non-terminal rule (e.g., `A -> BC`)
/// * `start_symbol` - The start symbol of the grammar (e.g., `S`).
///
/// # Returns
///
/// A boolean indicating whether the input string can be derived from the grammar.
/// - Returns `true` if the input string can be derived.
/// - Returns `false` otherwise.
pub fn cyk(input: &str, grammar: &HashMap<String, Vec<Rule>>, start_symbol: &str) -> bool {
    let n = input.len();

    if n == 0 {
        // Check for start symbol and empty string grammar rule (e.g., S -> ε)
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
