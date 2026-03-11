fn rosenbrock(x1: f64, x2: f64) -> f64 {
    100.0 * (x2 - x1.powi(2)).powi(2) + (1.0 - x1).powi(2)
}

fn phi(alpha: f64) -> f64 {
    let x1 = -1.0 + alpha;
    let x2 = 1.0 + alpha;
    rosenbrock(x1, x2)
}

fn phi_prime(alpha: f64) -> f64 {
    400.0 * alpha.powi(3) - 1800.0 * alpha.powi(2) + 1802.0 * alpha - 4.0
}

fn bisect_root(mut left: f64, mut right: f64) -> f64 {
    let mut f_left = phi_prime(left);

    for _ in 0..100 {
        let mid = (left + right) / 2.0;
        let f_mid = phi_prime(mid);

        if f_mid.abs() < 1e-14 || (right - left).abs() < 1e-12 {
            return mid;
        }

        if f_left * f_mid <= 0.0 {
            right = mid;
        } else {
            left = mid;
            f_left = f_mid;
        }
    }

    (left + right) / 2.0
}

fn stationary_points() -> Vec<f64> {
    let mut roots = Vec::new();
    let step = 0.001;
    let mut left = 0.0;
    let mut f_left = phi_prime(left);

    let mut right = left + step;
    while right <= 4.0 {
        let f_right = phi_prime(right);

        if f_left.abs() < 1e-12 {
            roots.push(left);
        } else if f_left * f_right < 0.0 {
            roots.push(bisect_root(left, right));
        }

        left = right;
        f_left = f_right;
        right += step;
    }

    roots.dedup_by(|a, b| (*a - *b).abs() < 1e-8);
    roots
}

fn armijo_backtracking(alpha0: f64, rho: f64, c1: f64) -> (f64, usize) {
    let xk = [-1.0, 1.0];
    let pk = [1.0, 1.0];
    let grad_fk = [-4.0, 0.0];
    let directional_derivative = grad_fk[0] * pk[0] + grad_fk[1] * pk[1];
    let fk = rosenbrock(xk[0], xk[1]);

    let mut alpha = alpha0;
    let mut iter = 0;

    println!(
        "{:<5} | {:<12} | {:<14} | {:<14} | {:<8}",
        "Iter", "alpha", "phi(alpha)", "Armijo RHS", "Accept"
    );
    println!("{:-<68}", "");

    loop {
        iter += 1;
        let candidate = fk + c1 * alpha * directional_derivative;
        let value = phi(alpha);
        let accept = value <= candidate;

        println!(
            "{:<5} | {:<12.6} | {:<14.6} | {:<14.6} | {:<8}",
            iter,
            alpha,
            value,
            candidate,
            if accept { "yes" } else { "no" }
        );

        if accept {
            break;
        }

        alpha *= rho;
    }

    (alpha, iter)
}

pub fn p1_9() {
    println!("\np1.9 Rosenbrock Inexact Line Search");
    println!("x_k = (-1, 1)^T, p_k = (1, 1)^T");
    println!("phi(alpha) = 100 alpha^4 - 600 alpha^3 + 901 alpha^2 - 4 alpha + 4");
    println!("phi'(alpha) = 400 alpha^3 - 1800 alpha^2 + 1802 alpha - 4");
    println!("grad f(x_k) = (-4, 0)^T, grad f(x_k)^T p_k = -4 < 0");

    let roots = stationary_points();
    let exact_best = roots
        .iter()
        .copied()
        .min_by(|a, b| phi(*a).partial_cmp(&phi(*b)).unwrap())
        .unwrap();

    println!("\nStationary points on alpha >= 0:");
    for alpha in &roots {
        println!("alpha ≈ {:.6}, phi(alpha) ≈ {:.6}", alpha, phi(*alpha));
    }
    println!(
        "Best exact line-search step on this ray: alpha ≈ {:.6}, phi(alpha) ≈ {:.6}",
        exact_best,
        phi(exact_best)
    );

    println!("\nArmijo backtracking (alpha0 = 1, rho = 0.5, c1 = 1e-4):");
    let (alpha, iterations) = armijo_backtracking(1.0, 0.5, 1e-4);

    println!("{:-<68}", "");
    println!(
        "Accepted inexact step: alpha ≈ {:.6} after {} iterations",
        alpha, iterations
    );
    println!(
        "New point: x_k + alpha p_k ≈ ({:.6}, {:.6})^T",
        -1.0 + alpha,
        1.0 + alpha
    );
    println!("f(x_k + alpha p_k) = phi(alpha) ≈ {:.6}", phi(alpha));
}
