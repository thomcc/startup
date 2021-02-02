use core::sync::atomic::*;
static VAL: AtomicU8 = AtomicU8::new(0);
startup::on_startup! { VAL.fetch_add(1, Ordering::Relaxed); }
startup::on_startup! { VAL.fetch_add(2, Ordering::Relaxed); }
startup::on_startup! { VAL.fetch_add(4, Ordering::Relaxed); }

fn main() {
    assert_eq!(VAL.load(Ordering::Relaxed), 1 + 2 + 4 + 8);
}

startup::on_startup! { VAL.fetch_add(8, Ordering::Relaxed); }
