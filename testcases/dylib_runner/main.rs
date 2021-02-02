fn main() {
    let path: std::path::PathBuf = std::env::args().skip(1).next().expect("expected arg").into();
    let sofile = path.join("libdylibtest.so");
    let dll = path.join("dylibtest.dll");
    let dylib = path.join("libdylibtest.dylib");
    let file: std::path::PathBuf = if sofile.exists() {
        sofile
    } else if dll.exists() {
        dll
    } else if dylib.exists() {
        dylib
    } else {
        panic!("couldnt find a dylib like {:?}", path);
    };
    {
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
    }
}
