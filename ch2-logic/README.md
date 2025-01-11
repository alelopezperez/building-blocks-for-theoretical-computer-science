# Logic Evaluator
The logic evaluator will generate truth table for a given expression, tautology check of any simple or compound proposition or predicate

## Truth Function or Operators
- Conjunction(and): `^`
- Disjunction(or): `v`
- Negation: `-`
- Material Implication: `->`
- Converse Implication: `<-`
- Biconditional(if and only if): `<->`

## Grammar
expr := binary
binary := unary (["^"|"v"] unary)*
unary := "-" expr | prposition
proposition := ("a".."u"|"w".."z")* 
