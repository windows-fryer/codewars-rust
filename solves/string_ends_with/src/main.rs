fn solution(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}

fn main() {
    assert_eq!(true, solution("abc", "c"));
    assert_eq!(false, solution("strawberry", "banana"));
}
