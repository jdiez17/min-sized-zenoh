# `min-sized-zenoh`

This repository is a demo of how to minimize the size of applications using Zenoh + Tokio.
There are two crates:

- `deps` (dylib): bundles tokio and zenoh into a single library
- `min-sized-zenoh` (bin): uses `deps` to send a single Zenoh message to the key `hello_world`.

To compile, run:

    $ RUSTFLAGS="-Cprefer-dynamic" cargo build --release

This generates two files in `target/release`:

    $ ls -l target/release/{libdeps.so,min-sized-zenoh}
    -rwxr-xr-x. 2 jdiez jdiez 22864288 Mar 31 11:00 target/release/libdeps.so
    -rwxr-xr-x. 2 jdiez jdiez   478160 Mar 31 11:00 target/release/min-sized-zenoh

`libdeps.so` is ~22MB and `min-sized-zenoh` is ~478kB.

First, make a copy of the original:

    $ cp target/release/min-sized-zenoh target/release/min-sized-zenoh.orig

Now let's make a change to `min-sized-zenoh/src/main.rs` (for example, the message that is sent) and recompile:

    $ RUSTFLAGS="-Cprefer-dynamic" cargo build --release

Check that the two files have a different sha256sum:

    $ sha256sum target/release/min-sized-zenoh*
    cd65933cbc249728ca1abf11034b431d516c39adf4fc744e0c39fe1070ef9479  target/release/min-sized-zenoh
    56965a31b0358fe3c330f6db876ee0de38a52e301bab68a3438ed48949609ab1  target/release/min-sized-zenoh.orig

Create a bsdiff patch:

    $ bsdiff
    bsdiff: usage: bsdiff oldfile newfile patchfile
    $ bsdiff target/release/min-sized-zenoh.orig target/release/min-sized-zenoh patch
    $ ls -l patch
    -rw-r--r--. 1 jdiez jdiez 843 Mar 31 11:09 patch

The patch is 843 bytes.
Now apply it to the original binary and check that it has the same `sha256sum`: 

    $ bspatch
    bspatch: usage: bspatch oldfile newfile patchfile
    $ bspatch target/release/min-sized-zenoh.orig min-sized-zenoh.new patch
    $ sha256sum min-sized-zenoh.new target/release/min-sized-zenoh
    cd65933cbc249728ca1abf11034b431d516c39adf4fc744e0c39fe1070ef9479  min-sized-zenoh.new
    cd65933cbc249728ca1abf11034b431d516c39adf4fc744e0c39fe1070ef9479  target/release/min-sized-zenoh

Feierabend! 🎉 
