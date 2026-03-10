fn f(x: f64) -> f64 {
    x.powi(4) + 2.0 * x + 4.0
}

/// 1. Fibonacci 搜索法
fn fibonacci_search(mut a: f64, mut b: f64, epsilon: f64) -> f64 {
    let target = (b - a) / epsilon;

    let mut fib = vec![1.0, 1.0];
    while *fib.last().unwrap() < target {
        let next = fib[fib.len() - 1] + fib[fib.len() - 2];
        fib.push(next);
    }

    let n = fib.len() - 1; // 迭代总次数

    let mut x1 = a + (fib[n - 2] / fib[n]) * (b - a);
    let mut x2 = a + (fib[n - 1] / fib[n]) * (b - a);
    let mut f1 = f(x1);
    let mut f2 = f(x2);

    for k in 1..(n - 1) {
        if f1 < f2 {
            b = x2;
            x2 = x1;
            f2 = f1;
            x1 = a + (fib[n - k - 2] / fib[n - k]) * (b - a);
            f1 = f(x1);
        } else {
            a = x1;
            x1 = x2;
            f1 = f2;
            x2 = a + (fib[n - k - 1] / fib[n - k]) * (b - a);
            f2 = f(x2);
        }
    }

    (a + b) / 2.0
}

fn quadratic_interpolation(mut a: f64, mut c: f64, epsilon: f64) -> f64 {
    let mut b = (a + c) / 2.0; // 取中点作为初始三点之一
    let mut x_old = f64::MAX;

    loop {
        let fa = f(a);
        let fb = f(b);
        let fc = f(c);

        // 二次插值公式计算极值点 x_new
        let num = (b - a).powi(2) * (fb - fc) - (b - c).powi(2) * (fb - fa);
        let den = (b - a) * (fb - fc) - (b - c) * (fb - fa);

        let x_new = b - 0.5 * (num / den);

        if (x_new - x_old).abs() < epsilon {
            return x_new;
        }

        x_old = x_new;

        // 更新区间点 (保持 a < b < c 且 x_new 在其中)
        if x_new > b {
            if f(x_new) > fb {
                c = x_new;
            } else {
                a = b;
                b = x_new;
            }
        } else {
            if f(x_new) > fb {
                a = x_new;
            } else {
                c = b;
                b = x_new;
            }
        }
    }
}

pub fn p1_7() {
    let a = -1.0;
    let b = 0.0;
    let epsilon = 0.01;

    let res_fib = fibonacci_search(a, b, epsilon);
    let res_quad = quadratic_interpolation(a, b, epsilon);

    println!("Fibonacci result: x ≈ {:.4}", res_fib);
    println!("Quadratic result: x ≈ {:.4}", res_quad);
}
