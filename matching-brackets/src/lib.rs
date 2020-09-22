pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    let mut vec = string.chars();
    for char in vec {
        match char {
            '{' | '[' | '(' => stack.push(char),
            ')' if stack.pop() != Some('(') => return false,
            ']' if stack.pop() != Some('[') => return false,
            '}' if stack.pop() != Some('{') => return false,
            _ => continue
        }
    }
    stack.is_empty()
}
