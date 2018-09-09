fn main() {
    println!("Hello World!");
}

// Initial code, all types.
#[allow(dead_code)]
fn build_vector_rough() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

/// Question: Can we omit the type of the function? No
#[allow(dead_code)]
fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}


// Final version.
#[allow(dead_code)]
fn build_vector_final() -> Vec<i16> {
    vec![10, 20]
}
