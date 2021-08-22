Utility library to work with tuples.

## Features:

* Test if all elements are `Ok`: **`all_ok()`**

  ```rust
  assert_eq!(
      all_ok((good(1), good(2), good(3))),
      Ok((1, 2, 3)),
  );
  assert_eq!(
      all_ok((good(1), bad(2), good(3))),
      Err((Ok(1), Err(2), Ok(3)))
  );
  ```

* Test if all elements are `Some`: **`all_some()`**

  ```rust
  assert_eq!(
      all_some((Some(1), Some(2), Some(3))),
      Ok((1, 2, 3))
  );
  assert_eq!(
      all_some((Some(1), Option::<()>::None, Some(3))),
      Err((Some(1), None, Some(3)))
  );
  ```

* Prepend an element to a tuple: **`prepend()`**

  ```rust
  assert_eq!(prepend(1, (2, 3, 4)), (1, 2, 3, 4));
  ```

* Append an element to a tuple: **`append()`**

  ```rust
  assert_eq!(append((1, 2, 3), 4), (1, 2, 3, 4));
  ```

* Concatenate two tuples: **`concat_tuples()`**

  ```rust
  assert_eq!(concat_tuples((1, 2), (3, 4, 5)), (1, 2, 3, 4, 5));
  ```

* Concatenate multiple tuples: **`concat_many()`**

  ```rust
  assert_eq!(concat_many(((), (1,), (2, 3,), (4, 5, 6))), (1, 2, 3, 4, 5, 6));
  ```

* Turn a reference to a tuple to a tuple of references: **`ref_tuple()`**

  ```rust
  assert_eq!(ref_tuple(&(1, 2, 3)), (&1, &2, &3));
  ```

* Turn a reference to a mutable tuple to a tuple of mutable references: **`tuple_ref_mut()`**

  ```rust
  assert_eq!(tuple_ref_mut(&mut (1, 2, 3)), (&mut 1, &mut 2, &mut 3));
  ```

* Extract the first element of a tuple: **`unprepend()`**

  ```rust
  assert_eq!(unprepend((1, 2, 3, 4)), (1, (2, 3, 4)));
  ```

* Extract the last element of a tuple: **`unappend()`**

  ```rust
  assert_eq!(unappend((1, 2, 3, 4)), ((1, 2, 3), 4));
  ```

 * Call a function with the tuple members as arguments: **`apply()`**

   `features = ["apply"]`, included by default

   ```rust
   fn add3(a: u32, b: u32, c: u32) -> u32 { a + b + c }

   let tpl3 = (1, 2, 3);
   assert_eq!(apply(&add3, tpl3), 6);
   ```

 * Element-wise wrap the element of a tuple in `Option`: **`option_tuple()`**

   `features = ["option"]`, included by default

   ```rust
   assert_eq!(option_tuple(Some((1, 2, 3))), (Some(1), Some(2), Some(3)));
   ```

* Get the length of a tuple: **`length()`**

  ```rust
  assert_eq!(<(u8, u16, u32) as TupleLength>::LENGTH, 3);
  ```

* Map a tuple: **`map_tuple()`**

  ```rust
  struct MyTupleEnum(usize);

  impl TupleMapper for MyTupleEnum {
      type MapElem<Type> = (usize, Type);
 
      fn map_elem<Elem>(&mut self, elem: Elem) -> Self::MapElem<Elem> {
          let index = self.0;
          self.0 += 1;
          (index, elem)
      }
  }

  assert_eq!(
      map_tuple(MyTupleEnum(1), ("hello", "world", "!")),
      ((1, "hello"), (2, "world"), (3, "!")),
  )
  ```

When used in libraries, you should probably use `default-features = false`, and only opt in
to the features you actually need.

## Supported tuple lengths:

By default the selected operations are implemented to tuples upto a length of 16 elements
(`features = ["default-len"]`).

You can specify a higher limit by using `feature = ["X"]`, where `X` can be
8, 16, 32, 64, 96, 128, 160, 192, 224, or 256. A higher number includes all lower numbers.

**Beware:** `features = ["256"]` needs about 5 GB of RAM to compile the module,
so only use it if you actually need it.
