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

; there are ways to do a implicit progn
; if the conditions are the opposite they cannot do anything and just return nil

; enclosed expressions are evaluated when the condition is ture
(defvar *number-is-odd* nil)
(when (oddp 5)
        (setf *number-is-odd* t)
        'odd-number) ; ODD-NUMBER
; *number-is-odd*: t

; enclosed expressions are evaluated when the condition is false
(unless (oddp 4)
        (setf *number-is-odd* nil)
        'even-number) ; EVEN-NUMBER
; *number-is-odd*: NIL

; cond is the classic way of doing branching
; implicit progn, multiple branches, eval several conditions
(defvar *arch-enemy* nil)
 (defun pudding-eater (person)
        (cond ((eq person 'henry) (setf *arch-enemy* 'stupid-lisp-alien)
                        '(curse you lisp alien – you ate my pudding))
                    ((eq person 'johnny) (setf *arch-enemy* 'useless-old-johnny)
                        '(i hope you choked on my pudding johnny))
                    (t  '(why you eat my pudding stranger ?))))

; can also use cases
; uses eq for comparisons, branching on symbols
; cannot be used to branch on string values and others
(defun pudding-eater (person)
        (case person   
                ((henry) (setf *arch-enemy* 'stupid-lisp-alien)
                                '(curse you lisp alien - you ate my pudding))
                ((johnny) (setf *arch-enemy* 'useless-old-johnny)
                                '(I hope you choked on my pudding johnny))
                ((otherwise '(why you eat my pudding stranger? )))))

; some tricks with and and or
; can set global var to true if a condition
; demonstrated short circuiting
(defparameter *is-it-even* nil)
(or (oddp 4) (setf *is-it-even* t))  



