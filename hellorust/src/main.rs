fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    let space = "    ";
    let space = space.len();


    let tup: (i32,f64,u8) = (20,4.4,2);

    let arr: [i32;3] = [1,2,3];

    println!("The value of x is: {}", tup.1);
    println!("Hello, world!{}",arr[0]);
}
