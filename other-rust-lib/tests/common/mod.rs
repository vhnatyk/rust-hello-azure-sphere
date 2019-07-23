
/*
	Rust hello from C for Azure Sphere

	Copyright 2019 by Vitaliy Hnatyk @ Eruptiq

	@license CC BY 4.0+ <https://creativecommons.org/licenses/by/4.0/legalcode.txt>
*/

pub fn keygen(
    t: usize,
    n: usize,
) -> u8 {
    //stub
    10
}

#[allow(dead_code)]
pub fn sign(t: usize, n: usize) {
    //stub
}

#[cfg(not(test))]
fn main() {
    let t = 1;
    let n = 2;

    println!("running keygen (t={},n={})", t, n);
    {
        let start = std::time::Instant::now();
        keygen(t, n);
        println!("keygen elapsed {:?}", start.elapsed()); // note :?

        let start = std::time::Instant::now();
        sign(t, n);
        println!("sign elapsed {:?}", start.elapsed()); // note :?
    }
}
