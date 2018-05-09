pub fn solve() -> i32 {
    let mut p:Vec<i32> = Vec::new();
    let mut n:i32 = 3;

    p.push(2);
    while p.len() < 10001 {
        let mut i:usize = 0;
        while p[i] * p[i] <= n {
            if n % p[i] == 0 {
                break;
            }
            i += 1;
        }

        if p[i] * p[i] > n {
            p.push(n);
        }

        n += 1;
    }



    p[10000]
}