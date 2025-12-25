// Reference counting in Rust - when you DO need multiple owners
// Rc = Reference Counted, Weak = Weak reference
// RefCell = Runtime-checked borrowing
// Arc = Atomic Rc (thread-safe)
// Mutex = Thread-safe RefCell

use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: Option<Weak<Node>>,  // Weak to prevent cycles
    children: Vec<Rc<Node>>,
}

// Rc<T> allows multiple owners (like Go!)
pub fn rc_example() {
    println!("\n=== Rc<T> - Reference Counted (Multiple Owners) ===\n");
    
    let data = Rc::new(42);
    
    println!("  Original Rc:  {:p}, value: {}, strong_count: {}", 
             data.as_ref(), data, Rc::strong_count(&data));
    
    let ref1 = Rc::clone(&data);  // Increment ref count
    let ref2 = Rc::clone(&data);  // Increment ref count
    let ref3 = Rc::clone(&data);  // Increment ref count
    
    println!("  After cloning:");
    println!("    ref1: {:p}, strong_count: {}", ref1.as_ref(), Rc::strong_count(&ref1));
    println!("    ref2: {:p}, strong_count: {}", ref2.as_ref(), Rc::strong_count(&ref2));
    println!("    ref3: {:p}, strong_count: {}", ref3.as_ref(), Rc::strong_count(&ref3));
    
    println!("\n  ✓ All point to same memory (like Go!)");
    println!("  ✓ Reference counted at RUNTIME");
    println!("  ⚠️ Not thread-safe (use Arc<T> for threads)");
    
    drop(ref1);
    println!("\n  After dropping ref1, strong_count: {}", Rc::strong_count(&data));
    
    drop(ref2);
    drop(ref3);
    println!("  After dropping all refs, strong_count: {}", Rc::strong_count(&data));
    println!("  When last reference drops, memory is freed!");
}

// Weak<T> prevents reference cycles
pub fn weak_example() {
    println!("\n=== Weak<T> - Preventing Reference Cycles ===\n");
    
    let parent = Rc::new(Node {
        value: 1,
        parent: None,
        children: vec![],
    });
    
    println!("  Parent created, strong_count: {}", Rc::strong_count(&parent));
    
    let child = Rc::new(Node {
        value: 2,
        parent: Some(Rc::downgrade(&parent)),  // Weak reference!
        children: vec![],
    });
    
    println!("  Child created with Weak parent reference");
    println!("    Parent strong_count: {}", Rc::strong_count(&parent));
    println!("    Parent weak_count: {}", Rc::weak_count(&parent));
    
    // Try to access parent through weak reference
    if let Some(parent_ref) = child.parent.as_ref().and_then(|w| w.upgrade()) {
        println!("    Parent value accessed through Weak: {}", parent_ref.value);
    }
    
    println!("\n  ✓ Weak doesn't increase strong_count");
    println!("  ✓ Prevents memory leaks from cycles");
    println!("  ✓ upgrade() returns Option (might be dropped)");
}

// Comparison: Go vs Rust reference counting
pub fn rc_comparison() {
    println!("\n=== Reference Counting: Go vs Rust ===\n");
    
    println!("Go (automatic):");
    println!("  user := &User{{...}}");
    println!("  ptr1 := user  // GC tracks automatically");
    println!("  ptr2 := user  // GC tracks automatically");
    println!("  ptr3 := user  // GC tracks automatically");
    println!("  ✓ Automatic reference counting");
    println!("  ⚠️ GC overhead, stop-the-world pauses");
    
    println!("\nRust (explicit with Rc):");
    println!("  let data = Rc::new(42);");
    println!("  let ref1 = Rc::clone(&data);  // Explicit clone");
    println!("  let ref2 = Rc::clone(&data);  // Explicit clone");
    println!("  let ref3 = Rc::clone(&data);  // Explicit clone");
    println!("  ✓ Explicit reference counting");
    println!("  ✓ No GC, no stop-the-world");
    println!("  ✓ Deterministic cleanup");
    println!("  ⚠️ Small runtime cost (increment/decrement counter)");
    
    println!("\nRust (default ownership):");
    println!("  let data = 42;");
    println!("  let ref1 = &data;  // Just borrows");
    println!("  let ref2 = &data;  // Just borrows");
    println!("  ✓ ZERO runtime cost!");
    println!("  ✓ Compile-time checks only");
}

// Show the cost difference
pub fn cost_comparison() {
    println!("\n=== Cost Comparison ===\n");
    
    println!("Rust borrowing (zero cost):");
    println!("  let data = vec![1, 2, 3];");
    println!("  let ref1 = &data;  // No cost");
    println!("  let ref2 = &data;  // No cost");
    println!("  Cost: 0 bytes, 0 cycles");
    
    println!("\nRust Rc (small cost):");
    println!("  let data = Rc::new(vec![1, 2, 3]);");
    println!("  let ref1 = Rc::clone(&data);  // Increment counter");
    println!("  let ref2 = Rc::clone(&data);  // Increment counter");
    println!("  Cost: Extra pointer + 2 counters (~16 bytes)");
    println!("        Atomic increment/decrement operations");
    
    println!("\nGo GC (runtime cost):");
    println!("  data := []int{{1, 2, 3}}");
    println!("  ref1 := data  // GC tracks");
    println!("  ref2 := data  // GC tracks");
    println!("  Cost: GC tracking + periodic collection");
    println!("        Stop-the-world pauses");
    println!("        Memory overhead for GC metadata");
}

// RefCell - interior mutability with runtime checks
pub fn refcell_example() {
    println!("\n=== RefCell<T> - Interior Mutability (Runtime Checks) ===\n");
    
    let data = RefCell::new(42);
    
    println!("  Original value: {}", data.borrow());
    
    // Multiple immutable borrows OK
    {
        let borrow1 = data.borrow();
        let borrow2 = data.borrow();
        println!("  Immutable borrows: {} and {}", borrow1, borrow2);
    } // Borrows dropped here
    
    // Mutable borrow OK (after immutable borrows done)
    {
        let mut borrow_mut = data.borrow_mut();
        *borrow_mut = 100;
        println!("  After mutation: {}", borrow_mut);
    }
    
    println!("\n  ✓ Borrowing rules checked at RUNTIME");
    println!("  ✓ Allows mutation through immutable reference");
    println!("  ⚠️ Panics if you violate rules (not compile error!)");
    
    // This would panic at runtime:
    // let borrow = data.borrow();
    // let mut_borrow = data.borrow_mut();  // 💥 Panic!
}

// Rc<RefCell<T>> - The common pattern
pub fn rc_refcell_example() {
    println!("\n=== Rc<RefCell<T>> - Multiple Owners + Mutability ===\n");
    
    let data = Rc::new(RefCell::new(vec![1, 2, 3]));
    
    let ref1 = Rc::clone(&data);
    let ref2 = Rc::clone(&data);
    let ref3 = Rc::clone(&data);
    
    println!("  Original: {:?}", data.borrow());
    
    // Mutate through ref1
    ref1.borrow_mut().push(4);
    println!("  After ref1.push(4): {:?}", data.borrow());
    
    // Mutate through ref2
    ref2.borrow_mut().push(5);
    println!("  After ref2.push(5): {:?}", data.borrow());
    
    // All refs see the changes!
    println!("  Via ref3: {:?}", ref3.borrow());
    
    println!("\n  ✓ Multiple owners (Rc)");
    println!("  ✓ Shared mutability (RefCell)");
    println!("  ✓ Like Go's behavior, but explicit!");
    println!("  ⚠️ Runtime borrow checking (can panic)");
}

// Compare with Go
pub fn refcell_vs_go() {
    println!("\n=== RefCell vs Go Mutability ===\n");
    
    println!("Go (automatic, no checks):");
    println!("  data := []int{{1, 2, 3}}");
    println!("  ref1 := &data");
    println!("  ref2 := &data");
    println!("  *ref1 = append(*ref1, 4)  // Mutate");
    println!("  *ref2 = append(*ref2, 5)  // Mutate");
    println!("  ✓ No borrow checking");
    println!("  ⚠️ Possible data races with goroutines");
    
    println!("\nRust with Rc<RefCell<T>> (single-threaded):");
    println!("  let data = Rc::new(RefCell::new(vec![1, 2, 3]));");
    println!("  let ref1 = Rc::clone(&data);");
    println!("  let ref2 = Rc::clone(&data);");
    println!("  ref1.borrow_mut().push(4);  // Runtime check");
    println!("  ref2.borrow_mut().push(5);  // Runtime check");
    println!("  ✓ Explicit ownership (Rc)");
    println!("  ✓ Runtime borrow checking (RefCell)");
    println!("  ✓ No data races (not thread-safe, won't compile)");
    
    println!("\nRust with Arc<Mutex<T>> (thread-safe):");
    println!("  let data = Arc::new(Mutex::new(vec![1, 2, 3]));");
    println!("  ✓ Multiple owners across threads");
    println!("  ✓ Safe concurrent mutation");
    println!("  ✓ Compiler enforces thread safety");
}

// Arc<Mutex<T>> - thread-safe version
pub fn arc_mutex_example() {
    println!("\n=== Arc<Mutex<T>> - Thread-Safe Rc<RefCell<T>> ===\n");
    
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    
    println!("  Original: {:?}", data.lock().unwrap());
    
    let mut handles = vec![];
    
    // Spawn 3 threads, each adds a number
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut vec = data_clone.lock().unwrap();
            vec.push(i + 10);
            println!("    Thread {} added {}", i, i + 10);
        });
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("  Final: {:?}", data.lock().unwrap());
    
    println!("\n  Arc = Atomic Rc (thread-safe ref counting)");
    println!("  Mutex = Thread-safe RefCell (locks instead of panics)");
    println!("  ✓ Can share across threads");
    println!("  ✓ Prevents data races");
    println!("  ⚠️ Higher cost (atomic ops + locking)");
}

// Compare all three
pub fn compare_all_three() {
    println!("\n=== Comparison: Rc<RefCell> vs Arc<Mutex> vs Go ===\n");
    
    println!("Rc<RefCell<T>> (single-threaded):");
    println!("  - Multiple owners: Rc (ref counting)");
    println!("  - Mutability: RefCell (runtime checks, can panic)");
    println!("  - Thread-safe: ❌ No");
    println!("  - Cost: Low (ref counting + borrow checks)");
    
    println!("\nArc<Mutex<T>> (thread-safe):");
    println!("  - Multiple owners: Arc (atomic ref counting)");
    println!("  - Mutability: Mutex (locks, blocks threads)");
    println!("  - Thread-safe: ✅ Yes");
    println!("  - Cost: Higher (atomic ops + locking)");
    
    println!("\nGo (automatic):");
    println!("  - Multiple owners: ✅ Automatic (GC)");
    println!("  - Mutability: ✅ Automatic (no checks)");
    println!("  - Thread-safe: ⚠️ Manual sync needed");
    println!("  - Cost: High (GC overhead + potential races)");
    
    println!("\nKey Insight:");
    println!("  Rust: Choose your tradeoff explicitly");
    println!("  Go: One size fits all (GC)");
}

pub fn demonstrate_rc() {
    rc_example();
    weak_example();
    rc_comparison();
    cost_comparison();
    refcell_example();
    rc_refcell_example();
    refcell_vs_go();
    arc_mutex_example();
    compare_all_three();
}

