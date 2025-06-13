use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    
    // Ввод размера матрицы (чётного числа)
    let n = loop {
        println!("Введите размер матрицы: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка чтения");
        let n: usize = match input.trim().parse() {
            Ok(num) if num % 2 == 0 => num,
            _ => {
                println!("Размер должен быть чётным числом!");
                continue;
            }
        };
        break n;
    };
    
    // Инициализация матрицы случайными числами [10, 30]
    let mut matrix = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = rng.gen_range(10..=30);
        }
    }
    
    // Вывод исходной матрицы
    println!("Исходная матрица:");
    for row in &matrix {
        for num in row {
            print!("{}\t", num);
        }
        println!();
    }
    
    let quarter = n / 2;
    
    // Обработка левой нижней четверти (заполняем нулями)
    for i in quarter..n {
        for j in 0..quarter {
            matrix[i][j] = 0;
        }
    }
    
    // Обработка правой нижней четверти (заполняем десятками)
    for i in quarter..n {
        for j in quarter..n {
            matrix[i][j] = 10;
        }
    }
    
    // Обмен верхних четвертей местами
    for i in 0..quarter {
        for j in 0..quarter {
            matrix[i].swap(j, j + quarter);
        }
    }
    
    // Вывод изменённой матрицы
    println!("\nИзмененная матрица:");
    for row in &matrix {
        for num in row {
            print!("{}\t", num);
        }
        println!();
    }
}
