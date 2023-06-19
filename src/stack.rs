type StackNodeRef<T> = Option<Box<StackNode<T>>>;

struct StackNode<T> {
    value: Option<T>,
    next: StackNodeRef<T>,
}

struct Stack<T> {
    head: StackNodeRef<T>,
    pub length: usize,
}

impl<T> Stack<T> {
    pub fn new(value: Option<T>) -> Self {
        match value {
            Some(value) => {
                let head = StackNode {
                    value: Some(value),
                    next: None,
                };
                Stack {
                    head: Some(Box::new(head)),
                    length: 1,
                }
            }
            None => Stack {
                head: None,
                length: 0,
            },
        }
    }

    pub fn push(&mut self, value: T) -> &Self {
        let next = self.head.take();
        let head = StackNode {
            value: Some(value),
            next,
        };

        self.head.replace(Box::new(head));
        self.length += 1;

        self
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take();
        match head {
            Some(head) => {
                self.head = head.next;
                self.length -= 1;
                head.value
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(head) => head.value.as_ref(),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let mut stack = Stack::new(Some(45));

        assert_eq!(stack.pop(), Some(45));
    }

    #[test]
    fn push_length() {
        let mut stack = Stack::new(Some(45));
        stack.push(46);
        stack.push(48);

        assert_eq!(stack.length, 3)
    }

    #[test]
    fn pop_length() {
        let mut stack = Stack::new(Some(45));
        stack.push(46);
        stack.push(48);

        assert_eq!(stack.length, 3);
        stack.pop();
        stack.pop();
        assert_eq!(stack.length, 1);
    }
    #[test]
    fn pop_value() {
        let mut stack = Stack::new(Some(45));
        stack.push(46);
        stack.push(48);
        stack.pop();
        stack.push(53);
        stack.pop();
        let last_value = stack.pop();

        assert_eq!(last_value, Some(46));
    }

    #[test]
    fn peek() {
        let mut stack = Stack::new(Some(45));
        stack.push(46);
        stack.push(48);

        assert_eq!(stack.peek(), Some(&48));
    }
}
