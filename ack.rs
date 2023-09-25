#[no_mangle]
pub fn ack(m: usize, n: usize) -> usize {
    match (m, n) {
        (0, n) => n + 1,
        (m, 0) => ack(m - 1, 1),
        (m, n) => ack(m - 1, ack(m, n - 1)),
    }
}
