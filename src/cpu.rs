/// ```rust
/// assert_eq!(2, add(2,3));
/// ```
fn add(a: i32, b: i32) -> i32{
    a + b
}


fn merge_opcodes(first: u8, second: u8) -> u16{
    let f = first as u16;
    let s = second as u16;
    let r = (f << 8) | s;

    println!("0x{:x} 0b{:b}", r, r);

    r
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_apa() {
        assert_eq!(merge_opcodes(0x1A, 0xB2), 0x1AB2);
    }
}
