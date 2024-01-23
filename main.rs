fn main() {
    println!("Hello, world!");

    fibonaci(10);
}




fn fibonaci(n : u32) -> u64 {

    let mut num1 = 0;
    let mut num2 = 1;
    let mut count = 2;
    let mut sum = 0;
    while count < n {
        sum = num1 + num2;
        num1 = num2;
        num2 = sum;
        count += 1;
    }
    println!("{}" , sum);
    sum 
}