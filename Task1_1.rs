use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    
    println!("Введите размер матрицы N: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения");
    let n: usize = input.trim().parse().expect("Ожидается число");
    
    // Инициализация матрицы
    let mut matrix = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = rng.gen_range(-10..=40);
        }
    }
    
    // Вывод матрицы
    println!("Исходная матрица:");
    for row in &matrix {
        for num in row {
            print!("{}\t", num);
        }
        println!();
    }
    
    // Поиск строк с 3+ отрицательными элементами
    let mut negative_rows = Vec::new();
    let mut negative_elements = Vec::new();
    let mut new_matrix = Vec::new();
    
    for (i, row) in matrix.iter().enumerate() {
        let mut count = 0;
        let mut current_negatives = Vec::new();
        
        for num in row {
            if *num < 0 {
                count += 1;
                current_negatives.push(*num);
            }
        }
        
        if count >= 3 {
            negative_rows.push(i);
            negative_elements.extend(current_negatives);
            new_matrix.push(row.clone());
        }
    }
    
    // Вывод результатов
    print!("\nНомера строк с 3+ отрицательными элементами: ");
    for row in &negative_rows {
        print!("{} ", row);
    }
    
    print!("\nОтрицательные элементы из этих строк: ");
    for num in &negative_elements {
        print!("{} ", num);
    }
    
    println!("\n\nНовая матрица из найденных строк:");
    for row in &new_matrix {
        for num in row {
            print!("{}\t", num);
        }
        println!();
    }
}
