# HashMap

- A hash map is a key value map, similar to unordered_map of STL. It takes a key, finds the hash of it, stores the data at the hash value. As a result, access to it becomes O(1).
- The APIs are similar to Vector with few variations.
- 

## Create and access

- First we need to import the collection classes. Vector doesn't need to import manually but other collections do.
  > use std::collections::HashMap;
- Similar to vector, the APIs to create it is new(), with_capacity() and from(). While using from(), the array of data is array of tuples. The APIs to check size is len(), capacity() etc.
  ```
  let my_map:HashMap<i32, f32> = HashMap::new();   // empty map

  println! ("capacity = {} ", my_map.capacity());
  println! ("len = {} ", my_map.len());
  println! ("is empty = {} ", my_map.is_empty());
 
  let my_map2:HashMap<i32, String> = HashMap::with_capacity(100);   // at least 100 capacity
  println! ("capacity = {} ", my_map2.capacity());
  println! ("len = {} ", my_map2.len());
  println! ("is empty = {} ", my_map2.is_empty());

  let my_map3:HashMap<i32, f32> = HashMap::from([
        (4, 5.6),
        (33, 98.5)
    ]);
  println! ("capacity = {} ", my_map3.capacity());
  println! ("len = {} ", my_map3.len());
  println! ("is empty = {} ", my_map3.is_empty());
  ```

## Iterators

  ```
  println! ("Keys : ");
  for key in my_map3.keys() {
      println!("{}", key);
  }
  println! ("Values : ");
  for val in my_map3.values() {
      println!("{}", val);
  }
  println! ("Mutable values : ");
  for val in my_map3.values_mut() {
      *val = *val + 1.0;
      println!("{}", val);
  }
  println! ("Iterator : ");
  for (key, val) in my_map3.iter() {
      println!("{} {}", key, val);
  }
  ```

## Iterator to vector
