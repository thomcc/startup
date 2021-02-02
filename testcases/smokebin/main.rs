use core::sync::atomic::*;
static VAL: AtomicU8 = AtomicU8::new(0);
startup::on_startup! { VAL.fetch_add(1, Ordering::Relaxed); }
startup::on_startup! { VAL.fetch_add(2, Ordering::Relaxed); }
startup::on_startup! { VAL.fetch_add(4, Ordering::Relaxed); }

fn main() {
    assert_eq!(VAL.load(Ordering::Relaxed), 1 + 2 + 4 + 8 + 16);
    assert_eq!("a b c 123", &*EXAMPLE);
    assert_eq!("foobar.baz", &*VALUE);
    assert_eq!("foobar.baz", &*VALUE2);
}

startup::on_startup! { VAL.fetch_add(8, Ordering::Relaxed); }

startup::eager_static! {
    pub static ref EXAMPLE: String = {
        VAL.fetch_add(16, Ordering::Relaxed);
        format!("a b c {}", 123)
    };
}

startup::eager_static! {
    static ref PREV: String = "foobar".to_string();
    static ref VALUE: String = format!("{}.{}", &*PREV, &*NEXT);
    static ref NEXT: String = "baz".to_string();
}

startup::eager_static! {
    static ref PREV2: String = "foobar".to_string();
}

startup::eager_static! {
    static ref VALUE2: String = format!("{}.{}", &*PREV2, &*NEXT2);
}

startup::eager_static! {
    static ref NEXT2: String = "baz".to_string();
}

