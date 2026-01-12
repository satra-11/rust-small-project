fn return_first_word_followed_by_space (target: &str) -> &str {
for i in 0..target.len() {
    if target.chars().nth(i) == Some(' ') {
        println!("{}", i);
        return &target[0..i];
    }
}
}
fn main () {
    let target = "Hello World";
    let ans = return_first_word_followed_by_space(&target);
    println!("{}", ans);
}