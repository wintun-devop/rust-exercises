### Variable
- case-sensitive.
- can combine alphanumeric characters and underscore.
- cannot start with a number.
- naming convention is snake case.
- Use snake_case (all lowercase with underscores).
```
fn get_user_name() {}        // ✅ Correct
let user_id = 42;            // ✅ Correct
mod utils_module {}          // ✅ Correct

fn GetUserName() {}          // ❌ Not idiomatic
let userId = 42;             // ❌ Not idiomatic
```
- Structs, Enums, Traits → PascalCase
```
struct UserProfile {         // ✅ Correct
    user_id: i32,
}

enum UserRole {              // ✅ Correct
    Admin,
    Guest,
}

trait Printable {}           // ✅ Correct

struct user_profile {}       // ❌ Not idiomatic
enum userrole {}             // ❌ Not idiomatic
```
- Lifetimes → Lowercase with a leading
```
fn get_ref<'a>(x: &'a str) -> &'a str { x } // ✅ Correct
```

 ----------- -----------------------  --------------------------------
| Item Type | Convention             | Example                         |
| --------- | ---------------------- | ------------------------------- |
| Functions | `snake_case`           | `fn get_user()`                 |
| Variables | `snake_case`           | `let user_name = ...`           |
| Modules   | `snake_case`           | `mod user_utils;`               |
| Structs   | `PascalCase`           | `struct UserProfile`            |
| Enums     | `PascalCase`           | `enum UserRole`                 |
| Traits    | `PascalCase`           | `trait Serializable`            |
| Constants | `SCREAMING_SNAKE_CASE` | `const MAX_SIZE: usize = 1024;` |
| Generics  | Single Uppercase       | `T`, `E`, `K`, `V`              |
| Lifetimes | `'lowercase`           | `'a`, `'b`                      |
