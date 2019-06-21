use std::fs;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    for ch in b'A'..=b'Z' {
        let ch = char::from(ch);
        let file = format!("tmp/example-{}.desc", ch);
        dbg!(&file);
        let mut file = fs::File::create(file)?;
        let desc = format!(
            "(0,0),(10,0),(10,10),(0,10)#(0,0)#(4,2),(6,2),(6,7),(4,7);(5,8),(6,8),(6,9),(5,9)#B(0,1);B(1,1);F(0,2);F(1,2);L(0,3);{}(0,4);X(0,9)",
            ch).into_bytes();
        file.write_all(&desc)?;
    }
    Ok(())
}
