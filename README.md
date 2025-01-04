# Rust Double Mutable Borrow Example

This repository demonstrates a common error in Rust: attempting to create multiple mutable references to the same variable. Rust's borrow checker prevents this to avoid data races and ensure memory safety.

The `bug.rs` file contains the erroneous code.  The `bugSolution.rs` file shows how to solve it using techniques like using only one mutable borrow or using interior mutability through RefCell or Mutex.  This illustrates safe ways to handle mutable shared state.