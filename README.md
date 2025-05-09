# cykling

A `cyk` function that implements the [CYK algorithm](https://en.wikipedia.org/wiki/CYK_algorithm) for context-free grammars in Chomsky normal form.

### Example

Consider the context-free grammar (G) with the following production rules.

```
S -> AB | BC
A -> BA | a
B -> CC | b
C -> AB | a
```

E.g., strings `aabab ∈ L(G)` and `bababb ∉ L(G)`.

## Docs

```
cargo doc
```

## Development

```
cargo check
cargo run
```

## Tests

```
cargo test
```

## License

MIT
