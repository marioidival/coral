# Grammar

## Literals

* `Numbers`
* `Strings`
* `Booleans`
* `None`


## Unary expressions

* `not`


## Binary expressions

### Arithmetic

* `+`
* `-`
* `/`
* `*`

### Logic

* `==`
* `>`
* `<`
* `>=`
* `<=`

## Parentheses for grouping

```python
1 - (2 * 3) < 4 == False
```

## Notation
```
expression → literal
    | unary
    | binary
    | grouping

literal → NUMBER | STRING | "True" | "False" | "None"
grouping → "(" expression ")"
unary → "not" expression
binary → expression operator expression
operator → "==" | "not" | "<" | ">" | ">=" | "<=" | "+" | "-" | "/" | "*"
```

