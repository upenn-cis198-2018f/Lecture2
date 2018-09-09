fn main() {}

fn array() {
    let a = [1, 2, 3];
    let zeroes = [0; 1000];

    // Size is hardcoded
    let b: [i32; 3] = [1, 2, 3];

    // Dynamically allocated
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(3);
}

fn slices() {
    // Fat pointer: pointer to heap, length, capacity,
    let v = vec![1, 2, 3];

    let sv: &[i32] = &v;
    let av: &[i32] = &a;

    // Write functions that can take either.
    fn print(s: &[i32]){
        unimplemented!()
    }

    print(sv);
    print(av);
    // Rust knows how to convert vectors to arrays.
    print(&v);
    print(&v[0..1]);

    // array.png
}

fn strings() {
    // char*
    // Where does this reference live
    // .data
    let s: &str = "rust";

    // Why do we do this?
    // People don't like this about rust.

    // Heap allocated, string.
    // UTF-8, growable, modifyiable, guaranteed to be correct UTF-8
    let s: String = "rust".to_string();

    // Example in book
    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let poodles = "ಠ_ಠ"; // Pre allocated read-only memory.
    // noodles.png

    fn print(s: &str) {

    }

    // Talk about just str.

    // Talk about char* as an argument i C
    // void f(char* c) {
        // Is it null?
        // what's it's length?
        // Can I modify it?
    // }

    // String vs array.png
}

