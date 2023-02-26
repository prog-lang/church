# Type System

```text
|- n : Int (int literal)

 Г |- x : T   Г |- y : R
-------------------------- (function)
Г |- x -> y : Func(T, R)

Г |- f : Func(T, R)   Г |- x : T
-------------------------------- (application)
         Г |- f x : R
```
