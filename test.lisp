; (defvar cube 5)
(defmacro cube (n)

  (defun s (n) (print n))
  (let ((x (gensym)))
    `(let ((,x ,n)) 
        (* ,x ,x ,x)))

  )
     
(defmacro cubes () print)   


((CUBES) 7)
