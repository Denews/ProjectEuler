pub fn solve() -> i32 {
    let N : isize = 600851475143;

    let sn:i32 = (N as f64).sqrt() as i32 + 1;

    let mut ch:Vec<bool> = Vec::new();

    let mut lf:i32 = 2;

    for _i in 0..sn {
        ch.push(false);
    }

    for _i in 2..sn {
        if ch[_i as usize] == false {
            let i:i32 = _i as i32;

            if N % (i as isize) == 0 {
                lf = i;
            }

            let mut j:i32 = 2;

            while i * j < sn {
                ch[(i * j) as usize] = true;
                j += 1;
            }
        }
    }

    lf
}