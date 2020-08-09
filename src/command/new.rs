use crate::helpers::create_new_file;

pub fn new() -> Result<(), Box<dyn std::error::Error>> {
    let new = create_new_file("# ")?;
    println!("{}", new);
    Ok(())
}
