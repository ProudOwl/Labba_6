use std::{thread, time};
use rand::Rng;

const WIDTH: usize = 60;
const HEIGHT: usize = 30;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_grid(grid: &[[bool; WIDTH]; HEIGHT]) {
    for row in grid {
        for &cell in row {
            print!("{}", if cell { "0" } else { " " });
        }
        println!();
    }
}

fn count_neighbors(grid: &[[bool; WIDTH]; HEIGHT], x: usize, y: usize) -> usize {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let nx = (x as isize + i + HEIGHT as isize) as usize % HEIGHT;
            let ny = (y as isize + j + WIDTH as isize) as usize % WIDTH;
            if grid[nx][ny] {
                count += 1;
            }
        }
    }
    count
}

fn next_generation(grid: &[[bool; WIDTH]; HEIGHT]) -> [[bool; WIDTH]; HEIGHT] {
    let mut new_grid = [[false; WIDTH]; HEIGHT];
    
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let neighbors = count_neighbors(grid, i, j);
            
            new_grid[i][j] = if grid[i][j] {
                neighbors == 2 || neighbors == 3
            } else {
                neighbors == 3
            };
        }
    }
    
    new_grid
}

fn setup_random(grid: &mut [[bool; WIDTH]; HEIGHT]) {
    let mut rng = rand::thread_rng();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            grid[i][j] = rng.gen_ratio(1, 4);
        }
    }
}

fn setup_gosper_glider_gun(grid: &mut [[bool; WIDTH]; HEIGHT]) {
    let gun_coords = [
        (5, 1), (5, 2), (6, 1), (6, 2),
        (5, 11), (6, 11), (7, 11),
        (4, 12), (8, 12),
        (3, 13), (9, 13),
        (3, 14), (9, 14),
        (6, 15),
        (4, 16), (8, 16),
        (5, 17), (6, 17), (7, 17),
        (6, 18),
        (3, 21), (4, 21), (5, 21),
        (3, 22), (4, 22), (5, 22),
        (2, 23), (6, 23),
        (1, 25), (2, 25), (6, 25), (7, 25),
        (3, 35), (4, 35), (3, 36), (4, 36)
    ];
    
    for &(x, y) in &gun_coords {
        if x < HEIGHT && y < WIDTH {
            grid[x][y] = true;
        }
    }
}

fn main() {
    let mut grid = [[false; WIDTH]; HEIGHT];
    
    println!("1. Случайная генерация");
    println!("2. Ружьё Госпера (Gosper Glider Gun)");
    println!("Номер режима (1 или 2): ");
    
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Ошибка чтения ввода");
    let choice: u32 = choice.trim().parse().unwrap_or(1);
    
    match choice {
        1 => setup_random(&mut grid),
        2 => setup_gosper_glider_gun(&mut grid),
        _ => setup_random(&mut grid),
    }
    
    let mut generation = 0;
    loop {
        clear_screen();
        print_grid(&grid);
        grid = next_generation(&grid);
        thread::sleep(time::Duration::from_millis(150));
    }
}
