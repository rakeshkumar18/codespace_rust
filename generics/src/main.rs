// generics


fn square<T>  (x: T) -> T 
where T : std::ops::Mul<Output = T> + Copy + std::ops::Add<Output = T> {
    x * x + x * x
}

fn main() {
    println!("The square of the number is {}", square(3.4) );
}
