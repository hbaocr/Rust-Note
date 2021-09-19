mod module_name{
    // default is private, only be called in this mod
    fn priv_fn(){
        println!{"priv_fn"}
    }
    // public function can be call outside 
    pub fn pub_fn(){
        println!{"pub_fn"} 
    }
    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }
    // pub nest module
    pub mod nest_mod{
        pub fn pub_fn_in_nest_mod(){
            println!{"pub_fn"} 
        }
        // We can also use `self` to access  inside this module
        pub fn test_self(){
            self::pub_fn_in_nest_mod()
        }
        // The `super` keyword refers to the parent scope of nest_mod 
        pub fn test_super(){
            super::pub_fn();
        }
    }
}

fn main(){
    module_name::pub_fn();
    module_name::public_function_in_crate();
    module_name::nest_mod::pub_fn_in_nest_mod();
    module_name::nest_mod::test_self();
    module_name::nest_mod::test_super();
}