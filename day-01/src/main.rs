use std::{fs, string::String};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let contents = String::from_utf8(fs::read("input.txt")?)?;
    let mut contents: Vec<u32> = contents.trim().split("\n\n")
                                        .map(|elf| elf.split("\n")
                                                .map(|n| n.parse::<u32>().unwrap_or(0))
                                                .reduce(|acc, n| acc + n)
                                                .unwrap_or(0)
                                        )
                                        .collect();
    contents.sort();
    let first_part = contents.last();
    contents.reverse();
    let second_part = contents.into_iter().take(3).reduce(|acc, n| acc + n);
    println!("{:?}", second_part);

    Ok(())
}
