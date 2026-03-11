pub fn p1_6() {
    let f = |x: f64| (-x).exp() + x.powi(2);

    let mut a: f64 = -1.0;
    let mut b: f64 = 1.0;
    let epsilon = 0.1;
    let phi = (5.0_f64.sqrt() - 1.0) / 2.0;

    println!(
        "{:<5} | {:<10} | {:<10} | {:<10} | {:<10}",
        "Iter", "a", "b", "L", "x_mid"
    );
    println!("{:-<60}", "");

    let mut i = 0;
    while (b - a).abs() > epsilon {
        i += 1;

        let x1 = b - phi * (b - a);
        let x2 = a + phi * (b - a);

        let f1 = f(x1);
        let f2 = f(x2);

        println!(
            "{: <5} | {:<10.4} | {:<10.4} | {:<10.4} | {:<10.4}",
            i,
            a,
            b,
            b - a,
            (a + b) / 2.0
        );

        if f1 < f2 {
            b = x2;
        } else {
            a = x1;
        }
    }

    let x_min = (a + b) / 2.0;
    println!("{:-<60}", "");
    println!("x ≈ {:.4}, f(x) ≈ {:.4}", x_min, f(x_min));
}
