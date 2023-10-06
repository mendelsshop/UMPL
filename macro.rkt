#lang racket
(define-syntax unquoted
  (syntax-rules ()
    [(unquoted   (a ...) ...)
      (let () (displayln a) ... ... ) ]))
(define-syntax rotate
  (syntax-rules ()
    [(rotate a c ...)
     (shift-to (c ... a) (a c ...))]))
 
(define-syntax shift-to
  (syntax-rules ()
    [(shift-to (from0 from ...) (to0 to ...))
     (let ([tmp from0])
       (set! to from) ...
       (set! to0 tmp))]))
(unquoted ( 4 6)  (5 6))
(define-syntax defmacro
  (syntax-rules ()
    [(_  name [(a ...) b ...] ...)
      (define-syntax name
        (syntax-rules ()
          [(_ a ...) (begin b ... )] ...))]))

(defmacro name
  [(c ... )
   (displayln (car c)) ... ])

(defmacro infix
  ;paren
  [((a ...)) (infix a ...)]
  [(a '/ b ...) (/ (infix a) (infix b ...))]
  [(a '* b ...) (* (infix a) (infix b ...))]
  [(a '- b ...) (- (infix a) (infix b ...))]
  [(a '+ b ...) (+ (infix a) (infix b ...))]
  ; kinda of buggy
  [(a '^ b ...) (expt (infix a) (infix b ...))]
  ; base case
  [(a) a]
) 

(defmacro infix-e
  ;paren
  [((a ...)) (infix-e a ...)]
  [(a '/ b ...) `(/ ,(infix-e a) ,(infix-e b ...))]
  [(a '* b ...) `(* ,(infix-e a) ,(infix-e b ...))]
  [(a '- b ...) `(- (infix-e a) ,(infix-e b ...))]
  [(a '+ b ...) `(+ ,(infix-e a) ,(infix-e b ...))]
  [(a '^ b ...) `(^ ,(infix-e a) ,(infix-e b ...))]
  ; base case
  [(a) a]
) 

(defmacro -> [((name `-> x) ...) (define name x) ...])
(name  '('- 2) '('+ 3))

(infix-e  2 '- (8 '* 8) `^ 5 '+ 6)

(-> (a `-> 1) (b '-> 2))
a
b