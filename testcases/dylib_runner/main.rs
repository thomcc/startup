fn main() {
    let mut args = std::env::args().skip(1);
    let path: std::path::PathBuf = args
        .next()
        .expect("expected target path")
        .into();
    let name = args
        .next()
        .expect("expected lib name");

    let sofile = path.join(format!("lib{}.som", name));
    let dll = path.join(format!("{}.dll", name));
    let dylib = path.join(format!("lib{}.dylib", name));
    let file: std::path::PathBuf = if sofile.exists() {
        sofile
    } else if dll.exists() {
        dll
    } else if dylib.exists() {
        dylib
    } else {
        panic!("couldnt find a dylib like {:?}", path);
    };
    let r = maybe_catch_unwind(name == "paniclib", || {
        let lib = libloading::Library::new(&file).unwrap_or_else(|e| {
            panic!("loading {:?} failed: {:?}", file, e);
        });
        unsafe {
            let func: libloading::Symbol<unsafe extern "C" fn() -> bool> =
                lib.get(b"startup_testcase\0").unwrap_or_else(|e| {
                    panic!("couldn't find symbol: {:?} in {:?}", e, file);
                });
            assert!(func());
        }
        lib.close().unwrap_or_else(|e| {
            panic!("dlclose failed for {:?}: {:?}", file, e);
        });
    });
    if r.is_err() {
        std::process::exit(99);
    }
}

fn maybe_catch_unwind(want_catch: bool, f: impl FnOnce()) -> std::thread::Result<()> {
    if want_catch {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(f))
    } else {
        f();
        Ok(())
    }
}
