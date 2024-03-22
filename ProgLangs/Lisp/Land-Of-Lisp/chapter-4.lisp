; lisp is imbued with symmetry

; empty list is false when evaluating a condition
(if '()
        'i-am-true
        'i-am-false)
; I-AM-FALSE

(if '(1)
        'i-am-true
        'i-am-false)
; I-AM-TRUE

; we can process lists via recursion since base case is empty list and it evals to false
(defun my-length (list)
        (if list 
                (1+ (my-length (cdr list)))
                0))
(my-length '(list with four symbols)) ; 4

; an empty list is the only false value in common list
; any value that is not equivalent to an empty list will be considered a true value
; () = '() = 'nil = nil

; exploring if conditions
; only one of the expressions after the if is actually evaluated
; we can only do one thing in an if statement
(if (= (+ 1 2) 3)
        'yup
        'nope) ; YUP

(if (oddp 5)
        'odd-number
        'even-number) ; ODD-NUMBER

; can use progn to wedge extra commands into a single expression
(defvar *number-was-odd* nil)
(if (oddp 5)
        (progn (setf *number-was-odd* t)
                        'odd-number)
        'even-number) ; ODD-NUMBER
; *number-was-odd*: t





