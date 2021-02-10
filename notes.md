# Notes

## Variable ownership in parameter passing

* **Without parameter borrowing (moves str's ownership to called function)**

  ```rust
  let str = String::from("Hello");
  function(str);
  // str cannot be used in this scope from this line onwards
  ```

* **With parameter borrowing, without mutability**

  `str` can still be used in the scope it was declared in, but `function()`
  cannot change its value.  

  ```rust
  let str = String::from("Hello");
  function(&str);
  // str can still be used in this scope, because function()
  returned the ownership of str to this scope after returning
  ```

* **With parameter borrowing and mutability**

  Makes `str` mutable, so its value can be changed in `function`.

  ```rust
  let mut str = String::from("Hello");
  function(&mut str);
  // str is now "Hello, world!"
  .
  .
  fn function(str: &mut String){
    str.push(", world!");
  }
  ```
