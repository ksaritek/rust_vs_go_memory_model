# Go Memory Model - Live Demo Slides

## Goal: Understand Go's Memory Model to Compare with Rust

---

## Slide 1: Go Memory Model Overview

### Key Concepts
- **Escape Analysis**: Compiler decides stack vs heap
- **Garbage Collector**: Runtime cleanup of heap memory
- **Multiple References**: Many pointers can share same memory

### The Big Question
> How does Go manage memory without manual allocation/deallocation?

**Answer:** Two-step process:
1. **Compile-time**: Escape analysis determines stack vs heap
2. **Runtime**: Garbage collector cleans up heap memory automatically

**invalid memory (dangling pointer!)**

```go
// Question 1: Does a pointer to this variable leave the function?
func escape1() *int {
    x := 42
    return &x  // YES → ESCAPES (pointer returned)
}

// Question 2: Is it stored in something that outlives the function?
var global *int
func escape2() {
    x := 42
    global = &x  // YES → ESCAPES (stored in global)
}

// Question 3: Is it captured by a closure that outlives the function?
func escape3() func() int {
    x := 42
    return func() int { 
        return x  // YES → ESCAPES (captured by closure)
    }
}

// Question 4: Is it too large for the stack?
func escape4() {
    large := make([]int, 1000000)  // YES → ESCAPES (too large)
    _ = large
}

// Question 5: Is it assigned to an interface?
var iface interface{}
func escape5() {
    x := 42
    iface = x  // YES → ESCAPES (interface assignment)
}
```

**You write:** `user := &User{Name: "Bob"}`
- **Compiler decides** where to allocate (stack or heap)
- **GC tracks** references and cleans up when done
- **No manual** `malloc`/`free` needed!


---

## Slide 2: DEMO - Run the Playground

```bash
make run
```

### What You'll See:
1. Stack vs Heap allocation
2. Multiple pointers sharing memory (4 pointers → same address!)
3. Slice sharing (modify one, affects all)
4. Memory measurements (actual bytes allocated)

### Key Observation
> Notice: Multiple pointers all point to `0x14000...` (same heap address)

---

## Slide 3: DEMO - What Escapes to Heap?

```bash
make heap-only
```

### What Causes Heap Allocation?
- Returning pointers from functions
- Large allocations
- Interface assignments
- Storing in globals
- Closures capturing variables

### Key Point
> The compiler decides! Not the programmer.

---

## Slide 4: DEMO - Detailed Escape Analysis

```bash
make escape
```

### What the Compiler Tells Us:

**1. `moved to heap: u`**
- Variable `u` is being moved from stack to heap
- Reason: Returning a pointer to it

**2. `can inline createUser`**
- Compiler can copy function body directly into caller
- Optimization: Eliminates function call overhead
- Example: `user := createUser()` becomes `user := &User{...}` directly

**3. `&LargeObject{...} escapes to heap`**
- Taking address of struct causes heap allocation
- Why: The pointer is returned/stored somewhere

### In Action:
```go
func createUser() *User {
    u := User{Name: "Alice"}  // Line 43: moved to heap: u
    return &u                  // Pointer escapes!
}
```

The compiler says:
- "I can inline this function" (fast!)
- "But u must go to heap" (cost of returning pointer)

### Compare in Your Mind
> In Rust: You would explicitly choose ownership/borrowing

---

## Slide 5: Go's Memory Model Summary

### Pros (Why Go is Easy)
- **No thinking required**: Compiler figures it out
- **Flexible**: Multiple references to same data
- **Safe**: GC prevents memory leaks/dangling pointers

### Cons (Why Rust Exists)
- **Runtime overhead**: GC pauses during collection
- **Unpredictable**: Hard to know what goes to heap
- **Less control**: Can't optimize critical paths

---

## Slide 6: Real Memory Impact

### Look at the output from make run:
```
Stack Only:              0 bytes    (no heap!)
Heap Allocation x10:  10,560 bytes  (20 objects)
Large Allocation:  1,048,576 bytes  (1MB on heap)
```

### The Point
> Go's GC tracks ALL of this at runtime. Cost = performance overhead.

---

## Slide 7: The Key Example - Multiple Owners

### In Go (Totally Fine):
```go
user := &User{Name: "Bob"}
ptr1 := user  // Share
ptr2 := user  // Share
ptr3 := user  // Share
ptr1.Age = 26 // Modify through any pointer
// GC cleans up when all references gone
```

### This is the CORE difference from Rust!
> In Rust: Only ONE owner, or explicit borrowing rules

---

## Slide 8: Transition to Rust

### What We Learned:
- Go uses **Escape Analysis** + **GC** (runtime)
- Multiple references allowed freely
- Compiler decides allocation
- GC cleans up automatically

### Key Questions:
> What if we moved ALL of this to **compile-time**?
> What if we enforced **single ownership**?
> What if we had **zero runtime cost**?

**→ That's Rust!**
