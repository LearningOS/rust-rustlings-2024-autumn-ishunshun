#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            self.data.pop()
        }
    }

    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.data.get(self.size - 1)
        }
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            self.data.get_mut(self.size - 1)
        }
    }
}

fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new();
    
    for char in bracket.chars() {
        match char {
            '(' | '{' | '[' => stack.push(char),
            ')' => {
                if stack.pop() != Some('(') {
                    return false; // Mismatched or unbalanced closing bracket
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false; // Mismatched or unbalanced closing bracket
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false; // Mismatched or unbalanced closing bracket
                }
            }
            _ => {}
        }
    }
    
    stack.is_empty() // If the stack is empty, all brackets are matched
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn bracket_matching_1(){
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }
    
    #[test]
    fn bracket_matching_2(){
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }
    
    #[test]
    fn bracket_matching_3(){
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), true);
    }
    
    #[test]
    fn bracket_matching_4(){
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }
    
    #[test]
    fn bracket_matching_5(){
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }
    
    #[test]
    fn bracket_matching_6(){
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}
