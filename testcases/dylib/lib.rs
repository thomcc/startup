use core::sync::atomic::*;

static SOMETHING: AtomicU8 = AtomicU8::new(0);

startup::on_startup! { SOMETHING.fetch_add(1, Ordering::Relaxed); }
startup::on_startup! { SOMETHING.fetch_add(2, Ordering::Relaxed); }
startup::on_startup! { SOMETHING.fetch_add(4, Ordering::Relaxed); }

#[no_mangle]
pub extern "C" fn startup_testcase() -> bool {
    SOMETHING.load(Ordering::Relaxed) == (1 + 2 + 4 + 8)
}

startup::on_startup! { SOMETHING.fetch_add(8, Ordering::Relaxed); }
