; lisp strives for a very basic syntax
; symbols: fundamental type of data in lisp, stand alone word
; symbols are case insensitive
(eq 'foo 'FoOo) ; this will evaluate to true

; supports floating point numbers and integers
; func with integer and floating point, integer will be "poisioned" and float returned
(+ 1 1.0) ; 2.0

; when dividing, will return rational number rather than floating point
(/ 4 6) ; 2/3

; if floating point in calc, we get floating point back
(/ 4.0 6) ; 0.666667

; indicate strings wtih double quotes
; use princ to display a string
(princ "Hello world")

; how does lisp distinguish between code and data?
; two modes: code mode and data mode
; when typing into REPL, compiler assumes you are entering command you want executed
; (command form...)
; form: a list with a special command at a beginning
; any stuff written in data mode is treated as data, compiler will not execute it
; single quote signals data, treat it as a list of items. called quoting
'(expt 2 3) ; signal quote tells the compiler that this is data

; lists in lisp are held together by cons cells
; cons cell: made of two connected boxes
; one can point to data and other can point to next part in list
; final cons cell points to "nil"
; three basic functions for list manipulation: cons, car, cdr

; cons can be used to link any two pieces of data in lisp program
; this is not a regular list, "." in the middle says that it is a cons cell
; nil is a special symbol that is used to terminate a list
; () can also be used in replace of nil
; a chain of cons cells and a list are exactly the same thing
; lists are just long chains of two item cells (cons)
(cons 'chicken 'cat) ; (CHICKEN . CAT)
(cons 'chicken 'nil)  ; (CHICKEN) 
(cons 'chicken ())     ; (CHICKEN)
(cons 'pork '(beef chicken)) ; (PORK BEFF CHICKEN)
(cons 'pork (cons 'beef (cons 'chicken ()))) ; the unsugared syntax

; car: used to get the thing out of the first slot of a cell
(car '(pork beef chicken)) ; PORK

; cdr: used to grab value out of second slot or rest of list
(cdr '(pork beef chicken)) ; (BEEF CHICKEN)

; can string together car and cdr
; cadr: takes the second element and then the first
(cadr '(pork beef chicken)) ; BEEF

; list: does the dirty work of creating all the cons cells
(list 'pork 'beef ' chicken) ; (PORK BEEF CHICKEN)

; lists can contain other lists
'(cat (duck bat) ant) ; this is a nested list

; taking things out of lists
(car '((peas carrots tomatoes) (pork beef chicken))) ; (PEAS CARROTS TOMATOES)
(cdr '(peas carrots tomatoes)) ; (CARROTS TOMATOES)
(cdr (car '((peas carrots tomatoes) (pork beef chicken)))) ; (CARROTS TOMATOES)
(cdar '((peas carrots tomatoes) (pork beef chicken))) ; (CARROTS TOMATOES)
(cddr '((peas carrots tomatoes) (pork beef chicken) duck)) ; (DUCK)
(caddr '((peas carrots tomatoes) (pork beef chicken) duck)) ; DUCK
(cddar '((peas carrots tomatoes) (pork beef chicken) duck)) ; (TOMATOES)
(cadadr '((peas carrots tomatoes) (pork beef chicken) duck)) ; BEEF

; can use any function with name c*r out of box up to four levels deep



