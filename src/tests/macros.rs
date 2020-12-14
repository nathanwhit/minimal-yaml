#![cfg(test)]

#[allow(unused)]
macro_rules! custom_test {
    (
        #[test(timeout = $timeout:expr)]
        $( #[$meta:meta] )*
        fn $fname:ident $($rest:tt)*
    ) => (
        #[test]
        $( #[$meta] )*
        fn $fname ()
        {
            let (done_tx, done_rx) = ::std::sync::mpsc::channel();
            let handle =
                ::std::thread::Builder::new()
                    .name(
                        concat!(module_path!(), "::", stringify!($fname))
                            .splitn(2, "::").nth(1).unwrap()
                            .into()
                    )
                    .spawn(move || {
                        {
                            fn $fname $($rest)*
                            $fname();
                        }
                        let _ = done_tx.send(());
                    })
                    .unwrap()
            ;

            match done_rx.recv_timeout({
                use ::std::time::*;
                $timeout
            }) {
                | Err(::std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    panic!("Test took too long");
                },
                | _ => if let Err(err) = handle.join() {
                    ::std::panic::resume_unwind(err);
                },
            }
        }
    );

    (
        $($tt:tt)*
    ) => (
        $($tt)*
    );
}

macro_rules! mk_test {
    ($($name: ident) +; $inp: expr => fail) => {
        paste::item! {
            #[test]
            fn [<test_parse_$($name _)+>] () {
                let input: &str = $inp;
                crate::parse(input).unwrap_err();
            }
        }
    };
    (timeout = $e:expr; $($name: ident) +; $inp: expr => $exp: expr) => {
        paste::item! {
            custom_test! {
                #[test(timeout = $e)]
                fn [<test_parse_$($name _)+>] () {
                    let input: &str = $inp;
                    assert_eq!(parse(input).unwrap(), $exp.into());
                }
            }
        }
    };
    ($($name: ident) +; $inp: expr => $exp: expr) => {
        paste::item! {
            #[test]
            fn [<test_parse_$($name _)+>] () {
                let input: &str = $inp;
                assert_eq!(crate::parse(input).unwrap(), $exp.into());
            }
        }
    };
    ($($name: ident) +; $inp: expr => err $exp: expr) => {
        paste::item! {
            #[test]
            fn [<test_parse_$($name _)+>] () {
                let input: &str = $inp;
                assert_eq!(crate::parse(input).unwrap_err(), $exp.into());
            }
        }
    };
    ($($name: ident) +; $inp: stmt => matches $exp: expr) => {
        paste::item! {
            #[test]
            fn [<test_parse_$($name _)+>] () {
                let input: &str = $inp;
                assert!(
                    match crate::parse(input) {
                        $exp => true,
                        _ => false,
                    }
                );
            }
        }
    };
    ($($name: ident) +; $inp: expr => err msg $exp: expr) => {
        paste::item! {
            #[test]
            fn [<test_parse_$($name _)+>] () {
                let input: &str = $inp;
                assert_eq!(crate::parse(input).unwrap_err().to_string(), $exp);
            }
        }
    };
}

macro_rules! seq {
    ($($val: expr),*) => {
        crate::Yaml::Sequence(vec![$( $val.into() ),*])
    }
}

macro_rules! map {
    { $($key : tt : $val : tt),* } => {
        $crate::Yaml::Mapping(vec![$($crate::Entry { key: $key.into() , value: $val.into() }),*])
    };
    { $($key : expr => $val : expr);* } => {
        $crate::Yaml::Mapping(vec![$($crate::Entry { key: $key.into() , value: $val.into() }),*])
    }
}
