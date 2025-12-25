// Comparing Go vs Rust memory models

#[allow(dead_code)]
#[derive(Debug)]
struct LargeObject {
    id: usize,
    data: Vec<u8>,
}

// Stack allocation in Rust
pub fn stack_allocation() {
    println!("\n=== Stack Allocation ===\n");
    
    let x = 42;
    let y = 100;
    
    println!("  x at: {:p}, value: {}", &x, x);
    println!("  y at: {:p}, value: {}", &y, y);
    println!("  ✓ Allocated on stack");
    println!("  ✓ Cleaned up when scope ends");
    println!("  ✓ No heap allocation, no GC!");
}

// Heap allocation in Rust - explicit with Box
pub fn heap_allocation() {
    println!("\n=== Heap Allocation ===\n");
    
    let x = Box::new(42);  // Explicitly heap-allocated
    let y = Box::new(100);
    
    println!("  x points to: {:p}, value: {}", x.as_ref(), x);
    println!("  y points to: {:p}, value: {}", y.as_ref(), y);
    println!("  ✓ Box<T> = explicit heap allocation");
    println!("  ✓ Still cleaned up deterministically (no GC)");
    println!("  ✓ Owner drops when out of scope");
}

// Compare: Go allows multiple owners, Rust doesn't
pub fn ownership_comparison() {
    println!("\n=== Go vs Rust: Multiple Owners ===\n");
    
    println!("Go (allowed):");
    println!("  user := &User{{...}}");
    println!("  ptr1 := user  // OK - GC tracks all");
    println!("  ptr2 := user  // OK - GC tracks all");
    println!("  ptr3 := user  // OK - GC tracks all");
    
    println!("\nRust (not allowed):");
    println!("  let user = User{{...}};");
    println!("  let owner2 = user;  // MOVES ownership");
    println!("  // ❌ user is now invalid!");
    
    println!("\nRust alternative (borrowing):");
    println!("  let user = User{{...}};");
    println!("  let ref1 = &user;  // Borrow");
    println!("  let ref2 = &user;  // Borrow");
    println!("  let ref3 = &user;  // Borrow");
    println!("  ✓ Multiple borrows OK");
    println!("  ✓ Original owner still controls lifetime");
}

// Memory tracking comparison
pub fn memory_comparison() {
    println!("\n=== Memory Allocation Comparison ===\n");
    
    let objects: Vec<LargeObject> = (0..10)
        .map(|i| LargeObject {
            id: i,
            data: vec![0u8; 1024],
        })
        .collect();
    
    println!("  Created 10 LargeObjects (1KB each)");
    println!("  Total: ~10KB");
    println!("\n  Go approach:");
    println!("    - Escape analysis decides heap allocation");
    println!("    - GC tracks at runtime");
    println!("    - GC pauses to clean up");
    println!("\n  Rust approach:");
    println!("    - Vec<T> explicitly owns heap data");
    println!("    - Cleaned up when 'objects' goes out of scope");
    println!("    - NO garbage collector");
    println!("    - NO runtime overhead");
    
    drop(objects);
    // ✓ After drop(), 'objects' is no longer accessible
    // Uncommenting this would cause a compile error:
    // println!("{:?}", objects);  // ❌ Error: borrow of moved value
    println!("\n  ✓ Objects dropped deterministically!");
}

pub fn demonstrate_comparisons() {
    stack_allocation();
    heap_allocation();
    ownership_comparison();
    memory_comparison();
}

