use rand::Rng;
use std::io;
use std::fmt;

type Byte = u8;
type Block = [[Byte; 4]; 4];

// Генерация случайного ключа
fn generate_random_key(length: usize) -> Vec<Byte> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen()).collect()
}

// Генерация случайного IV
fn generate_random_iv() -> Block {
    let mut rng = rand::thread_rng();
    let mut iv = [[0; 4]; 4];
    for row in &mut iv {
        for byte in row.iter_mut() {
            *byte = rng.gen();
        }
    }
    iv
}

// Вывод ключа
fn print_key(key: &[Byte]) {
    print!("Ключ: ");
    for &b in key {
        print!("{:02x} ", b);
    }
    println!();
}

// Вывод блока
fn print_block(block: &Block, title: &str) {
    if !title.is_empty() {
        println!("{}", title);
    }
    for row in block {
        for &byte in row {
            print!("{:02x} ", byte);
        }
        println!();
    }
}

// Преобразование текста в блоки
fn text_to_blocks(text: &str) -> Vec<Block> {
    let mut padded = text.to_string();
    while padded.len() % 16 != 0 {
        padded.push(' ');
    }

    let mut blocks = Vec::new();
    for chunk in padded.as_bytes().chunks(16) {
        let mut block = [[0; 4]; 4];
        for (i, &byte) in chunk.iter().enumerate() {
            block[i % 4][i / 4] = byte;
        }
        blocks.push(block);
    }
    blocks
}

// Преобразование блоков в текст
fn blocks_to_text(blocks: &[Block]) -> String {
    let mut text = String::new();
    for block in blocks {
        for col in 0..4 {
            for row in 0..4 {
                text.push(block[row][col] as char);
            }
        }
    }
    text.trim_end().to_string()
}

// S-box и обратный S-box
const SBOX: [Byte; 256] = [
    // ... (полный S-box из оригинального кода)
];

const INV_SBOX: [Byte; 256] = [
    // ... (полный обратный S-box из оригинального кода)
];

// Константы для расширения ключа
const RCON: [Byte; 11] = [0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];

// Расширение ключа
fn expand_key(key: &[Byte]) -> Vec<Block> {
    let mut expanded_key = vec![0; 176];
    expanded_key[..16].copy_from_slice(&key[..16]);

    let mut bytes_generated = 16;
    let mut rcon_index = 1;
    let mut temp = [0; 4];

    while bytes_generated < 176 {
        temp.copy_from_slice(&expanded_key[bytes_generated-4..bytes_generated]);

        if bytes_generated % 16 == 0 {
            temp.rotate_left(1);
            for byte in &mut temp {
                *byte = SBOX[*byte as usize];
            }
            temp[0] ^= RCON[rcon_index];
            rcon_index += 1;
        }

        for i in 0..4 {
            expanded_key[bytes_generated] = expanded_key[bytes_generated - 16] ^ temp[i];
            bytes_generated += 1;
        }
    }

    let mut round_keys = Vec::new();
    for i in 0..11 {
        let mut block = [[0; 4]; 4];
        for j in 0..16 {
            block[j % 4][j / 4] = expanded_key[i * 16 + j];
        }
        round_keys.push(block);
    }
    round_keys
}

// XOR блоков
fn xor_blocks(a: &Block, b: &Block) -> Block {
    let mut result = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            result[i][j] = a[i][j] ^ b[i][j];
        }
    }
    result
}

// Умножение в поле Галуа
fn gmul(a: Byte, b: Byte) -> Byte {
    let mut p = 0;
    let mut hi_bit_set;
    let mut a = a;
    let mut b = b;
    
    for _ in 0..8 {
        if b & 1 != 0 {
            p ^= a;
        }
        hi_bit_set = a & 0x80;
        a <<= 1;
        if hi_bit_set != 0 {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    p
}

// Шаги AES
fn sub_bytes(state: &mut Block) {
    for row in state {
        for byte in row {
            *byte = SBOX[*byte as usize];
        }
    }
}

fn shift_rows(state: &mut Block) {
    let temp = *state;
    for i in 1..4 {
        for j in 0..4 {
            state[i][j] = temp[i][(j + i) % 4];
        }
    }
}

fn mix_columns(state: &mut Block) {
    for col in 0..4 {
        let s0 = state[0][col];
        let s1 = state[1][col];
        let s2 = state[2][col];
        let s3 = state[3][col];
        
        state[0][col] = gmul(s0, 2) ^ gmul(s1, 3) ^ s2 ^ s3;
        state[1][col] = s0 ^ gmul(s1, 2) ^ gmul(s2, 3) ^ s3;
        state[2][col] = s0 ^ s1 ^ gmul(s2, 2) ^ gmul(s3, 3);
        state[3][col] = gmul(s0, 3) ^ s1 ^ s2 ^ gmul(s3, 2);
    }
}

// Шифрование блока
fn encrypt_block(input: &Block, round_keys: &[Block]) -> Block {
    println!("\nНачало шифрования блока:");
    print_block(input, "Исходный блок:");

    let mut state = xor_blocks(input, &round_keys[0]);
    print_block(&state, "После AddRoundKey (раунд 0):");

    for round in 1..10 {
        sub_bytes(&mut state);
        print_block(&state, &format!("После SubBytes (раунд {}):", round));

        shift_rows(&mut state);
        print_block(&state, &format!("После ShiftRows (раунд {}):", round));

        mix_columns(&mut state);
        print_block(&state, &format!("После MixColumns (раунд {}):", round));

        state = xor_blocks(&state, &round_keys[round]);
        print_block(&state, &format!("После AddRoundKey (раунд {}):", round));
    }

    sub_bytes(&mut state);
    print_block(&state, "После SubBytes (раунд 10):");

    shift_rows(&mut state);
    print_block(&state, "После ShiftRows (раунд 10):");

    state = xor_blocks(&state, &round_keys[10]);
    print_block(&state, "После AddRoundKey (раунд 10):");

    println!("Конец шифрования блока");
    state
}

// Обратные шаги AES
fn inv_sub_bytes(state: &mut Block) {
    for row in state {
        for byte in row {
            *byte = INV_SBOX[*byte as usize];
        }
    }
}

fn inv_shift_rows(state: &mut Block) {
    let temp = *state;
    for i in 1..4 {
        for j in 0..4 {
            state[i][j] = temp[i][(j - i + 4) % 4];
        }
    }
}

fn inv_mix_columns(state: &mut Block) {
    for col in 0..4 {
        let s0 = state[0][col];
        let s1 = state[1][col];
        let s2 = state[2][col];
        let s3 = state[3][col];
        
        state[0][col] = gmul(s0, 0x0e) ^ gmul(s1, 0x0b) ^ gmul(s2, 0x0d) ^ gmul(s3, 0x09);
        state[1][col] = gmul(s0, 0x09) ^ gmul(s1, 0x0e) ^ gmul(s2, 0x0b) ^ gmul(s3, 0x0d);
        state[2][col] = gmul(s0, 0x0d) ^ gmul(s1, 0x09) ^ gmul(s2, 0x0e) ^ gmul(s3, 0x0b);
        state[3][col] = gmul(s0, 0x0b) ^ gmul(s1, 0x0d) ^ gmul(s2, 0x09) ^ gmul(s3, 0x0e);
    }
}

// Дешифрование блока
fn decrypt_block(input: &Block, round_keys: &[Block]) -> Block {
    println!("\nНачало дешифрования блока:");
    print_block(input, "Зашифрованный блок:");

    let mut state = xor_blocks(input, &round_keys[10]);
    print_block(&state, "После AddRoundKey (раунд 10):");

    for round in (1..10).rev() {
        inv_shift_rows(&mut state);
        print_block(&state, &format!("После InvShiftRows (раунд {}):", round));

        inv_sub_bytes(&mut state);
        print_block(&state, &format!("После InvSubBytes (раунд {}):", round));

        state = xor_blocks(&state, &round_keys[round]);
        print_block(&state, &format!("После AddRoundKey (раунд {}):", round));

        inv_mix_columns(&mut state);
        print_block(&state, &format!("После InvMixColumns (раунд {}):", round));
    }

    inv_shift_rows(&mut state);
    print_block(&state, "После InvShiftRows (раунд 0):");

    inv_sub_bytes(&mut state);
    print_block(&state, "После InvSubBytes (раунд 0):");

    state = xor_blocks(&state, &round_keys[0]);
    print_block(&state, "После AddRoundKey (раунд 0):");

    println!("Конец дешифрования блока");
    state
}

// Режим CBC
fn aes_cbc_encrypt(plaintext: &[Block], round_keys: &[Block], iv: &Block) -> Vec<Block> {
    let mut ciphertext = Vec::new();
    let mut previous = *iv;

    for (i, block) in plaintext.iter().enumerate() {
        println!("\nШифрование блока {} из {}", i + 1, plaintext.len());
        print_block(&previous, "Вектор инициализации (IV) для этого блока:");

        let xored = xor_blocks(block, &previous);
        print_block(&xored, "После XOR с IV:");

        let encrypted = encrypt_block(&xored, round_keys);
        ciphertext.push(encrypted);
        previous = encrypted;
    }
    ciphertext
}

fn aes_cbc_decrypt(ciphertext: &[Block], round_keys: &[Block], iv: &Block) -> Vec<Block> {
    let mut decrypted = Vec::new();
    let mut previous = *iv;

    for (i, block) in ciphertext.iter().enumerate() {
        println!("\nДешифрование блока {} из {}", i + 1, ciphertext.len());
        print_block(&previous, "Вектор инициализации (IV) для этого блока:");

        let decrypted_block = decrypt_block(block, round_keys);
        let plain = xor_blocks(&decrypted_block, &previous);
        decrypted.push(plain);
        previous = *block;

        print_block(&plain, "После XOR с IV:");
    }
    decrypted
}

fn main() {
    println!("Введите текст для шифрования:");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Ошибка чтения");
    let input_text = input_text.trim();

    // Генерация ключей
    let master_key = generate_random_key(16);
    let iv = generate_random_iv();

    println!("\nГенерация ключей и вектора инициализации");
    println!("========================================");
    print_key(&master_key);
    print_block(&iv, "\nВектор инициализации:");

    // Подготовка данных
    let plaintext_blocks = text_to_blocks(input_text);

    println!("\nПреобразование текста в блоки");
    println!("============================");
    for (i, block) in plaintext_blocks.iter().enumerate() {
        print_block(block, &format!("Блок текста {}:", i + 1));
    }

    // Расширение ключа
    let round_keys = expand_key(&master_key);

    println!("\nРаундовые ключи");
    println!("===============");
    for (i, key) in round_keys.iter().enumerate() {
        print_block(key, &format!("Раундовый ключ {}:", i));
    }

    // Шифрование
    println!("\nПроцесс шифрования (AES-CBC)");
    println!("============================");
    let ciphertext_blocks = aes_cbc_encrypt(&plaintext_blocks, &round_keys, &iv);

    // Сохранение зашифрованных данных
    let ciphertext_str: Vec<u8> = ciphertext_blocks.iter()
        .flat_map(|block| {
            (0..4).flat_map(move |col| (0..4).map(move |row| block[row][col]))
        })
        .collect();

    println!("\nЗашифрованные данные сохранены в памяти");

    println!("\nРезультаты шифрования");
    println!("====================");
    for (i, block) in ciphertext_blocks.iter().enumerate() {
        print_block(block, &format!("Зашифрованный блок {}:", i + 1));
    }

    // Дешифрование
    println!("\nПроцесс дешифрования (AES-CBC)");
    println!("==============================");
    let decrypted_blocks = aes_cbc_decrypt(&ciphertext_blocks, &round_keys, &iv);
    let decrypted_text = blocks_to_text(&decrypted_blocks);

    println!("\nРезультаты дешифрования");
    println!("=======================");
    for (i, block) in decrypted_blocks.iter().enumerate() {
        print_block(block, &format!("Дешифрованный блок {}:", i + 1));
    }

    println!("\nИсходный текст:");
    println!("{}", input_text);
    println!("\nРасшифрованный текст:");
    println!("{}", decrypted_text);

    println!("\nЗашифрованный текст:");
    for byte in ciphertext_str {
        print!("{:02x} ", byte);
    }
    println!();
}
