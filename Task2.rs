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
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16
];

const INV_SBOX: [Byte; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d
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
