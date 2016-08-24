pub fn fac(n: i32) -> i32 {
    if n <= 1 { 1 }
    else { n * fac(n-1) }
}
