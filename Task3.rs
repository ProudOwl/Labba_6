use std::f64::EPSILON;
use std::cmp::max;

const EPS: f64 = 1e-6;

// Метод Гаусса
fn gaussian_elimination(a: &mut Vec<Vec<f64>>, b: &mut Vec<f64>) -> Vec<f64> {
    let n = a.len();
    
    for i in 0..n {
        // Поиск строки с максимальным элементом в текущем столбце
        let mut max_row = i;
        for k in (i + 1)..n {
            if a[k][i].abs() > a[max_row][i].abs() {
                max_row = k;
            }
        }

        // Обмен строками
        a.swap(i, max_row);
        b.swap(i, max_row);
        
        // Прямой ход
        for k in (i + 1)..n {
            let factor = a[k][i] / a[i][i];
            for j in i..n {
                a[k][j] -= factor * a[i][j];
            }
            b[k] -= factor * b[i];
        }
    }
    
    // Обратный ход
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = b[i];
        for j in (i + 1)..n {
            x[i] -= a[i][j] * x[j];
        }
        x[i] /= a[i][i];
    }
    
    x
}

// Метод Зейделя
fn seidel_method(a: &Vec<Vec<f64>>, b: &Vec<f64>, epsilon: f64, max_iterations: usize) -> Vec<f64> {
    let n = a.len();
    let mut x = vec![0.0; n];
    let mut x_prev = vec![0.0; n];
    let mut iteration = 0;
    let mut error;

    loop {
        x_prev = x.clone();
        for i in 0..n {
            let mut sum = 0.0;
            for j in 0..n {
                if j != i {
                    sum += a[i][j] * if j < i { x[j] } else { x_prev[j] };
                }
            }
            x[i] = (b[i] - sum) / a[i][i];
        }

        error = 0.0;
        for i in 0..n {
            error = error.max((x[i] - x_prev[i]).abs());
        }
        iteration += 1;

        if error <= epsilon || iteration >= max_iterations {
            break;
        }
    }

    if iteration == max_iterations {
        println!("Метод Зейделя не сходится");
    } else {
        println!("Метод Зейделя сошелся за {} итераций", iteration);
    }
    
    x
}

// Вывод результатов
fn print_results(x: &[f64], method_name: &str) {
    println!("\nРезультаты решения методом {}:", method_name);
    println!("+-----+-------------+-------------+-------------+-------------+");
    println!("|  N  |     x1      |     x2      |     x3      |     x4      |");
    println!("+-----+-------------+-------------+-------------+-------------+");
    println!("|  1  | {:11.8} | {:11.8} | {:11.8} | {:11.8} |", x[0], x[1], x[2], x[3]);
    println!("+-----+-------------+-------------+-------------+-------------+\n");
}

fn main() {
    let mut a = vec![
        vec![-18.0,  -0.04,  0.21,  0.91],
        vec![-0.09,  -1.23,  -0.23, 0.25],
        vec![-0.13, -0.23,  0.8,   -0.21],
        vec![-1.04,  -1.31,  0.06,  0.15]
    ];

    let mut b = vec![-1.24, -1.04, 2.56, 0.91];

    // Проверка диагонального преобладания
    let mut is_dominant = true;
    for i in 0..4 {
        let mut sum = 0.0;
        for j in 0..4 {
            if j != i {
                sum += a[i][j].abs();
            }
        }
        if a[i][i].abs() <= sum {
            is_dominant = false;
            println!("Строка {} не удовлетворяет условию диагонального преобладания", i+1);
        }
    }
    
    if !is_dominant {
        println!("Внимание: матрица не имеет диагонального преобладания!");
    }

    // Метод Гаусса
    let mut a_gauss = a.clone();
    let mut b_gauss = b.clone();
    let x_gauss = gaussian_elimination(&mut a_gauss, &mut b_gauss);
    print_results(&x_gauss, "Гаусса");

    // Метод Зейделя
    let x_seidel = seidel_method(&a, &b, EPS, 100);
    print_results(&x_seidel, "Зейделя");
}
