### Array Types Quick Comparison Table

| **Type** | **Size** | **Allocation** | **Mutable	Common Use** |
| `[T; N]`      | Compile-time |  Stack	    |   Yes	|   Fixed data      |
| `&[T]`	    | Runtime	   |  Borrowed	|   No	|   Function params |
| `&mut [T]`    | Runtime	   |  Borrowed	|   Yes	|   Modify data     |
| `Vec<T>`	    | Dynamic	   |  Heap	    |   Yes	|   Most apps       |
| `Box<[T]>`    | Fixed	       |  Heap	    |   No	|   Fixed heap data |
| `VecDeque<T>` | Dynamic	   |  Heap	    |   Yes	|   Queues          |