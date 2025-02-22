# A lisp in rust

## Special forms
- [x] if: `(if true 1 2)`
- [x] quote: `(quote (1 2 3))` <!-- literal form of next s-exp -->
- [x] do: `(do expr*)`
- [x] def: `(def name val)` <!-- Defined only in the current scope -->
- [x] set: `(set name val)`
- [ ] function: `(function (arg*) returnExpr)`
- [ ] defmacro: `(macro name func)`

## Builtin functions
- [x] car/cdr: `(car (quote (1 2 3))) => 1`
- [x] cons: `(cons 1 '(2 3)) => '(1 2 3)` <!-- Head of this is 1, tail is (2 3) -->
- [x] +-*/: overloaded

## Native functions
- [ ] open/close: for files
- [ ] open/close/bind: for sockets
- [ ] something: for threads
- [ ] something: for time


## Macro forms
- [ ] fn: `(fn (arg1 arg2) expr*) -> (function (arg1 arg2) (do expr*))
- [ ] defn: `(defn name (args) body) -> (def name (fn (args) body)`
- [ ] cond: `(cond ((a b) (c d))) -> (if a b (if c d))`
- [ ] car/cdr: large combinations of caaddaadar
