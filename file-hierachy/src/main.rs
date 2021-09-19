
mod my_module; // include my_module
fn main() {
    println!("Hello, world!");
    my_module::pub_fun_in_mymodule();
    my_module::public_nest_mod::pub_fun();
}
