// Shouldn't panic over unwind boundary.
startup::on_startup! { panic!(); }

#[no_mangle]
pub extern "C" fn startup_testcase() -> bool {
    true
}
