fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s == s.chars().rev().collect::<String>()
}

#[test]
fn prr_10test() {
    for &num in &[123, 121, 1221] {
        println!("{}: {}", num, is_palindrome(num));
    }
}