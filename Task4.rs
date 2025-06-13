use std::io;

// Функция для подсчета количества единиц (бананов) в двоичном представлении числа
fn count_bananas(num: u64) -> u32 {
    num.count_ones()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения");
    let n: u64 = input.trim().parse().expect("Ожидается положительное число");

    let mut best_pair = (0, n);
    let mut max_bananas = count_bananas(0) + count_bananas(n);
    let mut max_diff = n;

    // Перебираем все возможные пары чисел (a, b), где a + b = N
    for a in 1..=n/2 {
        let b = n - a;
        let current_bananas = count_bananas(a) + count_bananas(b);

        // Если нашли пару с большим количеством бананов
        if current_bananas > max_bananas {
            max_bananas = current_bananas;
            best_pair = (a, b);
            max_diff = b - a;
        }
        // Если количество бананов такое же, но разность больше
        else if current_bananas == max_bananas && (b - a) > max_diff {
            best_pair = (a, b);
            max_diff = b - a;
        }
    }

    println!("{} {}", best_pair.0, best_pair.1);
}
