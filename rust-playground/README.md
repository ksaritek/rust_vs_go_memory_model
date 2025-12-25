# Rust Ownership & Borrowing Playground

> Understanding Rust's ownership system and comparing with Go's GC approach

## What's Inside

- **main.rs** - Core ownership and borrowing examples
- **borrow_checker.rs** - Borrow checker rules explained
- **comparison.rs** - Direct Go vs Rust comparisons
- **rc_weak.rs** - Rc<T>, Weak<T>, and RefCell<T> for flexible ownership

## Key Concepts Demonstrated

### 1. **Ownership** (Single Owner)
```rust
let user = User{...};  // user OWNS the data
// Only ONE owner at a time
```

### 2. **Move Semantics**
```rust
let user1 = User{...};
let user2 = user1;  // Ownership MOVES to user2
// user1 is no longer valid!
```

### 3. **Borrowing** (References)
```rust
let user = User{...};
let ref1 = &user;  // Immutable borrow
let ref2 = &user;  // Multiple immutable borrows OK
// user still owns the data
```

### 4. **Mutable Borrowing**
```rust
let mut user = User{...};
let ref_mut = &mut user;  // Only ONE mutable borrow at a time
// Prevents data races at compile-time!
```

### 5. **No Garbage Collector**
```rust
{
    let user = User{...};  // Created
    // ...
}  // Dropped here - deterministic!
```

### 6. **Rc<T> and Weak<T>** (When You DO Need Multiple Owners)
```rust
let data = Rc::new(42);
let ref1 = Rc::clone(&data);  // Reference counted
let ref2 = Rc::clone(&data);  // Reference counted
// Cleaned up when last Rc is dropped
```

**Weak<T>** prevents reference cycles:
```rust
let parent = Rc::new(Node{...});
let child = Node {
    parent: Some(Rc::downgrade(&parent)),  // Weak reference
    ...
};
```

### 7. **RefCell<T>** (Interior Mutability - Runtime Checks)
```rust
let data = RefCell::new(42);
let borrow = data.borrow();       // Immutable borrow
let mut_borrow = data.borrow_mut(); // Mutable borrow
// Borrowing rules checked at RUNTIME (can panic!)
```

### 8. **Rc<RefCell<T>>** (Multiple Owners + Mutability)
```rust
let data = Rc::new(RefCell::new(vec![1, 2, 3]));
let ref1 = Rc::clone(&data);
ref1.borrow_mut().push(4);  // Mutate through shared ownership
// Like Go's behavior, but explicit!
```

## Go vs Rust: The Key Differences

| Aspect | Go | Rust Borrowing | Rust Rc<RefCell<T>> |
|--------|----|--------------|--------------------|
| **Multiple owners** | ✅ Allowed | ❌ Single owner | ✅ Allowed |
| **Mutability** | ✅ Automatic | Compile-time rules | ✅ Runtime checks |
| **When checked** | Runtime (GC) | **Compile-time** | **Runtime** (borrow_mut) |
| **Runtime cost** | High (GC) | **Zero!** | Low (ref count + checks) |
| **Can panic** | ❌ No | ❌ No | ⚠️ Yes (if misused) |
| **Thread-safe** | ✅ (with sync) | ✅ (with rules) | ❌ (use Arc<Mutex>) |
| **Explicit** | ❌ Implicit | ✅ Explicit | ✅ Very explicit |

## Rust's Flexibility: Choose Your Cost Model

**1. Borrowing (default) - Zero Cost, Compile-Time:**
```rust
let data = vec![1, 2, 3];
let ref1 = &data;  // No runtime cost!
// Checked at COMPILE-TIME
```

**2. Rc<T> - Small Cost, Reference Counting:**
```rust
let data = Rc::new(vec![1, 2, 3]);
let ref1 = Rc::clone(&data);  // Increment ref count
// Runtime cost: increment/decrement counter
```

**3. RefCell<T> - Runtime Borrow Checks:**
```rust
let data = RefCell::new(vec![1, 2, 3]);
data.borrow_mut().push(4);  // Checked at RUNTIME, can panic!
// Runtime cost: check borrow state
```

**4. Rc<RefCell<T>> - Like Go (Explicit + Runtime Checks):**
```rust
let data = Rc::new(RefCell::new(vec![1, 2, 3]));
let ref1 = Rc::clone(&data);
ref1.borrow_mut().push(4);  // Runtime borrow check! Can panic!
// Combines: Rc (ref counting) + RefCell (runtime borrow check)
// Runtime cost: ref counting + borrow checking
```

**5. Arc<Mutex<T>> - Thread-Safe Version of Rc<RefCell<T>>:**
```rust
let data = Arc::new(Mutex::new(vec![1, 2, 3]));
let ref1 = Arc::clone(&data);  // Can send across threads!
ref1.lock().unwrap().push(4);  // Lock + runtime check
// Arc = Atomic Rc (thread-safe ref counting)
// Mutex = Thread-safe RefCell (locks instead of panics)
// Runtime cost: atomic ref counting + mutex locking
```

### The Parallel:

| Feature | Single-threaded | Thread-safe |
|---------|----------------|-------------|
| **Multiple Owners** | `Rc<T>` | `Arc<T>` (Atomic) |
| **Interior Mutability** | `RefCell<T>` | `Mutex<T>` |
| **Combined** | `Rc<RefCell<T>>` | `Arc<Mutex<T>>` |
| **Cost** | Ref counting + checks | Atomic ops + locking |
| **Failure Mode** | Panics | Panics (poison) |

## The Trade-off

**Go:**
- ✅ Easy to write
- ✅ Multiple references allowed
- ⚠️ GC pauses
- ⚠️ Runtime overhead

**Rust (Borrowing - default):**
- ✅ Zero runtime cost
- ✅ Compile-time safety
- ⚠️ Single owner, explicit borrows

**Rust (Rc<T> - when needed):**
- ✅ Multiple owners (like Go)
- ✅ No GC, deterministic cleanup
- ⚠️ Small runtime cost (ref counting)
- ⚠️ Not thread-safe (use Arc<T>)

## Examples Run

```bash
make run            # Run all examples
make refcell-panic  # See RefCell runtime checking
make examples       # List all examples
```

You'll see:
1. Ownership and move semantics
2. Borrowing rules in action
3. Deterministic cleanup (no GC!)
4. Borrow checker rules explained
5. Direct Go vs Rust comparisons

