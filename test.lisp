
(defmacro cube (n)


  (let ((x (gensym)))
    `(let ((,x ,n)) 
        (* ,x ,x ,x))))

(print (cube 5))

