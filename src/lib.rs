//! This crate provides a working implementation of [RFC 2173] on stable Rust.
//! For documentation, please see the [guide-level explanation].
//!
//! [guide-level explanation]: https://github.com/Centril/rfcs/blob/rfc/quick-debug-macro/text/0000-quick-debug-macro.md#guide-level-explanation
//! [RFC 2173]: https://github.com/rust-lang/rfcs/pull/2173

#![cfg_attr(use_nightly, feature(core_intrinsics, specialization))]

/// See module level documentation.
#[macro_export]
macro_rules! dbg {
    // Handle trailing comma:
    ($($val: expr),+,) => {
        dbg!( $($val),+ )
    };
    ($($lab: expr => $val: expr),+,) => {
        dbg!( $($lab => $val),+ )
    };
    // Without label, use source of $val:
    ($valf: expr $(, $val: expr)*) => {{
        // in order: for panics, clarification on: dbg!(expr);, dbg!(expr)
        #[allow(unreachable_code, unused_must_use, unused_parens)]
        let _r = {
        #[cfg(not(debug_assertions))] { ($valf $(, $val)*) }
        #[cfg(debug_assertions)] {
            // DEBUG: Lock STDERR in a buffered writer.
            // Motivation:
            // 1. to avoid needless re-locking of STDERR at every write(ln)!.
            // 2. to ensure that the printed message is not interleaved, which
            // would disturb the readability of the output, by other messages to
            // STDERR.
            use ::std::io::Write;
            let stderr = ::std::io::stderr();
            let mut err = ::std::io::BufWriter::new(stderr.lock());

            // Deal with specialization:
            // On nightly: typeof(expr) doesn't need to be Debug.
            struct WrapDebug<T>(T);
            use std::fmt::{Debug, Formatter, Result};

            impl<T: Debug> Debug for WrapDebug<T> {
                fn fmt(&self, f: &mut Formatter) -> Result { self.0.fmt(f) }
            }

            #[cfg(use_nightly)]
            impl<T> Debug for WrapDebug<T> {
                default fn fmt(&self, f: &mut Formatter) -> Result {
                    use ::std::intrinsics::type_name;
                    write!(f, "[<unknown> of type {} is !Debug]",
                        unsafe { type_name::<T>() })
                }
            }

            // Are we in not in compact mode (detailed)?
            // If so:
            // + {:?} is used instead of {:#?},
            // + Header is: [<location>]
            let detailed = option_env!("RUST_DBG_COMPACT")
                            .map_or(true, |s| s == "0");

            (if detailed {
                write!(&mut err, "[DEBUGGING, {}:{}]\n=> ", file!(), line!())
            } else {
                write!(&mut err, "[{}:{}] ", file!(), line!())
            }).unwrap();

            // Foreach label and expression:
            //     1. Evaluate each expression,
            //     2. Print out $lab = value of expression
            let _ret = (
                {
                    // Print out $lab = :
                    write!(&mut err, "{} = ", stringify!($valf)).unwrap();

                    // Evaluate, tmp is value:
                    let _tmp = WrapDebug($valf);
                    // Won't get further if $val panics.

                    // Print out tmp:
                    (if detailed { write!(&mut err, "{:#?}", _tmp) }
                    else         { write!(&mut err, "{:?}" , _tmp) }).unwrap();

                    // Yield tmp:
                    _tmp.0
                }
                $(, {
                    // Comma separator:
                    write!(&mut err, ", ").unwrap();

                    // Print out $lab = :
                    write!(&mut err, "{} = ", stringify!($val)).unwrap();

                    // Evaluate, tmp is value:
                    let _tmp = WrapDebug($val);
                    // Won't get further if $val panics.

                    // Print out tmp:
                    (if detailed { write!(&mut err, "{:#?}", _tmp) }
                     else        { write!(&mut err, "{:?}" , _tmp) }).unwrap();

                    // Yield tmp:
                    _tmp.0
                } )*
            );

            // Newline:
            (if detailed { writeln!(&mut err, "\n") }
             else        { writeln!(&mut err, "")   }).unwrap();

            // Return the expression:
            _ret
        }
        };
        _r
    }};
    // With label:
    ($labf: expr => $valf: expr $(, $lab: expr => $val: expr)*) => {{
        // in order: for panics, clarification on: dbg!(expr);, dbg!(expr)
        #[allow(unreachable_code, unused_must_use, unused_parens)]
        let _r = {
        #[cfg(not(debug_assertions))] { ($valf $(, $val)*) }
        #[cfg(debug_assertions)] {
            // DEBUG: Lock STDERR in a buffered writer.
            // Motivation:
            // 1. to avoid needless re-locking of STDERR at every write(ln)!.
            // 2. to ensure that the printed message is not interleaved, which
            // would disturb the readability of the output, by other messages to
            // STDERR.
            use ::std::io::Write;
            let stderr = ::std::io::stderr();
            let mut err = ::std::io::BufWriter::new(stderr.lock());

            // Deal with specialization:
            // On nightly: typeof(expr) doesn't need to be Debug.
            struct WrapDebug<T>(T);
            use std::fmt::{Debug, Formatter, Result};

            impl<T: Debug> Debug for WrapDebug<T> {
                fn fmt(&self, f: &mut Formatter) -> Result { self.0.fmt(f) }
            }

            #[cfg(use_nightly)]
            impl<T> Debug for WrapDebug<T> {
                default fn fmt(&self, f: &mut Formatter) -> Result {
                    use std::intrinsics::type_name;
                    write!(f, "[<unknown> of type {} is !Debug]",
                        unsafe { type_name::<T>() })
                }
            }

            // Are we in not in compact mode (detailed)?
            // If so:
            // + {:?} is used instead of {:#?},
            // + Header is: [<location>]
            let detailed = option_env!("RUST_DBG_COMPACT")
                            .map_or(true, |s| s == "0");

            (if detailed {
                write!(&mut err, "[DEBUGGING, {}:{}]\n=> ", file!(), line!())
            } else {
                write!(&mut err, "[{}:{}] ", file!(), line!())
            }).unwrap();

            // Foreach label and expression:
            //     1. Evaluate each expression,
            //     2. Print out $lab = value of expression
            let _ret = (
                {
                    // Enforce is_literal_string($lab):
                    let _ = concat!($labf, "");
                    let _ : &'static str = $labf;

                    // Print out $lab = :
                    write!(&mut err, "{} = ", stringify!($labf)).unwrap();

                    // Evaluate, tmp is value:
                    let _tmp = WrapDebug($valf);
                    // Won't get further if $val panics.

                    // Print out tmp:
                    (if detailed { write!(&mut err, "{:#?}", _tmp) }
                     else        { write!(&mut err, "{:?}" , _tmp) }).unwrap();

                    // Yield tmp:
                    _tmp.0
                }
                $(, {
                    // Comma separator:
                    write!(&mut err, ", ").unwrap();

                    // Enforce is_literal_string($lab):
                    let _ = concat!($lab, "");
                    let _ : &'static str = $lab;

                    // Print out $lab = :
                    write!(&mut err, "{} = ", stringify!($lab)).unwrap();

                    // Evaluate, tmp is value:
                    let _tmp = WrapDebug($val);
                    // Won't get further if $val panics.

                    // Print out tmp:
                    (if detailed { write!(&mut err, "{:#?}", _tmp) }
                     else        { write!(&mut err, "{:?}" , _tmp) }).unwrap();

                    // Yield tmp:
                    _tmp.0
                } )*
            );

            // Newline:
            (if detailed { writeln!(&mut err, "\n") }
             else        { writeln!(&mut err, "")   }).unwrap();

            // Return the expression:
            _ret
        }
        };
        _r
    }};
}

#[cfg(test)]
mod tests {
    #[derive(Debug)] // The type of expr in dbg!(expr) must be Debug.
    struct Point {
        x: usize,
        y: usize,
    }

    macro_rules! test {
        ($test: ident $block: block) => {
            #[test]
            fn $test() {
                eprintln!();
                $block
            }
        };
    }

    test!(common_use {
        dbg!(Point { x: 1, y: 2 });

        let p = Point { x: 4, y: 5 };
        dbg!(p);
    });

    test!(passthrough {
        let x = dbg!(1 + 2);
        let y = dbg!(x + 1) + dbg!(3);
        dbg!(y);
    });

    test!(types {
        let a = 1;
        let b = 2;
        let _ : u32 = dbg!(a);
        let _ : (u32, u32) = dbg!(a, b);
        let _ : (u32, u32, u32) = dbg!(a, b, a + b);

        let p = Point { x: 4, y: 5 };
        let q = Point { x: 2, y: 1 };
        let _ : (&Point, &Point) = dbg!(&p, &q);
    });

    test!(labels {        
        let w = 1;
        let h = 2;
        dbg!("width" => w, "height" => h, "area" => w * h);

        let p = Point { x: 4, y: 5 };
        dbg!("first point" => &p, "same point" => &p);
    });

    test!(not_debug {        
        struct X(usize);
        let a = X(1);
        dbg!(&a);
    });

    test!(factorial_simple {
        fn factorial(n: u32) -> u32 {
            if dbg!(n <= 1) {
                dbg!(1)
            } else {
                dbg!(n * factorial(n - 1))
            }
        }

        dbg!(factorial(4));
    });

    test!(factorial_labels {
        fn factorial(n: u32) -> u32 {
            if dbg!("are we at the base case?" => n <= 1) {
                dbg!("base value" => 1)
            } else {
                dbg!("ascending with n * factorial(n - 1)" => n * factorial(n - 1))
            }
        }

        dbg!(factorial(4));
    });

    test!(factorial_multiarg {
        fn factorial(n: u32) -> u32 {
            if dbg!(n, (n <= 1)).1 {
                dbg!(n, 1).1
            } else {
                dbg!(n, n * factorial(n - 1)).1
            }
        }

        dbg!(factorial(4));
    });

    #[should_panic]
    #[test]
    fn panics() {
        eprintln!();
        let (a, b) = (1, 2);
        dbg!(a, panic!(), b);
    }
}