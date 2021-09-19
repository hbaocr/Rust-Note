

//mod.rs is the entry point of your own module.
mod private_nest_mod; // this only use inside my_module.Can not be called outside ( in main.rs)
pub mod public_nest_mod;//Can be called outside ( in main.rs)

//Can be called outside ( in main.rs)
pub fn pub_fun_in_mymodule(){
    println!("pub_fun_in_mylib");
    private_nest_mod::pub_fun();
    public_nest_mod::pub_fun();
}

// default is private fn. It can Only be called inside this mod.rs
fn private_fun_in_mymodule(){
    println!("private_fun_in_mylib can not call out side")
}
