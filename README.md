# Rust vs Go Memory Model Comparison

A hands-on playground for understanding and comparing memory management approaches between Go and Rust.

## Purpose

This project demonstrates the fundamental differences in how Go and Rust handle memory management through executable examples and interactive demos.

**Go's Approach:**
- **Escape Analysis**: Compiler automatically decides stack vs heap allocation
- **Garbage Collection**: Runtime cleanup with automatic memory reclamation
- **Flexible References**: Multiple pointers can freely share memory
- **Trade-off**: Easy to write but comes with GC pauses and runtime overhead

**Rust's Approach:**
- **Ownership System**: Single owner with compile-time guarantees
- **Borrowing Rules**: Explicit, checked at compile-time for safety
- **Zero-Cost Abstractions**: No garbage collector, deterministic cleanup
- **Flexibility When Needed**: Opt-in runtime costs with `Rc<T>`, `RefCell<T>`, `Arc<T>`, `Mutex<T>`
- **Trade-off**: More explicit but delivers zero runtime cost and predictable performance

## Key Learning Points

1. **Go**: Compiler decides allocation, GC tracks references - easy but with runtime overhead
2. **Rust (default)**: Single ownership, compile-time checks - zero cost but requires explicit thinking
3. **Rust (flexible)**: `Rc<RefCell<T>>` provides Go-like behavior when needed - explicit and opt-in

## Getting Started

### Go Playground
```bash
cd golang-playground
make run       # See memory allocation in action
```

### Rust Playground
```bash
cd rust-playground
make run            # Explore ownership and borrowing
```

## The Core Question

> What if we moved garbage collection from runtime to compile-time and enforced single ownership?

**Answer:** You get Rust - zero runtime cost, deterministic cleanup, and memory safety guaranteed at compile-time, with explicit opt-in for flexible patterns when needed.

