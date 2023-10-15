use std::io;

fn main() {
    println!("Добро пожаловать в калькулятор!");

    loop {
        let mut input = String::new();

        println!("Введите выражение (например, 2 + 2): ");
        io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 3 {
            println!("Неверный формат ввода. Пожалуйста, введите выражение в формате 'число оператор число'.");
            continue;
        }

        let num1: f64 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ошибка при парсинге числа '{}'", parts[0]);
                continue;
            }
        };

        let operator = parts[1];

        let num2: f64 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ошибка при парсинге числа '{}'", parts[2]);
                continue;
            }
        };

        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    println!("Ошибка: деление на ноль!");
                    continue;
                }
            }
            _ => {
                println!("Неподдерживаемый оператор '{}'", operator);
                continue;
            }
        };

        println!("Результат: {}", result);
    }

    println!("Спасибо за использование калькулятора!");
}
