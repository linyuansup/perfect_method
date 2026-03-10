struct Rosenbrock;

impl Rosenbrock {
    // 计算函数值 f(x)
    fn value(x: &[f64; 2]) -> f64 {
        100.0 * (x[1] - x[0].powi(2)).powi(2) + (1.0 - x[0]).powi(2)
    }

    // 计算梯度 g(x)
    fn gradient(x: &[f64; 2]) -> [f64; 2] {
        let df_dx1 = -400.0 * x[0] * (x[1] - x[0].powi(2)) - 2.0 * (1.0 - x[0]);
        let df_dx2 = 200.0 * (x[1] - x[0].powi(2));
        [df_dx1, df_dx2]
    }
}

/// 不精确一维搜索：Armijo 准则
/// xk: 当前点, pk: 搜索方向, rho: 减少系数 (0, 0.5), sigma: 步长缩减因子 (0, 1)
fn inexact_line_search(xk: &[f64; 2], pk: &[f64; 2], mut alpha: f64) -> f64 {
    let rho = 0.1; // 常用参数
    let sigma = 0.5; // 每次迭代将步长减半

    let f_xk = Rosenbrock::value(xk);
    let gk = Rosenbrock::gradient(xk);

    // 计算方向导数: grad(f)^T * p
    let g_dot_p = gk[0] * pk[0] + gk[1] * pk[1];

    // 循环直到满足 Armijo 条件: f(xk + alpha*pk) <= f(xk) + rho * alpha * (gk^T * pk)
    loop {
        let x_next = [xk[0] + alpha * pk[0], xk[1] + alpha * pk[1]];
        let f_next = Rosenbrock::value(&x_next);

        if f_next <= f_xk + rho * alpha * g_dot_p {
            break;
        }
        alpha *= sigma;

        if alpha < 1e-10 {
            break;
        } // 防止死循环
    }
    alpha
}

pub fn p1_8() {
    let xk = [-1.0, 1.0];
    let pk = [1.0, 1.0];
    let initial_alpha = 1.0;

    let alpha = inexact_line_search(&xk, &pk, initial_alpha);
    let final_x = [xk[0] + alpha * pk[0], xk[1] + alpha * pk[1]];
    let final_val = Rosenbrock::value(&final_x);

    println!("Start Point xk: {:?}", xk);
    println!("Search Direction pk: {:?}", pk);
    println!("Found Step Size alpha: {:.6}", alpha);
    println!("Updated Point x_{{k+1}}: {:?}", final_x);
    println!("Function Value f(x_{{k+1}}): {:.6}", final_val);
}
