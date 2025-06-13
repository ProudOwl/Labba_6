use rand::Rng;
use std::cmp::min;
use std::i32::MAX;

fn generate_random_board(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();
    (0..rows)
        .map(|_| (0..cols).map(|_| rng.gen_range(0..2)).collect())
        .collect()
}

fn print_board(board: &[Vec<i32>]) {
    for row in board {
        for coin in row {
            print!("{} ", coin);
        }
        println!();
    }
}

fn count_mismatches(board: &[Vec<i32>], start_with: bool) -> i32 {
    let mut mismatches = 0;
    let mut expected = start_with;
    
    for row in board {
        let mut current_expected = expected;
        for &coin in row {
            if (coin == 1) != current_expected {
                mismatches += 1;
            }
            current_expected = !current_expected;
        }
        expected = !expected;
    }
    mismatches
}

fn solve_coin_board(board: &[Vec<i32>]) -> i32 {
    let mismatches1 = count_mismatches(board, true);
    let mismatches2 = count_mismatches(board, false);
    
    if mismatches1 % 2 != 0 && mismatches2 % 2 != 0 {
        return -1;
    }
    
    let min_moves1 = if mismatches1 % 2 == 0 {
        mismatches1 / 2
    } else {
        MAX
    };
    
    let min_moves2 = if mismatches2 % 2 == 0 {
        mismatches2 / 2
    } else {
        MAX
    };
    
    if min_moves1 == MAX && min_moves2 == MAX {
        -1
    } else {
        min(min_moves1, min_moves2)
    }
}

fn main() {
    println!("Введите количество строк и столбцов доски: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");
    
    let dimensions: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    
    if dimensions.len() != 2 || dimensions[0] == 0 || dimensions[1] == 0 {
        println!("Некорректные размеры доски!");
        return;
    }
    
    let (rows, cols) = (dimensions[0], dimensions[1]);
    let board = generate_random_board(rows, cols);
    
    println!("\nСгенерированная доска:");
    print_board(&board);
    
    let min_moves = solve_coin_board(&board);
    
    if min_moves == -1 {
        println!("\nНевозможно преобразовать доску в шахматный порядок");
    } else {
        println!("\nМинимальное количество ходов: {}", min_moves);
    }
}
