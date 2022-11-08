use std::io::stdin;

fn main() {
    println!("Введите значение х");
    let x = read_var();
    if x >= 0.0 && x <= 3.0 {
        let x = x.powi(2);
        println!("{x}");
    }   else {
        println!("4");
    }
}

fn read_var() -> f64 {
    loop {
        let mut x =String::new();
        stdin()
            .read_line(&mut x)
            .expect("Ошибка чтения ввода"); //считывание ввода и запись его в переменную X (с точки зрения компилятора это одна строка)

        let x: f64 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Переменная должна быть равна числу");
                continue;
            },
        }; //преобразование ввода со string в float-point integer с перезапуском loop в случае ошибки
        break x;
    }
}