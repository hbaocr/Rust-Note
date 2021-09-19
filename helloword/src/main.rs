use std::collections::HashMap;
use std::option::Option;

fn test_hash_map(){
    let mut general_amp:HashMap<&str,i8>=HashMap::new();
    general_amp.insert("key1",1);
    let dt_out:Option<&i8> = general_amp.get("key2");
    match dt_out{
        None => println!("None"),
        Some(r)=> println!("{}",r)

    }

}

fn test_vector(){
    let str_vector: Vec<&str> = vec!["one", "two", "three"];
    println!("Hello, world!");
    for i in str_vector.iter() {
        println!("{}", i);
    }
}

fn err_check(check:bool)->Result<i8,&'static str>{
    if check==true {
        Err("this is err")
    }else{
        Ok(100)
    }
}




fn main() {
    test_vector();
    test_hash_map();
    let r:i8 =err_check(true).unwrap();
    println!("{}",r)
}
