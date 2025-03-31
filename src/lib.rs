// Prevent Rust from renaming the symbol.
#[no_mangle]
pub extern "C" fn hello_world() {
    println!("Hello, world!");
}
