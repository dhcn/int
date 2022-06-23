use std::sync::{Arc, Mutex};
use std::thread;

#[cfg(test)]
mod test {
    use int::engine::trading_strategy;
    #[test]
    fn test_int() {
        println!("xxxxx------>");
    }
}

fn threads_test() {
    const COUNT: u32 = 10000;
    let global = Arc::new(Mutex::new(0));
    let clone1 = global.clone();
    let thread1 = thread::spawn(move || {
        for _ in 0..COUNT {
            let mut value = clone1.lock().unwrap();
            *value += 1;
        }
    });
    let clone2 = global.clone();
    let thread2 = thread::spawn(move || {
        for _ in 0..COUNT - 2 {
            let mut value = clone2.lock().unwrap();
            *value -= 1;
        }
    });
    thread1.join().ok();
    thread2.join().ok();
    println!("final value :{:?}", global);
}
fn lifetime_test() {
    struct T {
        dropped: bool,
    }
    impl T {
        fn new() -> Self {
            T { dropped: false }
        }
    }
    impl Drop for T {
        fn drop(&mut self) {
            self.dropped = true;
        }
    }
    struct R<'a> {
        inner: Option<&'a T>,
    }
    impl<'a> R<'a> {
        fn new() -> Self {
            R { inner: None }
        }
        fn set_ref<'b: 'a>(&mut self, ptr: &'b T) {
            self.inner = Some(ptr);
        }
    }
    impl<'a> Drop for R<'a> {
        fn drop(&mut self) {
            if let Some(ref inner) = self.inner {
                println!("droppen R when T is P{}", inner.dropped);
            }
        }
    }
    {
        let (a, mut b): (T, R) = (T::new(), R::new());
        b.set_ref(&a);
    }
    {
        let (mut a, b): (R, T) = (R::new(), T::new());
        //a.set_ref(&b);
    }
}
