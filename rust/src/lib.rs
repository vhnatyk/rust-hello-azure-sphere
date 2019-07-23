/*
	Rust hello from C for Azure Sphere
	Copyright 2019 by Vitaliy Hnatyk @ Eruptiq

	@license CC BY 4.0+ <https://creativecommons.org/licenses/by/4.0/legalcode.txt>
*/

#[macro_use]
mod woo;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
