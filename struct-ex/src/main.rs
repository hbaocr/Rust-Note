struct Human{
    name:String,
    age:i8,
    thought:String
}

impl Human{
    fn new(_name:&str,_age:i8)-> Human{
        return Human{
            name:_name.to_string(),
            age:_age,
            thought:String::from("nothing")

        }
    }
    fn without_thought(mut self,_thought:&str)->Human{
        
            self.thought =_thought.to_string();
            return self;
    }
    fn print(&self){
        println!("{},{},{}",&self.age,&self.name,&self.thought)
    }
}

fn main() {
    println!("Hello, world!");
        
    let mut dev:Human = Human::new("John",19);
    {
        let t:&str ="Hello The word";
         dev =Human::without_thought(dev,t);
    }
    
    dev.print()
}
