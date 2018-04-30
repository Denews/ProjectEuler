use num::pow;

pub fn solve() -> i32 {
    let plist = [2, 3, 5, 7, 11, 13, 17, 19];
    let mut result:i32 = 1;

    for i in plist.iter() {
        result *= pow(*i, (20 as f64).log(*i as f64).floor() as usize);
    }

    result
}