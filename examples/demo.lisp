; this is a macro
(defmacro defn (name arg body) (def name (fn arg body)))

; a simple loop 
(def i 1)
(while
  (< i 10)
  (begin
    (probe i)
    (def i (+ i 1))))

; a nested data struct
(def mystruct 
  {@person 
    {@name @kaka
     @value 3.14}
   @addr
    {@name @haha
     @value 1024}})

; a lambda
(defn print x 
    (begin
      (probe @run_in_prin)
      (probe x)
      (probe (list (' a) (' b)))))
(print mystruct)

; a clousure demo
(defn OneObject (x) 
    (begin
      (defn internalfn () 
        (begin
          (probe x)
          (probe yy)))      ;; yy is defind in local env
      (def yy 3.14)
      {@myfn    internalfn
       @value   yy}))

(def obj (OneObject 1024))
((@myfn obj))
(probe obj)

(+ obj 1)
