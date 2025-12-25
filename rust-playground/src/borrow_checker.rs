// This file demonstrates Rust's borrowing rules
// These are COMPILE-TIME checks - no runtime cost!

#[allow(dead_code)]
#[derive(Debug)]
struct Data {
    value: i32,
}

// Rule 1: You can have EITHER multiple immutable refs OR one mutable ref
#[allow(dead_code)]
fn borrowing_rules() {
    let mut data = Data { value: 42 };
    
    // ✓ Multiple immutable borrows OK
    let r1 = &data;
    let r2 = &data;
    println!("r1: {}, r2: {}", r1.value, r2.value);
    
    // ✓ Mutable borrow OK (after immutable borrows done)
    let r3 = &mut data;
    r3.value = 100;
    println!("r3: {}", r3.value);
    
    // ❌ This would fail: can't have immutable and mutable at same time
    // let r4 = &data;
    // let r5 = &mut data;  // Error!
}

// Rule 2: References must not outlive the data they refer to
#[allow(dead_code)]
fn no_dangling_references() {
    let _reference: &Data;
    
    {
        let _data = Data { value: 42 };
        // ❌ This would fail: reference would outlive data
        // _reference = &_data;  // Error: borrowed value does not live long enough
    }
    
    // If we could compile above, reference would be dangling here!
    // println!("{:?}", _reference);  // Would be undefined behavior in C/Go
}

// Rule 3: Moves prevent use-after-move bugs
#[allow(dead_code)]
fn move_semantics() {
    let data1 = Data { value: 42 };
    let data2 = data1;  // Ownership moves to data2
    
    // ❌ This would fail: can't use data1 after move
    // println!("{}", data1.value);  // Error: value borrowed after move
    
    println!("{}", data2.value);  // ✓ OK
}

// Rule 4: Interior mutability with RefCell (runtime checks)
#[allow(dead_code)]
fn interior_mutability_example() {
    use std::cell::RefCell;
    
    let data = RefCell::new(Data { value: 42 });
    
    // Multiple borrows through RefCell
    let borrow1 = data.borrow();
    let borrow2 = data.borrow();
    println!("borrow1: {}, borrow2: {}", borrow1.value, borrow2.value);
    drop(borrow1);
    drop(borrow2);
    
    // Mutable borrow
    let mut borrow_mut = data.borrow_mut();
    borrow_mut.value = 100;
    println!("After mutation: {}", borrow_mut.value);
}

// Demonstrate the key rules
pub fn demonstrate_borrow_checker() {
    println!("\n=== Borrow Checker Rules ===\n");
    
    println!("Rule 1: Multiple immutable OR one mutable");
    borrowing_rules();
    
    println!("\nRule 2: No dangling references (enforced at compile-time)");
    println!("  ✓ Compiler prevents dangling pointers");
    
    println!("\nRule 3: Move semantics prevent use-after-move");
    move_semantics();
    
    println!("\nRule 4: RefCell for runtime-checked borrowing");
    interior_mutability_example();
}

