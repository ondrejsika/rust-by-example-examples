#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!("Now {:?} will print!", 42);
    println!("Now {:?} will print!", Structure(42));
    println!("Now {:?} will print!", Deep(Structure(42)));
}