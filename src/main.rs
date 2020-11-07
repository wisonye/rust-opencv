/// Pay attention on MacOSX:
///
/// 1. After finishing `brew update && brew install opencv`, run the command below to make
///    sure you've set the `DYLD_FALLBACK_LIBRARY_PATH` env correct. Otherwise, you will
///    get the `dyld: Library not loaded: @rpath/libclang.dylib` error when building.
///
///    ```bash
///    ls -lht $DYLD_FALLBACK_LIBRARY_PATH/libclang.dylib
///    ```
///    
///    If it prints ok, that's fine. Otherwise, add the env setting below into your shell
///    config (example below is `Fish` shell config syntax):
///
///    ```bash
///    # This is the standard path, but not works for me
///    # set DYLD_FALLBACK_LIBRARY_PATH (xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/
///
///    # This is my case, make sure you fill the correct path
///    set DYLD_FALLBACK_LIBRARY_PATH (xcode-select --print-path)/usr/lib/
///    ```
///
///    Also, the tricky thing is that `cargo run` works in fish shell, but `cargo watch -exec` will
///    fail with `dyld: Library not loaded: @rpath/libclang.dylib` error. It seems the
///    `DYLD_FALLBACK_LIBRARY_PATH` doesn't exists on child process, don't know why yet!!!
///    
fn main() {
    println!("OpenCV Hello world:)");
}
