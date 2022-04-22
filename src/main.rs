fn main() -> Result<(), String> {
    let mut ratlab = ratlab::Ratlab::new();

    println!("{}", ratlab.input("x = -10:10")?);
    println!("{}", ratlab.input("y = [x x]")?);

    Ok(())
}
