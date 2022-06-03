pub fn add_them(x: &u32, y: &u32) -> u64 {
    (*x as u64) + (*y as u64)
}

pub fn add_them2(x: u32, y: u32) -> u64 {
    (x as u64) + (y as u64)
}

pub fn add_them3(x: u32, y: &u32) -> u64 {
    (x as u64) + (*y as u64)
}

pub fn make_random_string(len: usize, mut letters: u8, mut numbers: u8) -> String {
    use rand::Rng;

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz";
    const NUMSET: &[u8] = b"0123456789";
    const PUNCSET: &[u8] = b"!@#$%^&*()_+=,.<>[]{}\\";

    if letters > 100 {
        letters = 100;
        numbers = 0;
    } else {
        numbers = letters + numbers;
        if numbers > 100 {
            numbers = 100;
        }
    }

    let punc = 100 - numbers;
    assert_eq!(100, numbers + punc);

    let mut rng = rand::thread_rng();

    let password: String = (0..len)
        .map(|_| {
            let chance = rng.gen_range(1..100);
            if chance <= letters {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            } else if chance <= numbers {
                let idx = rng.gen_range(0..NUMSET.len());
                NUMSET[idx] as char
            } else {
                let idx = rng.gen_range(0..PUNCSET.len());
                PUNCSET[idx] as char
            }
        })
        .collect();

    password
}

pub fn numbers_letters(orig: &str) -> (String, String) {
    let guess_len = orig.len();
    let mut letters = String::with_capacity(guess_len);
    let mut numbers = String::with_capacity(guess_len);

    for c in orig.chars() {
        if !c.is_alphanumeric() {
            continue;
        } else if c.is_alphabetic() {
            letters.push(c);
        } else if c.is_digit(10) {
            numbers.push(c);
        }
    }
    letters.shrink_to_fit();
    numbers.shrink_to_fit();

    (letters, numbers)
}

pub fn numbers_letters_orig(orig: &str) -> (String, String) {
    let mut letters = String::new();
    let mut numbers = String::new();

    for c in orig.chars() {
        if c.is_digit(10) {
            numbers.push(c);
        }
        if c.is_alphabetic() {
            letters.push(c);
        }
    }

    (letters, numbers)
}

pub fn numbers_letters_iter(orig: &str) -> (String, String) {
    let numbers: String = orig.chars().filter(|c| c.is_digit(10)).collect();
    let letters: String = orig.chars().filter(|c| c.is_alphabetic()).collect();

    (letters, numbers)
}

pub fn numbers_letters_part(orig: &str) -> (String, String) {
    let (letters, numbers): (String, String) = orig
        .chars()
        .filter(|x| x.is_alphanumeric())
        .partition(|c| c.is_alphabetic());

    (letters, numbers)
}

pub fn numbers_letters_fold(orig: &str) -> (String, String) {
    let guess_len = orig.len();
    let (mut letters, mut numbers) = orig.chars().fold(
        (
            String::with_capacity(guess_len),
            String::with_capacity(guess_len),
        ),
        |(mut letters, mut numbers), c| {
            if c.is_digit(10) {
                numbers.push(c);
            } else if c.is_alphabetic() {
                letters.push(c);
            }
            (letters, numbers)
        },
    );

    letters.shrink_to_fit();
    numbers.shrink_to_fit();

    (letters, numbers)
}

#[test]
fn test_functions() {
    for _ in 0..100 {
        let mumble = make_random_string(1024, 66, 33);
        let (orig_str, orig_num) = numbers_letters_orig(&mumble);
        let (opt_str, opt_num) = numbers_letters(&mumble);
        let (iter_str, iter_num) = numbers_letters_iter(&mumble);
        let (part_str, part_num) = numbers_letters_part(&mumble);
        let (fold_str, fold_num) = numbers_letters_fold(&mumble);

        assert_eq!(orig_num, opt_num);
        assert_eq!(orig_num, iter_num);
        assert_eq!(orig_num, part_num);
        assert_eq!(orig_num, fold_num);

        assert_eq!(orig_str, opt_str);
        assert_eq!(orig_str, iter_str);
        assert_eq!(orig_str, part_str);
        assert_eq!(orig_str, fold_str);
    }
}
