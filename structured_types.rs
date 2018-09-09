fn structured_types() {
    let t = (5, "cat");
    let num = t.0;
    let str = t.1;

    // pub fn pipe() -> Result<(RawFd, RawFd)>

    struct coordinate {
        x: f32,
        y: f32,
    };

    enum weekdays {
        Monday,
        Tuesday,
        Wednesday,
    }
}


fn references() {
    // References

    // Think of them as C/C++ pointers, but safe!
    let x = 3;
    let r = &x;

    // First version.
    // fn add1(r: &i32) {
        // r += 1;
    // }

    // Second version
    // fn add1(r: &i32) {
        // *r += 1;
    // }

    // Working version
    fn add1(r: &mut i32) {
        *r += 1;
    }

    // add1(r);
}

fn boxes() {
    // Java and Python
    // Allocated in Heap
    let t = (12, "eggs");

    // When out of scope, memory is freed.
    // Unless moved
    let b = Box::new(t);

    // Raw pointers.
    let x = 3;
    let raw: *const i32 = &3;
}

fn main() {}
