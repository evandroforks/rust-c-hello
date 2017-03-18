extern crate gcc;

fn main()
{
    gcc::Config::new()
            .file("hello.c")
            .include("src")
            .compile("libhello.a");
}
