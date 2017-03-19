# rust-c-hello


Simple example of running `C` code from `Rust` code, based on the examples at:

1. https://github.com/alexcrichton/gcc-rs
1. https://users.rust-lang.org/t/linking-with-custom-c-library/637/4
1. http://doc.crates.io/build-script.html


### Building

To build it, just type `cargo build`. To build and run, just type `cargo run`.

For Windows users', you need to run it from a command line setup with the MSVS compiler
environment variables. You can create a batch file named `vs.bat` like this:
```batch


@echo off

:: Path to your Visual Studio folder.
::
:: Examples:
::     C:\Program Files\Microsoft Visual Studio 9.0
::     F:\VisualStudio2015
set VISUAL_STUDIO_FOLDER=F:\VisualStudio2015

:: Load compilation environment
call "%VISUAL_STUDIO_FOLDER%\VC\vcvarsall.bat" x64

:: Invoke compiler with any options passed to this batch file
%*
```

And put it on your path variable. Therefore just call `vs.bat cargo run` and it would
set up the MSVS environment variable and call the command `cargo run` to build the project.

This also allow you to call `vs.bat cargo run` from other shells environments like `Cygwin's Mintty`
command line.
