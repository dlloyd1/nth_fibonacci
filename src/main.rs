use std::io;

fn get_nth_fib(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    

    let mut seq: Vec<i32> = Vec::new();
    for i in 0..n {
        if i == 0 || i == 1 {
            seq.push(i);
        } else {
            seq.push(seq[(i-1) as usize] + seq[(i-2) as usize]);
        }
    }
    println!("seq {:?}", seq);
    seq[(n-1) as usize]
}

fn main() {
    println!("Nth Fibonacci generator");
    println!("Enter target fibonacci sequence number: ");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Could not read line from std in");
    let n: i32 = n.trim().parse().expect("Enter an integer only");

    let result: i32 = get_nth_fib(n);

    println!("{}th fibonacci number in the sequence was: {}", n, result);
}
