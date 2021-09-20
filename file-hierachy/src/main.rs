
mod my_module; // include my_module as folder
mod my_single_file_mod; // include  single file module
fn main() {
    my_single_file_mod::pub_single_file_fn();
    my_module::pub_fun_in_mymodule();
    my_module::public_nest_mod::pub_fun();
}
