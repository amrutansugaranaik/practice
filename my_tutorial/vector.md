# Vector

- A growable array. Basically has an initial size (could be 0). When an element is added, a new memory location is created with almost double size or so and the original array is copied to it. Then the first array is removed from memory. Usually the capacity of vector is a power of 2.
-  Even though if requires reallocation and copying of data when it is full, the amortized analysis shows it runs fast in real life scenario.
- A vector has three elements, a pointer to data, length of data and capacity.
- It supports adding and deleting entry from one end like a stack.
- It is allocated in heap.

## Creating a Vector

- Create an empty vector with size is 0. Here the type is not required. On first initialization, type can be inferred by the compiler. Adding type is not an error too.
  > let mut data:Vec\<i32> = Vec::new();
- Create a vector using macro with default values
  > let data2 = vec![1, 2, 3];
- Create empty vector using macro. Since the vector is empty, its type must be explicitly mentioned.
  > let data2:Vec\<i32> = vec![];
- Creating from another vector. Since vector is using heap, we need to clone the content, otherwise ownership will be transferred.
  > let data3 = Vec::from(data.clone());
- Transfering ownership of vector
    ```
    let data3 = data;
    // println! ("size = {} ", data.len());  // error since ownership is over
    println! ("size = {} ", data3.len());
    ```
- Creating from an array
  > let data4:Vec\<i32> = Vec::from([5,6,7]);

### Complete code with vector creation :

```

fn main() {

    // Create vector using new()
    let mut data:Vec<i32> = Vec::new();
    println! ("Initial size = {} ", data.len());

    // Create vector using macro
    let data2:Vec<i32> = vec![];
    println! ("Initial size in macro = {} ", data2.len());

    // Add new data at the end
    data.push (5);
    data.push (6);

    // Print the size
    println! ("Size after adding two elements = {} ", data.len());

    // print the content of the vector
    println! ("{} {}", data[0], data[1]);

    // let data3 = Vec::from(data.clone());
    let data3 = data;
    // println! ("size = {} ", data.len());  // error since ownership is over
    println! ("size = {} ", data3.len());

    let data4:Vec<i32> = Vec::from([5,6,7]);
    
}
```

## Access a Vector

- Since vector is an array, we can access its elements like an array and accessing out of index will cause panic.
  > println! ("{} {}", data[0], data[1]);
- Access size of vector
  > println! ("Size= {} ", data.len());
- Access capacity of vector
  > println! ("capacity= {} ", data.capacity());
- Check if vector is empty
  > data.is_empty(); // returns bool
- Add an element at the end
  > data.push (5);
- Insert an element at random space. Index must be valid, else it will panic.
  > data.insert (2, 100);  // index, value
- Remove an element at random space. Index must be valid, else it will panic.
  > data.remove (2);
- Extending one vector to another. Both vectors are available after extend.
  ```
  let data2:Vec<i32> = Vec::from([5,6,7]);
  data.extend (data2.clone());
  data.extend (Vec::from([11,12,13]));
  ```
- Append one vector to another. Second vector is emptied.
  > data.append (data2);
- Remove all elements
  > data.clear()
- Deleting from an end (same end where new entry is added). pop() method deletes an element at the end and returns an Option<> which has some element if vector is not empty, else return None.
  ```
  let d:Option<i32> = data.pop();
    if d.is_some() {
        println! ("popped data = {} and new size of vector = {}", 
            d.unwrap(), data.len());
    } else {
        println! ("vector is empty ");
    }
  ```

### Complete code for accessing vector :

```
fn main() {

    // Create vector using macro
    let mut data:Vec<i32> = vec![2, 3, 5];
    println! ("Size= {} ", data.len());  // print 3
    println! ("capacity= {} ", data.capacity());  // print 3

    // Add new data at the end
    data.push (5);
    data.push (6);

    // print the content of the vector
    println! ("{} {}", data[0], data[1]); // print 5 6

    // Appending one vector to another
    let data2:Vec<i32> = Vec::from([5,6,7]);
    data.extend (data2.clone()); // must be cloned since Vec is in heap
    println! ("Size = {} ", data2.len()); // print 3
    println! ("Size = {} ", data.len());  // print 8

    // Directly extend from static array
    data.extend (Vec::from([11,12,13]));
    println! ("Size = {} ", data.len());  // print 11

    // Deleting an element at the end
    let d:Option<i32> = data.pop();
    if d.is_some() {
        println! ("popped data = {} and new size of vector = {}", 
            d.unwrap(), data.len()); // print 13 10
    } else {
        println! ("vector is empty ");
    }

    println! ("data[2] = {}", data[2]);
    data.insert (2, 100);
    println! ("data[2] = {}", data[2]);
    println! ("data[3] = {}", data[3]);

    println! ("data[2] = {}", data[2]);
    data.remove (2);
    println! ("data[2] = {}", data[2]);
    println! ("data[3] = {}", data[4]);
}
```

## Vector resizing related API

- Creating a vector with a predefined size
  > vec = Vec::with_capacity(5);
- Add an additional capacity compared to current len (not current capacity).
  > data.reserve_exact (100) // len + 100 size.
- Add an additional capacity which is atleast more than the requested amount
  > data.reserve (100)  // len + 100 or more
- Remove extra capacity more than len. So it doesn't take extra spaces.
  > data.shrink_to_fit()
- Shrink to an exact capacity.
  > data.shrink_to(10); // if len is more than 10, ignore, else shrink
- Truncate vector. Removes any extra data (even it valid)
  > data.truncate(2);

### complete code for resizing :

```
fn main() {

    // Create vector using macro
    let mut data:Vec<i32> = vec![2, 3, 5];
    println! ("Size= {} ", data.len());           // print 3
    println! ("capacity= {} ", data.capacity());  // print 3

    data.reserve_exact (100);
    println! ("capacity= {} ", data.capacity());  // print 103 or more

    data.reserve (1000);
    println! ("capacity= {} ", data.capacity());  // print 103 or more

    data.shrink_to_fit();
    println! ("capacity= {} ", data.capacity());  // print 3

    data.shrink_to(10);
    println! ("capacity= {} ", data.capacity());  // print 3

    println! ("len before= {} ", data.len());     // print 3
    data.truncate(2);
    println! ("capacity= {} ", data.capacity());  // print 3
    println! ("len after= {} ", data.len());      // print 3
}
```

## Cool Algorithms in Vector

- Reverse a vector. It is inplace, so vector must be mutable.
  > data.reverse();
- Check if an element is present. Returns bool and data is reference.
  > data.contains (&3)
- sort which is unstable.
  > data.sort_unstable();
- sort which is stable
  > data.sort();
- sort using a comparator
  > data.sort_by(compare_numbers)
  
  Example  of comparator
  ```
  fn compare_numbers(a: &i32, b: &i32) -> std::cmp::Ordering {
    b.cmp(a)
  }
  ```
- Other algorithms to explore
  > binary_search, sort_by_key, iter, allocator
