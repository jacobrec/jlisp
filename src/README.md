# A lisp in rust

## Special forms
- [x] if: `(if true 1 2)`
- [ ] def: `(def name val)` <!-- Defined only in the current scope -->
- [ ] function: `(function (arg1 arg2) return)`
- [ ] do: `(do expr*)`
- [ ] set: `(set name val)`
- [ ] quote: `(quote (1 2 3))` <!-- literal form of next s-exp -->

## Builtin functions
- [ ] car: support any arbitrary combo of caaddaadar
- [ ] cdr: see above
- [x] +-*/: overloaded


## Macro forms
- [ ] fn: `(fn (arg1 arg2) expr*) -> (function (arg1 arg2) (do expr*))
- [ ] defn: `(defn name (args) body) -> (def name (fn (args) body)`
- [ ] cond: `(cond ((a b) (c d))) -> (if a b (if c d))`

