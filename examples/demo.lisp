; this is a macro
(defmacro defn (name arg body) (def name (fn arg body)))

; a simple loop 
(def i 1)
(while
  (< i 10)
  (begin
    (probe i)
    (def i (+ i 1))))

; a complex data struct
(probe 
  {@person 
    {@name @kaka
     @value 3.14}
   @addr
    {@name @haha
     @value 1024}})




