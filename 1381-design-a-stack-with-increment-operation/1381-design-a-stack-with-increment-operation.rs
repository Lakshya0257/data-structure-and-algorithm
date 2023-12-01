struct CustomStack {
    stack: Vec<(i32, i32)>,
    max_size: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {

    fn new(maxSize: i32) -> Self {
        Self {
            stack: Vec::new(),
            max_size: maxSize as usize,
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.stack.len() < self.max_size {
            self.stack.push((x, 0));
        }
    }
    
    fn pop(&mut self) -> i32 {
        if self.stack.is_empty() {
           return -1;
        } 
        let top = self.stack.pop().unwrap();
        self.increment(self.stack.len() as i32, top.1);
        top.0 + top.1
    }
    
    fn increment(&mut self, k: i32, val: i32) {
        if self.stack.is_empty() {
            return;
        }
        let n = self.stack.len();
        if n < k as usize {
            self.stack[n - 1].1 += val;        
        } else {
            self.stack[k as usize - 1].1 += val;
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */