To create a packed signal:

1. Allocate ID from /protocol/pk-id.txt
1. Describe the signal in /protocol/spec.txt
1. Declare signal struct in pk/$name.rs
1. Add new pub mod to pk/mod.rs
1. Implement signal I/O in pk/$name.rs
1. Add signal handler to handler.rs
1. Add handler match call to pk/mod.rs
1. Implement handler in SignalHandler impls
1. Add signal I/O test case to /protocol/test.txt
