/// Simple reference-counting garbage collector with cycle detection
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashSet;

/// Garbage-collected pointer
#[derive(Debug, Clone)]
pub struct GcPtr {
    inner: Rc<RefCell<GcObject>>,
}

#[derive(Debug)]
struct GcObject {
    data: Vec<u8>,
    refs: Vec<GcPtr>,
    marked: bool,
}

impl GcPtr {
    pub fn new(data: Vec<u8>) -> Self {
        GcPtr {
            inner: Rc::new(RefCell::new(GcObject {
                data,
                refs: Vec::new(),
                marked: false,
            })),
        }
    }
    
    pub fn data(&self) -> Vec<u8> {
        self.inner.borrow().data.clone()
    }
    
    pub fn add_ref(&self, ptr: GcPtr) {
        self.inner.borrow_mut().refs.push(ptr);
    }
}

/// Garbage collector
pub struct GarbageCollector {
    roots: Vec<GcPtr>,
    threshold: usize,
    allocated: usize,
}

impl GarbageCollector {
    pub fn new() -> Self {
        GarbageCollector {
            roots: Vec::new(),
            threshold: 1024 * 1024, // 1MB
            allocated: 0,
        }
    }
    
    pub fn alloc(&mut self, data: Vec<u8>) -> GcPtr {
        self.allocated += data.len();
        let ptr = GcPtr::new(data);
        self.roots.push(ptr.clone());
        
        if self.allocated > self.threshold {
            self.collect();
        }
        
        ptr
    }
    
    /// Mark-and-sweep collection
    pub fn collect(&mut self) {
        // Mark phase
        self.mark();
        
        // Sweep phase
        self.sweep();
        
        // Update threshold
        self.threshold = self.allocated * 2;
    }
    
    fn mark(&self) {
        let mut visited = HashSet::new();
        for root in &self.roots {
            self.mark_recursive(root, &mut visited);
        }
    }
    
    fn mark_recursive(&self, ptr: &GcPtr, visited: &mut HashSet<usize>) {
        let addr = Rc::as_ptr(&ptr.inner) as usize;
        if visited.contains(&addr) {
            return;
        }
        visited.insert(addr);
        
        ptr.inner.borrow_mut().marked = true;
        
        for child in &ptr.inner.borrow().refs {
            self.mark_recursive(child, visited);
        }
    }
    
    fn sweep(&mut self) {
        self.roots.retain(|ptr| {
            let marked = ptr.inner.borrow().marked;
            if marked {
                ptr.inner.borrow_mut().marked = false;
                true
            } else {
                self.allocated -= ptr.inner.borrow().data.len();
                false
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gc_alloc() {
        let mut gc = GarbageCollector::new();
        let ptr = gc.alloc(vec![1, 2, 3]);
        assert_eq!(ptr.data(), vec![1, 2, 3]);
    }
    
    #[test]
    fn test_gc_collect() {
        let mut gc = GarbageCollector::new();
        let _ptr1 = gc.alloc(vec![1; 100]);
        let _ptr2 = gc.alloc(vec![2; 100]);
        gc.collect();
        assert!(gc.allocated > 0);
    }
}
