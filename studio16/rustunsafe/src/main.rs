unsafe fn swap<T: Copy>(x: *mut T, y: *mut T) {
    let z = *x;
    *x = *y;
    *y = z;
}

fn main() {
    let mut x = 4;
    let mut y = 33;

    println!("x = {x}, y = {y}");

    let x_ptr: *mut i32 = &mut x;
    let y_ptr: *mut i32 = &mut y;
    unsafe {
        swap(x_ptr, y_ptr);
    }

    println!("x = {x}, y = {y}");
}
