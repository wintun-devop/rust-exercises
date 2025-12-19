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

|**Type** | **Size**| **Allocation** | **Mutable** |**Common Use**|
|---|---:|---|:---:|---|   
| `[T; N]`     | Compile-time | Stack	 |   Yes | Fixed data      |
| `&[T]`	   | Runtime	  | Borrowed |   No	 | Function params |
| `&mut [T]`   | Runtime	  | Borrowed |   Yes | Modify data     |
| `Vec<T>`	   | Dynamic	  | Heap	 |   Yes | Most apps       |
| `Box<[T]>`   | Fixed	      | Heap	 |   No	 | Fixed heap data |
| `VecDeque<T>`| Dynamic	  | Heap	 |   Yes | Queues          |