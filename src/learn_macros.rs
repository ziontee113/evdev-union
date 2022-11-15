#![allow(unused_macros)]

macro_rules! add {
    ($($a:expr),+ => $b:expr) => {
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

macro_rules! munch{
 // first arm in case of single argument and last remaining variable/number
    ($a:expr) => {
        $a
    };
// second arm in case of two arument are passed and stop recursion in case of odd number ofarguments
    ($a:expr,$b:expr) => {
        {
            $a+$b
        }
    };
// add the number and the result of remaining arguments
    ($a:expr,$($b:tt)*) => {
       {
           $a+munch!($($b)*)
       }
    }
}

#[cfg(test)]
mod test_macros {
    #[test]
    fn pp_macro() {
        assert_eq!("hi", my_macro!("hi|hello"));
    }

    #[test]
    fn tt_muncher_macro() {
        assert_eq![munch!(1, 2, 3), 6];
    }

    #[test]
    fn variable_args_add() {
        let result = add![1, 9 => 10];
        assert_eq![result, 0];
    }
}
