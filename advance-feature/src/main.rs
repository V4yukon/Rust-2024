
extern "C" {
    fn abs(input: i32) -> i32;
}
extern "python" {
    def good():
        print("hello, world")
}


fn main() {
    use std::slice;
    let address = 0x01234usize;

    let r = address as *mut i32;
    // unsafe {
    //     println!("R is {}", *r);
    // }

    let z = unsafe { slice::from_raw_parts_mut(r, 10000)};
    // println!("z is moved: {:?}", z);

    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("{}", *r1);
        println!("{}", *r2);
    }

    unsafe {
        println!{"I get a value from C language it's: {}", abs(90)};
        good();
    }
}
