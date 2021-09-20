
## 1. Create Project
* Create `Helloword` Project and run with Cargo:  
    * Create Project:`cargo new  Helloword `
    * Run Project: `cd Helloword` then `cargo run`

* `let` can only be used in function  

* Install `crate` (the dependencies lib like `npm`) : `cargo install <lib name>`

* Uninstall `crate` (the dependencies lib like `npm`) : `cargo uninstall <lib name>`

* All needed dependencies info are added in `Cargo.toml` like `package.json` in Nodejs

* The version of needed dependencies info are locked in `Cargo.lock` like `package-lock.json` in Nodejs

* If we want to build a release, we simply run the following command: `cargo build --release`

* Create a project lib : `cargo new <lib_name> --lib`

* The `comment` in rust support `markdown` syntax when generate the document automatically
    * The markdown syntax  is use after `///` ( 3 slash marker)
    * Run `cargo doc` to gen all the markdown comment in the source code
    * Run `cargo doc --open`  to open the generated doc

## 2. String type `str`,`&str` and `String` and `&String`
* [web.mit.edu/rust-lang_v1.25](http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/strings.html)
* [https://doc.rust-lang.org/book/ch04-03-slices.html](https://doc.rust-lang.org/book/ch04-03-slices.html)


* **Figure 1:**
    * `s` is the `String`. It has  these attribute : {`ptr`, `len`, `capacity`}. 
    * `world` is the `slice string` ,`&str`. It only has these attribute : {`ptr`, `len`}
    * `&String` can `automatically coerce` to a `&str`. This is a feature called [Deref coercions](http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/deref-coercions.html).
    * Note that you normally **cannot access** a `str` directly
     
![slice string](img/SliceString.png)

```rust
    let ref_literal_str:&str ="hello world";
    let s1:String = ref_literal_str.to_string();

    let s:String = String::from(ref_literal_str); 
    let world:&str = &s[6..10];
    let ref_literal_str:&str = s.as_str();
```


* A `string slice` is a reference to part of a String, and it looks like this: `let world:&str = &s[6..10];`

    * A `&str` is `string slice`. A string slice has a fixed size, and cannot be mutated. It is a reference to a sequence of UTF-8 bytes.

    * The `"hello world"` is a `string literal` and its type is `&'static str`. A string literal is a string slice that is statically allocated, meaning that it’s saved inside our compiled program, and exists for the entire duration it run.

    * `str` is actually the literal string in heap. Note that you normally **cannot access** a `str` directly, but only through a `&str` reference. This is because str is an [unsized type](http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/unsized-types.html) which requires additional runtime information to be usable

* A `String` is a `heap-allocated` string. This string is growable, and is also guaranteed to be UTF-8. Strings are commonly created by converting from a `string slice` using the `to_string` method. It has  these attribute : {`ptr`, `len`, `capacity`}  see **s** in **figure 1** .
    * `String` is the abstract of `Vect` type. So `String` variable is just the `Vector` array of `char`. This vector holds a reference to `str` and string size and string capacity  . `Rust` always pre-know size of `String` when compiling time
    * Can convert String to the other type by calling these built-in method
        * To slice string `&str`  : `.as_str()`
        * To array of u8  `&[u8]` : `.as_bytes()`
* **Concatenation** 
    * If you have a `String`, you can concatenate a `&str` to the end of it:
    ```rust
        let hello:String = "Hello".to_string();
        let world:&str = "world";
        let hello_world:String = hello + world;
    ```
    * But if you have two `Strings`, you need an `&`
    ```rust
        let hello:String = "Hello ".to_string();
        let world:String = "world!".to_string();
        let hello_world:String = hello + &world;
    ```
    * This is because `&String` can automatically coerce to a `&str`. This is a feature called `Deref coercions`.
* **Unsize Type** : Rust understands a few of these types, but they have some restrictions. There are three:
    * We `can only manipulate` an instance of an `unsized type via a pointer`. An `&[T] works fine`, but a [`T] does not`.
    * `Variables and arguments cannot have dynamically sized types`.
    * Only the last field in a struct may have a dynamically sized type; the other fields must not. Enum variants must not have dynamically sized types as data.

## 3. Object Type:
### Struct
* Declare `Struct Hello{ num1:i8,num2:i8,st:String};`
* We can use struct as the same as `Class` in C by `impl` more method for `Struct`
    * `impl Hello{ fn method1(){} .....}`
* `self` === `this` in CPP or `self` in Python

### Enum

* Declare `Enum EnumExp{ S(String) , I(i8)};`
* We can  also use `impl`to  implement method for `Enum`
* `self` === `this` in CPP or `self` in Python

### Interface:

* The class interface methodology can be implemented using `trait` impl for `struct`
```rust
struct StructEx{
    var1:i8,
    var2:i8
}
trait TraitName{
    fn interface_trait_method(&self){
        // do sth here
    }
}
// apply trait for struct ==> sothat "StructEx" can use the "interface_trait_method" in trait
impl TraitName for StructEx{}
```

## 4. Metaprogramming with macros

* Metaprogramming can generally be described as a way in which the program can manipulate itself based on certain instructions


* [Derive macros](https://doc.rust-lang.org/rust-by-example/trait/derive.html) can be analogous to `decorators in JavaScript and Python`. They sit `on top` of `structs`, `enums` an `union` and change its functionality
    * `#[derive(Clone, Copy)]` macro on top of `struct Coordinate`  to implement the trait `Clone` and `Copy` on `Struct Coordinate` so that `Coordinate` can use these trait to copy and clone the object. 
        * Note that: if the Object struct implement trait `Clone` and `Copy`, we pass it through the functions it will copy this object struct to use inside these functions like the `primitive` type (`i8`,`u8`,...) instead fo take `ownership` of it.
        ```rust
        #[derive(Clone, Copy)]
        struct Coordinate {
            x: i8,
            y: i8
        }
        ```
    * `#[derive(Serialize, Deserialize)]` macro on top of `struct Coordinate`  to implement the trait `Serialize` and `Deserialize` on `Struct Coordinate` so that we can pass coordinate into the `crate's functions` to `serialize` into JSON, and then `deserialize`.  
    
        ```rust
        use serde::{Serialize, Deserialize};
        #[derive(Serialize, Deserialize)]
        struct Coordinate {
            x: i8,
            y: i8
        }
        ```
    * **Note** : `derive` only works for `structs`, `enums` an `union` type definitions and it then applies the `SomeName` procedural macro (called a derive macro) to the source code that makes the type definition. The macro can then output / spit / emit its own forged source code, **that will be emitted next to the original type definition source, which remains unaffected by the macro**

* [Attribute like macro](https://doc.rust-lang.org/book/ch19-06-macros.html#attribute-like-macros) are similar to c`ustom derive macros`, but instead of generating code for the derive attribute, they allow you to `create new attributes`. They’re also more flexible: **derive only works for structs and enums** however **attributes can be applied to other items as well, such as functions**.

    * `#[route(GET, "/")]` is attribute named route that annotates functions when using a web application framework
        ```rust
        #[route(GET, "/")]
        fn index() {}
        ```
    * This `#[route] attribute` would be defined by the framework as a procedural macro. The signature of the macro definition function would look like this:
        ```rust
        // using #[proc_macro_attribute] to declare an attribute like procedural macro

        #[proc_macro_attribute]
        pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
        ```
    * Here, we have two parameters of type `TokenStream`. The first is for the contents of the attribute: the `GET, "/"` part. The second is the body of the item the attribute is attached to: in this case, `fn index(){}` and the rest of the function’s body.

    * Other than that, attribute-like macros work the same way as `custom derive macros`: you create a crate with the proc-macro crate type and implement a function that generates the code you want!
    * **Note :** `#[some_name]` is an attribute that can be applied to `any Rust item` (and in the future, even Rust expressions and statements (and maybe even types and patterns)), including functions. It applies the **procedural macro** some_name to the source code that makes the item definition. That input source code `is not re-emitted, so the attribute macro has all the power to decide what gets emitted instead.` It takes arbitrary input and `returns valid Rust code`. There are 3 type of `procedural macros`
        * Attribute-like macros
        * Derive macros
        * Function-like macros

* See more [macro example](https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/) and [procedural macros](https://blog.logrocket.com/procedural-macros-in-rust/)



## 5. Modules 

### Module : `mod`
A module is a collection of items: `functions`, `structs`, `traits`, `impl` blocks, and even other `modules`.
* Items in modules default to `private` visibility.
* Use the `pub` modifier to override default visibility.
* [link](https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html)

```rust
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
```

### File Hierachy

`Modules` can be `mapped` to a `file/directory` hierarchy. Call `tree .` to list the Hierachy. Here is the structure of the project
```
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── main.rs                  ---> main app src: include `mod my_module` 
│   ├── my_single_file_mod.rs    ---> single file mod, is used when declaring by mod my_single_file_mod in main.rs
│   └── my_module                ---> folder name is module name
│       ├── mod.rs               ---> this is the entry point of my_module
│       ├── private_nest_mod.rs  ---> nest module, declared as priv module in mod.rs
│       └── public_nest_mod.rs   ---> nest module, declared as pub module in mod.rs
```
* To create your own module as the single file:
    * 1. create my_single_file_mod.rs
    * 2. import then use by `mod my_single_file_mod` in `main.rs`
* To create your own module as the folder named `my_module`:
    * 1. Create folder which name is the name of your module :  `my_module`
    * 2. Create `my_module/mod.rs`  as the entry point of your lib `my_module`.
    * 3. Write your own code in `my_module/mod.rs`: you can declare your public/private function or public/private module in this file or in separated file. Each file as the `nest module` of `my_module`
    * 4. To use `my_module` in `main.rs` declare `mod my_module`. This will let the `main.rs` use all the `public function and nest module` declaring in `my_module/mod.rs`

* `main.rs`:
    ```rust
    mod my_module; // include my_module as folder
    mod my_single_file_mod; // include  single file module
    fn main() {
        my_single_file_mod::pub_single_file_fn();
        my_module::pub_fun_in_mymodule();
        my_module::public_nest_mod::pub_fun();
    }
    ```
* `my_single_file_mod.rs`:
    ```rust
    pub fn pub_single_file_fn(){
        println!("pub_single_file_fn")
    }
    ```
* `my_module/mod.rs`:
    ```rust
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
    ```
* `my_module/private_nest_mod.rs`: this file is declared as private module in `mod.rs`
    ```rust
    // can call outside : in mod.rs
    pub fn pub_fun() {
        println!("private_nest_mod::pub_fun");
    }
    // only can call on this file
    fn priv_fun() {
        println!("private_nest_mod::priv_fun can not call");
    }
    ```
 
* `my_module/pub_nest_mod.rs`: this file is declared as public module in `mod.rs`
    ```rust
    // can call outside : in mod.rs
    pub fn pub_fun() {
        println!("pub_nest_mod::pub_fun");
    }
    // only can call on this file
    fn priv_fun() {
        println!("pub_nest_mod::priv_fun can not call");
    }
    ```

* Note that, you can use `mod` to load the module then use `use` to load function or module in side the loaded module by `mod` 
```rust
// main.rs
mod my_module; // declare my_module first
use my_module::pub_fun_in_mymodule; // use to load the mod/fn inside declared my_module
fn main() {
    println!("Hello, world!");
    pub_fun_in_mymodule();// call without my_module::pub_fun_in_mymodule()
    my_module::public_nest_mod::pub_fun();
}
```