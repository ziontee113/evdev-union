#![allow(unused_macros)]

macro_rules! add {
    ($($a:expr), * => $b:expr) => {
        0
        $(+$a)*
        -$b
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
        let result = add![1, 9 => 10];
        assert_eq![result, 0];
    }
}
