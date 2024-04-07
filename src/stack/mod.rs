#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: vec![],
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

    fn pop(&mut self, val: T) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }
        self.data.get(self.size - 1)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        if self.size == 0 {
            return None;
        }
        self.data.get_mut(self.size - 1)
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: vec![] };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = Iter { stack: vec![] };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}

struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

mod BasicCauculator {}

#[cfg(test)]
mod test {
    #[test]
    fn check_cauculate() {}
}
