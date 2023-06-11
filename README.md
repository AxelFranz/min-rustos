# min-rustos
Just me trying to follow https://os.phil-opp.com

It's a minimal Rust-written OS. Cargo run/test

Implemented : 
- Handle software timer, Double faults, INT3, Page Fault and Stack Overflow
- Can write/delete/create a new line
<br>
TODO :
- Handle the delete key (return works)
- Finish reading the blog
<br>

To install additional dependencies : 
- `cargo install bootimage`
- qemu-system-x86_64 in path (included in qemu-base)
- qemu-ui-gtk to run qemu in graphical mode
