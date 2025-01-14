fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x

    *y += 1; // this works fine
    *z +=1; // this causes a compile time error
}