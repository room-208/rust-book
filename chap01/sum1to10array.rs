fn main() {
    //let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut sum = 0;
    for i in nums {
        sum += i;
    }
    println!("{}", sum);
}
