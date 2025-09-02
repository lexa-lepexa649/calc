use std::io;



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
    match base.checked_pow(exponent.try_into().unwrap()) {
        Some(result) => result,
        None => panic!("Переполнение при вычислении {}^{}", base, exponent),
    }
    //base.pow(exponent.try_into().unwrap())
}

fn sqrt(num: f64) -> f64 {
    num.sqrt()
}



fn calc() {
    loop {
        let mut res: i32 = 0;
    let mut res_1: f64 = 0.0;
   
    println!("Введите calc и имя команды, которую хотите использовать(div, mul, sub, add, pow, sqrt, exit): ");

////////////////////////////////////////////////////////////////////

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка чтения ввода");
    
    let values: Vec<&str> = input.trim().split_whitespace().collect();
    
    if values.len() < 2 {
        println!("Нужно ввести хотя бы два значения!");
        return;
    }
    
    let num1: String = values[0].parse().expect("Первое значение не число");
    let num2: String = values[1].parse().expect("Второе значение не число");


    if num1 == "sqrt" {
        println!("Введите число, из которого хотите извлечь корень: ");
        let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка чтения ввода");
    
    let number: f64 = input.trim().parse().expect("Введите корректное число!");
    if number < 0.0 {
        println!("Нельзя извлечь корень из отрицательного числа!");
        return;
    }
    let res = sqrt(number);
    print!("{}", res)
    
    

    } else if num2 == "div" || num2 == "mul" || num2 == "sub" || num2 == "add" || num2 == "pow" {
        println!("Введите два числа через пробел: ");

            let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Ошибка чтения ввода");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
            
        if parts.len() < 2 {
        println!("Нужно ввести хотя бы два значения!");
        return;
    } 

        let num_1: i32 = parts[0].parse().expect("Некорректное число!");
        let num_2: i32 = parts[1].parse().expect("Некорректное число!");

        let tuple = (num_1, num_2);

        if num_2 == 0 && num2 == "div" {
            println!("На ноль делить нельзя");
            return;
        }
        println!("Данные: {:?}", tuple); 
        let choice: &str = &values[1];
            match choice {
                "div" => res_1 = div(tuple.0.into(), tuple.1.into()),
                "mul" => res = mul(tuple.0, tuple.1),
                "sub" => res = sub(tuple.0, tuple.1),
                "add" => res = add(tuple.0, tuple.1),
                "pow" => res = pow(tuple.0, tuple.1),
                _ => println!("Вы ввели хуйню, сэр!")
            }
            
        } else if values[1] == "exit" {
            break;
        } else {
        println!("Неизвестная операция: {}", num2);
        return;
    }
    if values[1] == "div" {println!("Ответ: {}", res_1)}
    else {println!("Ответ: {}", res)}
    }
    }


fn main() {
    calc();
}

