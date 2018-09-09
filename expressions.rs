fn main() {}

fn unit_type() -> () {
    println!("hello");

    // Type of this function?
    return;
}

fn if_expr(b : bool) {
    let x: i32 =
        if b {
            5
        } else {
            10
        };
}

fn match_expr(x: Option<i32>) {
    match x {
        Some(n) => println!("{}", n),
        None => (),
    }

    let i = match x {
        Some(n) => n,
        _ => 5,
    };
}

fn if_let(x: Option<i32>) {
    if let Some(n) = x {
        println!("{}", n);
    }
}

fn while_expr(b: bool) {
    while b {
        unimplemented!()
    }

    loop {
        // Do things.
    }
}

fn for_expr() {

    // Called Ranges, exclusive
    for i in 0..20 {
        println!("{}", i);
    }

    let v: Vec<&str> = vec!["cis", "198", "rust", "programming"];
    for e in v {
        let e2: &str = e;
        // Do stuff with v
    }
}

fn function_calls(v: Vec<i32>) {
    let v2 = v.clone();
    let mut v3: Vec<i32> = Vec::new();
    v3.push(4);

    // turbo.fish
    // let v4 = Vec::<i32>::new();

    // https://doc.rust-lang.org/std/collections/struct.HashMap.html
    use std::collections::HashMap;

    // Static method vs method
    let mut ht: HashMap<i32, &str> = HashMap::new();
    // takes mutable reference.
    ht.insert(1, "one");
    // takes reference.
    ht.is_empty();
}

// NOTE: Will NOT talk about in class.
fn reference_dereference(s: &mut String) {

    fn f(i: i32){
        unimplemented!()
    }

    let v = vec![0; 10];
    for i in & v {
        f(*i);
    }

    s.push('c');
}

fn closure() {
    // Anonymous functions, capture enviornment.
    let x = 3;
    let f = |y| y + 3;
    assert_eq!(f(5), 8);

    let is_even = |x: i32| -> bool {x % 2 == 0};

    // Type inference.
    let vec : Vec<i32> = vec![1, 2, 3, 4, 5];
    let vec2 = vec.into_iter().filter(|x| x % 2 == 0).collect::<Vec<i32>>();

    // let ht: HashMap<i32, i32> = v.clone().into_iter().map(|x| (x, x)).collect();
}
