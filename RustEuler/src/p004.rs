pub fn solve() -> i32 {
    for i in (0..1000).rev() {
        for j in 0..(1000 - i) {
            if isPalindrome((i + j) * (i - j)) {
                return (i + j) * (i - j);
            }
        }
    }
    0
}

fn isPalindrome(n : i32) -> bool {
    let str = n.to_string();
    let half = str.len() / 2;

    str.bytes().take(half).eq(str.bytes().rev().take(half))
}