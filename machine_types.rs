#[allow(dead_code)]
fn machine_types() {
    // Why do we have specific sizes instead of the C/C++ approach?
    // https://stackoverflow.com/questions/11438794/is-the-size-of-c-int-2-bytes-or-4-bytes
    // "it depends" = bad

    // Alternative: Let's say all C ints must be 32 bits!
    let n: i8 = 0;
    let m: i16 = 0;
    let o: i32 = 0;
    // Different size literals
    let p = 0i64;

    // Unsigned versions.
    let n = 0u32;

    // Machine dependent types
    let n: isize = 40;
    // Indexer!
    let i: usize = 40;


    let i: i32 = 3;
    let array = [1, 2, 3];

    array[i as usize];

    // Chars and bools are not ints!

    // floats, bools, chars.
}

fn main() {
    println!("Hello World!");
}
