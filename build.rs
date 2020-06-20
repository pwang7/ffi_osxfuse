// cc
//  -Iexample/50d858e@@hello_ll@exe
//  -Iexample
//  -I../example
//  -Iinclude
//  -I../include
//  -Ilib
//  -I../lib
//  -I.
//  -I../
//  -Wall
//  -Wextra
//  -Winvalid-pch
//  -Wmissing-declarations
//  -Wno-sign-compare
//  -Wno-unused-result
//  -Wstrict-prototypes
//  -Wwrite-strings
//  -D_FILE_OFFSET_BITS=64
//  -D_REENTRANT
//  -DHAVE_CONFIG_H
//  -O2
//  -fdiagnostics-color=always
//  -fno-strict-aliasing
//  -pipe
//  -pthread
//  -g
//  -MD
//  -MQ 'example/50d858e@@hello_ll@exe/hello_ll.c.o'
//  -MF 'example/50d858e@@hello_ll@exe/hello_ll.c.o.d'
//  -o 'example/50d858e@@hello_ll@exe/hello_ll.c.o'
//  -c ../example/hello_ll.c
extern crate cc;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(target_os = "macos")]
fn list_dir(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.to_str().unwrap().ends_with(".c")
                && !p.to_str().unwrap().contains("mount.c")
                && !p.to_str().unwrap().contains("mount_bsd.c")
        })
        .collect::<Vec<PathBuf>>()
}

fn main() {
    cc::Build::new()
        .no_default_flags(true)
        .cpp(false)
        .include("src/osxfuse/common") // for fuse_param.h etc
        .include("src/osxfuse/example")
        .include("src/osxfuse/include")
        .include("src/osxfuse/lib")
        .define("FUSE_USE_VERSION", "26") // fuse version
        // .define("FUSERMOUNT_DIR", "\"/bin\"") // fusermount folder
        // .define("FUSERMOUNT_PROG", "\"fusermount\"") // override fusermount3
        .define("_FILE_OFFSET_BITS", "64")
        .define("_REENTRANT", None)
        .define("HAVE_CONFIG_H", None)
        // .define("__APPLE__", None)
        .define("__NetBSD__", None) // avoid include mntent.h
        .define("SPECNAMELEN", "255") // max length of devicename
        .opt_level(0)
        .flag("-fdiagnostics-color=auto")
        // .flag("-fno-strict-aliasing")
        .flag("-pipe") // avoid temporary files, speeding up builds
        .flag("-pthread")
        // .flag("-c") // just compile
        .flag("-v") // verbose
        .flag("-g") // debug version binary
        // .flag("-MD")
        // .flag("-static")
        // .flag("-X c")
        // .flag("-std=c99")
        // .flag("-Winvalid-pch")
        // .flag("-Wmissing-declarations")
        // .flag("-Wno-sign-compare")
        // .flag("-Wno-unused-result")
        // .flag("-Wstrict-prototypes")
        // .flag("-Wwrite-strings")
        // .flag("-Wl,-framework,CoreFoundation") // mac framework
        // .flag("-Wl,-framework,CoreServices") // mac framework
        // .flag("-Wl,-framework,DiskArbitration") // mac framework
        // .flag("-Wl,-framework,IOKit") // mac framework
        // .flag("-Xlinker -liconv") // for iconv
        .warnings(true)
        .extra_warnings(true)
        .file("src/osxfuse/example/my_hello_ll.c")
        // .file("src/osxfuse/example/my_passthrough_ll.c")
        .files(list_dir("src/osxfuse/lib"))
        .files(list_dir("src/osxfuse/lib/modules"))
        .compile("fuse");
}
