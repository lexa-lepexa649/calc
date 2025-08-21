use std::io;
use crossterm::event::{read, Event, KeyCode};
use std::process;


fn div(num_1: f64, num_2: f64) -> f64 {
    num_1 / num_2
}

fn mul(num_1: i32, num_2: i32) -> i32 {
    num_1 * num_2
}

fn add(num_1: i32, num_2: i32) -> i32 {
    num_1 + num_2
}

fn sub(num_1: i32, num_2: i32) -> i32 {
    num_1 - num_2
}

fn pow(base: i32, exponent: i32) -> i32 {
    base.pow(exponent)
}

fn sqrt(num: f64) -> f64 {
    num.sqrt()
}

fn exit() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Ошибка чтения ввода");

        // Удаляем пробелы и перевод строки, приводим к нижнему регистру
        let cmd = input.trim().to_lowercase();

        if cmd == "q" || cmd == "exit" {
            println!("Завершение программы...");
            break;
        } else {calc()}
    }
}

fn calc() {
    let mut res: i32 = 0;
    let mut res_1: f64 = 0.0;
    println!("Введите числа через пробел (например: 1 2 3 4 5):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка чтения ввода");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 4 {
        panic!("Нужно ввести возраст и имя!");
    }

    let calc = parts[0].to_string();
    let typ = parts[1].to_string(); // Преобразуем &str в String
    let num_1: i32 = parts[2].parse().expect("Возраст должен быть числом");
    let num_2: i32 = parts[3].parse().expect("Возраст должен быть числом");

    let tuple = (calc, typ, num_1, num_2);
    println!("Данные: {:?}", tuple); // Например: (25, "Alice")

    if tuple.0 == "calc" {
        

            let choice: &str = &tuple.1;
            match choice {
                "div" => res_1 = div(tuple.2.into(), tuple.3.into()),
                "mul" => res = mul(tuple.2, tuple.3),
                "sub" => res = sub(tuple.2, tuple.3),
                "add" => res = add(tuple.2, tuple.3),
                "pow" => res = pow(tuple.2, tuple.3),
                _ => println!("Вы ввели хуйню, сэр!")
            }
            
        }
    if tuple.1 == "div" {println!("{}", res_1)}
    else {println!("{}", res)}

    

    // loop {
    //     if let Ok(Event::Key(event)) = read() {
    //         if event.code != KeyCode::Char('q') { // Можно задать конкретную клавишу
    //             break;
    //         }
    //     }
    // }

}

fn main() {
    // loop {
    //     if let Ok(Event::Key(event)) = read() {
    //         if event.code != KeyCode::Char('q') { // Можно задать конкретную клавишу
    //             break;
    //         } else {calc()}
    //     }
    // }
    loop {calc()}
}

// use std::io;

// #[derive(Debug)]
// struct MixedData {
//     id: i32,
//     name: String,
//     is_active: bool,
// }

// fn main() {
//     println!("Введите ID, имя и активность (число строка true/false):");
    
//     let input = read_input();
//     let data = parse_to_struct(&input);
    
//     println!("Структура: {:?}", data);
// }

// fn read_input() -> String {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Ошибка чтения");
//     input.trim().to_string()
// }

// fn parse_to_struct(input: &str) -> MixedData {
//     let parts: Vec<&str> = input.split_whitespace().collect();
    
//     if parts.len() < 3 {
//         panic!("Недостаточно данных");
//     }
    
//     MixedData {
//         id: parts[0].parse().expect("ID должен быть числом"),
//         name: parts[1].to_string(),
//         is_active: parts[2].parse().expect("Активность должна быть true/false"),
//     }
// }