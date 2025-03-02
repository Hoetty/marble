use std::io::{stdin, stdout, Write};

/// Creates an iterator, that first prints " > " to stdout an then gets an input
pub fn input() -> impl Iterator<Item = String> {
    [0u8]
        .iter()
        .cycle()
        .inspect(|_| {
            print!(" > ");
            let _ = stdout().flush();
        })
        .zip(stdin().lines().map_while(|l| l.ok()))
        .map(|(_, s)| s)
}
