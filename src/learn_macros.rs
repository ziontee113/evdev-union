#[allow(unused_macros)]

macro_rules! add {
    ($($a:expr), *) => {
        0
        $(+$a)*
    }
}

macro_rules! my_macro {
    ($a:expr) => {{
        let x = $a;
        let mut split = x.split('|');
        split.next().unwrap().to_owned()
    }};
}

#[cfg(test)]
mod test_macros {
    #[test]
    fn pp_macro() {
        assert_eq!("hi", my_macro!("hi|hello"));
    }

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
