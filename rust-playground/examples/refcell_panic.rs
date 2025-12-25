// Example: RefCell runtime check that would panic

use std::cell::RefCell;

fn main() {
    let data = RefCell::new(42);
    
    println!("=== RefCell Runtime Check Example ===\n");
    
    // This works fine - sequential borrows
    {
        let borrow1 = data.borrow();
        println!("Immutable borrow 1: {}", borrow1);
    } // borrow1 dropped
    
    {
        let mut borrow2 = data.borrow_mut();
        *borrow2 = 100;
        println!("Mutable borrow: {}", borrow2);
    } // borrow2 dropped
    
    println!("\n✅ Sequential borrows work fine!\n");
    
    // This will PANIC at runtime!
    println!("Now trying to have immutable and mutable borrow at same time...");
    let _borrow = data.borrow();  // Immutable borrow
    
    // Uncomment this line to see the panic:
    // let _mut_borrow = data.borrow_mut();  // 💥 PANIC! "already borrowed: BorrowMutError"
    
    println!("(Commented out the panic line - uncomment to see it fail!)");
    println!("\n⚠️ RefCell checks borrowing rules at RUNTIME");
    println!("⚠️ Violating rules causes PANIC, not compile error");
}

