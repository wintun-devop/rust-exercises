### Array Types Quick Comparison Table
- Fixed-Size Array [T; N]
  - Key Points
    - Size is known at compile time
    - Stored on the stack
    - Length cannot change
  - Use When
    - You know the exact size
    - Performance-critical code
    - Embedded / low-level systems
```
let a: [i32; 3] = [1, 2, 3];
```
```
let numbers = [0; 10]; // 10 zeros
```
- Slice &[T] and &mut [T]
  - Key Points
    - Borrowed view of an array or vector
    - Size known at runtime
    - No ownership
  - Use When
    - Writing generic functions
    - Passing read-only or mutable data
    - Avoid copying
```
let a = [1, 2, 3, 4];
```
```
let s: &[i32] = &a[1..3];
```
```
fn sum(data: &[i32]) -> i32 {
    data.iter().sum()
}
```
- Vector Vec<T>
  - Key Points
    - Heap allocated
    - Growable and shrinkable
    - Most commonly used collection
  - Use When
    - Dynamic size
    - API responses
    - Database results
    - General purpose collections

```
let mut v: Vec<i32> = vec![1, 2, 3];
```
```
v.push(4);
```
```
let mut users = Vec::new();
```
```
users.push("Alice");
```
- Array Reference &[T; N]
 - Key Points
   - Reference to a fixed-size array
   - Keeps size info at compile time
 - Use When
   - You need both reference + fixed size
   - Const-generic APIs
```
let a = [1, 2, 3];
```
```
let r: &[i32; 3] = &a;

```

- Boxed Array Box<[T]>
  - Key Points
    - Heap allocated
    - Fixed size after creation
  - Use When
    - Heap allocation required
    - Size should not change
    - Reduce stack usage

```
let b: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
```
- VecDeque<T> (Double-Ended Queue)
  - Key Points
    - Efficient push/pop front & back
    - Circular buffer
  - Use When
    - Queues
    - Sliding windows
    - Task schedulers

```
use std::collections::VecDeque;
let mut d = VecDeque::new();
d.push_front(1);
d.push_back(2);
```

|**Type** | **Size**| **Allocation** | **Mutable** |**Common Use**|
|---|---:|---|:---:|---|   
| `[T; N]`     | Compile-time | Stack	 |   Yes | Fixed data      |
| `&[T]`	   | Runtime	  | Borrowed |   No	 | Function params |
| `&mut [T]`   | Runtime	  | Borrowed |   Yes | Modify data     |
| `Vec<T>`	   | Dynamic	  | Heap	 |   Yes | Most apps       |
| `Box<[T]>`   | Fixed	      | Heap	 |   No	 | Fixed heap data |
| `VecDeque<T>`| Dynamic	  | Heap	 |   Yes | Queues          |