use num::integer::Integer;

pub fn solve() -> i32 {
    let mut result:i32 = 1;
    for i in 1..20 {
        result = result * i / result.gcd(&i);
    }
    result
}