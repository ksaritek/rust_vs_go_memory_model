use rust_playground::{borrow_checker, comparison, rc_weak};

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    println!("=== Rust Ownership & Borrowing Playground ===\n");

    // Example 1: Ownership (Single Owner)
    println!("1. Ownership - Single Owner");
    ownership_example();

    // Example 2: Move Semantics (Transfer Ownership)
    println!("\n2. Move Semantics");
    move_example();

    // Example 3: Borrowing (References)
    println!("\n3. Borrowing - Immutable References");
    borrowing_example();

    // Example 4: Mutable Borrowing
    println!("\n4. Mutable Borrowing");
    mutable_borrowing_example();

    // Example 5: Lifetimes & Stack Cleanup
    println!("\n5. Deterministic Cleanup (No GC!)");
    deterministic_cleanup();

    // Example 6: Borrow Checker Rules
    borrow_checker::demonstrate_borrow_checker();

    // Example 7: Go vs Rust Comparisons
    comparison::demonstrate_comparisons();

    // Example 8: Rc and Weak - Multiple Ownership
    rc_weak::demonstrate_rc();
}

// Example 1: Ownership - each value has ONE owner
fn ownership_example() {
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };
    
    println!("  Owner: {:p} -> {:?}", &user, user);
    println!("  ✓ Single owner: 'user' owns the data");
    
    // user goes out of scope here - automatically cleaned up!
}

// Example 2: Move semantics - ownership transfers
fn move_example() {
    let user1 = User {
        name: String::from("Bob"),
        age: 25,
    };
    
    println!("  user1 owns:     {:p} -> {:?}", &user1, user1);
    
    let user2 = user1;  // Ownership MOVES to user2
    
    println!("  user2 owns:     {:p} -> {:?}", &user2, user2);
    println!("  ✗ user1 is no longer valid (moved!)");
    
    // Uncommenting this would cause a compile error:
    // println!("{:?}", user1);  // ❌ Error: value borrowed after move
}

// Example 3: Borrowing - multiple immutable references allowed
fn borrowing_example() {
    let user = User {
        name: String::from("Charlie"),
        age: 35,
    };
    
    println!("  Owner:  {:p} -> {:?}", &user, user);
    
    // Multiple immutable borrows are OK!
    let ref1 = &user;
    let ref2 = &user;
    let ref3 = &user;
    
    println!("  Ref1:   {:p} -> {:?}", ref1, ref1);
    println!("  Ref2:   {:p} -> {:?}", ref2, ref2);
    println!("  Ref3:   {:p} -> {:?}", ref3, ref3);
    println!("  ✓ Multiple immutable borrows allowed");
    println!("  ✓ All point to same memory (like Go)");
    println!("  ✓ But owner maintains control!");
}

// Example 4: Mutable borrowing - only ONE mutable reference allowed
fn mutable_borrowing_example() {
    let mut user = User {
        name: String::from("Diana"),
        age: 28,
    };
    
    println!("  Original: {:?}", user);
    
    // Only ONE mutable borrow at a time!
    let user_ref = &mut user;
    user_ref.age = 29;
    
    println!("  After modification: {:?}", user_ref);
    println!("  ✓ Only ONE mutable borrow at a time");
    println!("  ✓ Prevents data races at compile-time!");
    
    // Uncommenting this would cause a compile error:
    // let ref2 = &mut user;  // ❌ Error: cannot borrow as mutable more than once
}

// Example 5: Deterministic cleanup - no GC needed!
fn deterministic_cleanup() {
    println!("  Creating users...");
    
    {
        let user1 = User {
            name: String::from("Eve"),
            age: 40,
        };
        println!("    user1 created: {:?}", user1);
        
        {
            let user2 = User {
                name: String::from("Frank"),
                age: 45,
            };
            println!("    user2 created: {:?}", user2);
            
            println!("    user2 scope ends → cleaned up immediately");
        } // user2 dropped here - deterministic!
        
        println!("    user1 scope ends → cleaned up immediately");
    } // user1 dropped here - deterministic!
    
    println!("  ✓ No garbage collector needed");
    println!("  ✓ Memory freed at end of scope (RAII)");
    println!("  ✓ Zero runtime overhead!");
}
