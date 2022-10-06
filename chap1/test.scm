;; A test program, test.scm
;;
;;

(+ 1 2340)

(define x 120)

(define (example x y z)
  (if (> x 0)
      (+ y z)
      (- y z)))

(define ss "This is longer string.")
(string-length ss)
(string-ref ss 2)
(define s "This is a.")
;; (print "chicken test")

;; (define buggy-remove
;;   (lamda (x ls)
;; 	 (if (null? x)
;; 	     '()
;; 	     (if (equal? (car ls) x)
;; 		 (buggy-remove x (cdr ls))
;; 		 (cons (car ls) (buggy-remove x (cdr ls)))))))

(define buggy-remove
  (lambda (x ls)
    (if (null? ls)
	'()
	(if (eq? (car ls) x)
	    (buggy-remove x (cdr ls))
	    (cons (car ls)
		  (buggy-remove x (cdr ls)))))
    )
  )


