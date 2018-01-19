#[macro_use] extern crate dbg;

fn main() {
    #[derive(Debug)] // The type of expr in dbg!(expr) must be Debug.
    struct Point {
        x: usize,
        y: usize,
    }

    macro_rules! test {
        ($test: ident $block: block) => {{
            eprintln!("{0:-^80}\nRunning example: {1}\n{0:-^80}\n", "",
                stringify!($test));
            $block
        }};
    }

    test!(unit_works {
        dbg!();
    });

    // Will not work on nightly:
    test!(not_debug {        
        struct X(usize);
        let a = X(1);
        dbg!(&a);
    });

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
            if dbg!(n, n <= 1).1 {
                dbg!(n, 1).1
            } else {
                dbg!(n, n * factorial(n - 1)).1
            }
        }

        dbg!(factorial(4));
    });
}