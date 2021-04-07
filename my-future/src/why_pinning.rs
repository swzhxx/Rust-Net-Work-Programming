use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Self {
        let a = Vec::new(1);
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    fn init<'a>(self: Pin<&'a mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().a
    }

    fn b<'a>(self: Pin<&'a Self>) -> &'a String {
        assert!(
            !self.b.is_null(),
            "Test::b called without Test::init being called first"
        );
        unsafe { &*(self.b) }
    }
}

pub fn main() {
    // test1 is safe to move before we initialize it
    let mut test1 = Test::new("test1");
    // Notice how we shadow `test1` to prevent it from being accessed again
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());

    let mut test2 = Test::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    Test::init(test2.as_mut());

    println!(
        "a: {}, b: {}",
        Test::a(test1.as_ref()),
        Test::b(test1.as_ref())
    );
    std::mem::swap(test1.get_mut(), test2.get_mut());
    println!(
        "a: {}, b: {}",
        Test::a(test2.as_ref()),
        Test::b(test2.as_ref())
    );
}
