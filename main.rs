
fn main()
{
    #[link(name="hello", kind="static")]
    extern { fn hello(); }
    unsafe { hello(); };
}
