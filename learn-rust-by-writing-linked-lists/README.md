# Learn Rust By Writing 'Too Many' Linked Lists

## Why Linked Lists?
Linked lists are challenging in C/C++ and most languages without GC (garbage collection).
To implement linked lists effectively in any of these, we should have opportunity to explore 
a breadth of both basic and advanced language features.

## Wait - Aren't Linked Lists Terrible?
Yes, they're not a very performant data structure. A few valid use cases could include:

- *lots* of splitting or merging of large lists
- lock-free concurrency
- writing an intrusive list
- you're writing Haskell and can leverage laziness

## Okay, I Get It - What's Our Alternative?
Use a freaking Vec (array stack), or - rarely - a VecDeque (array deque). They have
less frequent allocation, lower memory overhead, true random access, and cache loyalty.

