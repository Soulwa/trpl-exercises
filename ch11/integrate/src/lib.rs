pub fn add_two(a: i32) -> i32 {
	internal_adder(a, 2)
}

fn internal_adder(lhs: i32, rhs: i32) -> i32 {
	lhs + rhs
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
