pub fn solve() -> i32 {
    let border = 4000000;
    let mut sum = 0;
    let mut f1 = 1;
    let mut f2 = 1;

    while f1 < border && f2 < border {
        if (f1 + f2) % 2 == 0 {
            sum += f1 + f2;
        }
        if f1 < f2 {
            f1 = f1 + f2;
        }
        else {
            f2 = f1 + f2;
        }
    }

    sum
}