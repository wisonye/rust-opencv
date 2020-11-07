# OpenCV in Rust

## Install `OpenCV` via `brew`

```bash
# `update` first is important, it will solve update some
# dependencies before build `opencv`!!!
#
# By default, it will install `opencv4`. If you want `3`,
# then run `brew install opencv@3`
brew update && brew install opencv
```

## About the `DYLD_FALLBACK_LIBRARY_PATH` environment variable

Pay attention on MacOSX:

After finishing `brew update && brew install opencv`, run the command below to make
sure you've set the `DYLD_FALLBACK_LIBRARY_PATH` env correct. Otherwise, you will
get the `dyld: Library not loaded: @rpath/libclang.dylib` error when building.

```bash
ls -lht $DYLD_FALLBACK_LIBRARY_PATH/libclang.dylib
```
   
If it prints ok, that's fine. Otherwise, set it like below into your shell
config (example below is `Fish` shell config syntax):

```bash
# This is the standard path, but not works for me
# set DYLD_FALLBACK_LIBRARY_PATH (xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/

# This is my case, make sure you fill the correct path
set DYLD_FALLBACK_LIBRARY_PATH (xcode-select --print-path)/usr/lib/
```

Also, the tricky thing is that `cargo run` works fine in fish shell, but `cargo watch --exec run` will
fail with `dyld: Library not loaded: @rpath/libclang.dylib` error. It seems the
`DYLD_FALLBACK_LIBRARY_PATH` doesn't exist in the child process, have idea on that yet.
   

## How to run examples

```bash
cargo run --example show-image-in-window
cargo run --example video-capture-in-web-cam
cargo run --example video-capture-web-cam-with-face-detection
```
