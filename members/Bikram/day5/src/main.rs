fn main() {
    // === STACK ALLOCATION ===
    // This i32 lives entirely on main()'s stack frame
    let stack_number: i32 = 42;
    
    // === HEAP ALLOCATION ===
    // The String struct (pointer, length, capacity) lives on the stack,
    // but the actual text bytes live on the heap.
    let heap_string = String::from("I live on the heap!");
    
    println!("=== main() stack frame ===");
    println!("stack_number:");
    println!("  value  = {}", stack_number);
    println!("  address on stack = {:p}", &stack_number);
    
    println!("\nheap_string:");
    println!("  String struct (on stack) = {:p}", &heap_string);
    println!("  pointer to heap data     = {:p}", heap_string.as_ptr());
    println!("  heap data contents       = \"{}\"", heap_string);
    
    // Call a function: a new stack frame is pushed on top of main's.
    // stack_number is COPIED (i32 is Copy) into the new frame.
    // heap_string is MOVED: its struct (the pointer/len/cap) is copied
    // into the new frame, but the heap data itself stays put.
    process_values(stack_number, heap_string);
    
    // === FUNCTION RETURNED ===
    // process_values()'s stack frame has been popped off.
    // stack_number is still valid because it was copied.
    println!("\n=== Back in main() ===");
    println!("stack_number still accessible: {}", stack_number);
    
    // heap_string is NOT accessible here — it was moved into the function
    // and dropped when that stack frame was popped (freeing the heap memory).
    // Uncommenting the next line would be a compile error:
    // println!("{}", heap_string);
}

fn process_values(local_num: i32, local_str: String) {
    // These are pushed onto THIS function's stack frame
    let local_flag: bool = true;
    let local_array: [i64; 3] = [1, 2, 3];
    
    println!("\n=== process_values() stack frame ===");
    
    println!("local_num (copy of stack_number):");
    println!("  value  = {}", local_num);
    println!("  address on stack = {:p}", &local_num);
    
    println!("\nlocal_str (moved heap_string):");
    println!("  String struct (on this stack frame) = {:p}", &local_str);
    println!("  pointer to heap data                = {:p}", local_str.as_ptr());
    println!("  heap data contents                  = \"{}\"", local_str);
    // Notice: the heap data address is the SAME as in main() — 
    // the data never moved, only the pointer to it was passed.
    
    println!("\nlocal_flag:");
    println!("  address on stack = {:p}", &local_flag);
    
    println!("local_array:");
    println!("  address on stack = {:p}", &local_array);
    
    // === FUNCTION ENDS ===
    // The entire stack frame is popped:
    //   - local_num    → gone
    //   - local_flag   → gone  
    //   - local_array  → gone
    //   - local_str    → gone (its Drop impl frees the heap memory)
    //
    // The heap data is deallocated now because local_str owned it.
    println!("\n--- process_values() returning: popping stack frame ---");
}