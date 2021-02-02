fn main() {
    let path = std::env::args().skip(1).next().expect("expected arg");
    let sofile = format!("{}.so", path);
    let dll = format!("{}.dll", path);
    let dylib = format!("{}.dylib", path);
    let file: std::path::PathBuf = if exists(&sofile) {
        sofile.into()
    } else if exists(&dll) {
        dll.into()
    } else if exists(&dylib) {
        dylib.into()
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
    fn exists(p: impl AsRef<std::path::Path>) -> bool {
        p.as_ref().exists()
    }
}
