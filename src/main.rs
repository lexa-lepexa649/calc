use std::io;

fn div(a: i32, b: i32) -> Result<f64, String> {
    if b == 0 {
        return Err("Деление на ноль невозможно".to_string());
    }
    Ok(a as f64 / b as f64)
}

fn mul(a: i32, b: i32) -> Result<i32, String> {
    Ok(a * b)
}

fn sub(a: i32, b: i32) -> Result<i32, String> {
    Ok(a - b)
}

fn add(a: i32, b: i32) -> Result<i32, String> {
    Ok(a + b)
}

fn pow(a: i32, b: i32) -> Result<i32, String> {
    if b > 10 {
        println!("Предупреждение: Результат может быть слишком большим");
    }
    Ok(a.pow(b as u32))
}

fn sqrt(a: i32) -> Result<f64, String> {
    if a < 0 {
        return Err("Нельзя извлечь корень из отрицательного числа".to_string());
    }
    Ok((a as f64).sqrt())
}

fn calc() {
    loop {
        println!("Введите calc и команду (div, mul, sub, add, pow, sqrt, exit): ");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        // Выход
        if parts.get(0) == Some(&"exit") {
            println!("Выход из калькулятора");
            break;
        }
        
        // Обработка команд
        let result = match parts.as_slice() {
            ["calc", "exit"] => {
                println!("Выход");
                break;
            },
            
            // Команды с одним аргументом
            ["calc", "sqrt", num] => {
                num.parse::<i32>()
                    .map_err(|_| format!("Некорректное число: {}", num))
                    .and_then(|n| sqrt(n))
            },
            
            // Команды с двумя аргументами
            ["calc", command, num1, num2] => {
                let a = num1.parse::<i32>()
                    .map_err(|_| format!("Некорректное число: {}", num1))?;
                
                let b = num2.parse::<i32>()
                    .map_err(|_| format!("Некорректное число: {}", num2))?;
                
                match *command {
                    "div" => div(a, b),
                    "mul" => mul(a, b).map(|res| res as f64),
                    "sub" => sub(a, b).map(|res| res as f64),
                    "add" => add(a, b).map(|res| res as f64),
                    "pow" => pow(a, b).map(|res| res as f64),
                    _ => Err(format!("Неизвестная операция: {}", command)),
                }
            },
            
            // Недостаточно аргументов
            ["calc", "div"] | ["calc", "mul"] | ["calc", "sub"] | ["calc", "add"] | ["calc", "pow"] => {
                Err("Недостаточно аргументов для операции".to_string())
            },
            
            ["calc", "sqrt"] => {
                Err("Недостаточно аргументов для извлечения корня".to_string())
            },
            
            // Неизвестная команда
            ["calc", command, ..] if !["div", "mul", "sub", "add", "pow", "sqrt"].contains(command) => {
                Err(format!("Неизвестная операция: {}", command))
            },
            
            // Неправильный формат
            _ => Err("Неверный формат! Используйте: calc команда число1 [число2]".to_string()),
        };
        
        // Вывод результата или ошибки
        match result {
            Ok(res) => println!("Результат: {:.2}", res),
            Err(err) => println!("Ошибка: {}", err),
        }
        
        println!(); // Пустая строка для читаемости
    }
}

fn main() {
    calc();
}