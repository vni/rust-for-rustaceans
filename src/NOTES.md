## chapter 1


- each shared reference is Copy


- The order in which to drop are fairly simple: variables (including function arguments) are dropped in reverse order, and nested values are dropped in source-code order.


- The Rust compiler is allowed to assume that the value a shared reference points to will not change while that reference lives.