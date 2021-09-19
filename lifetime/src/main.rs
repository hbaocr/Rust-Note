fn get_higher<'a>(first: &'a i8, second: &'a i8) -> &'a i8 {
    if first > second {
        first
    } else {
        second
    }
}

fn test1() {
    println!("Hello, world tes1 the same lifetime");
    let one: i8 = 1;
    {
        let two: i8 = 2;
        let out: &i8 = get_higher(&one, &two);
        println!("{}", out)
    }
}

fn filter<'a, 'b>(first: &'a i8, second: &'b i8) -> &'a i8 {
    if first < second {
        &0
    } else {
        first
    }
}

fn test2() {
    println!("Hello, world test2 diff lifetime");
    let one: i8 = 1;
    let out: &i8;
    {
        let two: i8 = 2;
        out = filter(&one, &two);
    }
    println!("{}", out)
}

fn filter1<'a, 'b>(first_number: &'a i8, second_number: &'b i8) -> &'a i8 {
    if first_number < second_number {
        &0
    } else {
        first_number
    }
}

fn main() {
    let one: i8 = 1;
    let outcome: &i8;
    {
        let two: i8 = 2;
        outcome = filter1(&one, &two);
    }
    println!("{}", outcome);
}
