use std::io;
mod arrays_hashing;
use crate::arrays_hashing::arrays_hashing_tests;

fn main() -> io::Result<()>
{
    arrays_hashing_tests();

    Ok(())
}
