#[allow(unused_macros)]

macro_rules! add {
    ($($a:expr), *) => {
        0
        $(+$a)*
    }
}

#[cfg(test)]
mod test_macros {
    #[test]
    fn variable_args_add() {
        let result = add![1, 9];
        assert_eq![result, 10];

        let result = add!(1);
        assert_eq![result, 1];

        let result = add!(1, 2, 3, 5);
        assert_eq![result, 11];
    }
}
