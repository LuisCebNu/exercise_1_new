fn main() {
    let temperature = 89.9;
    let number = 5;
    println!("{} Farenheit is {:.2} Celsius", temperature, farenheit_to_celsius(temperature));
    println!("The {}th fibonacci is {}", number, fibonacci(number));
    christmas_carol();
}

//Exercise 1.1 Farenheit to Celsius
fn farenheit_to_celsius (tempe: f64) -> f64{
    //Formula: (32 °F − 32) × 5/9 = 0 °C
    (tempe - 32.0) * (5.0/9.0)
}

//Exercise 1.2 Fibonacci series
fn fibonacci(number_fib: i32) -> i32 {
   let result: i32;
   if number_fib == 0 {
    0
   } else if number_fib == 1{
    1
   } else {
    result = fibonacci(number_fib - 1) + fibonacci(number_fib - 2);
    result
   }
}

//Exercise 1.3 Christmas Carol "The twelve days of Christmas"
fn christmas_carol() {
    let day_number = ["First", "Second", "Third", "Fourth", "Fifh", "Sixth", "Seventh", "Eighth", "Ninth", "Tenth","Eleventh", "Twefth"];
    let day_gift = ["And a song for the Christmas tree", "Two candy canes","Three boughs of holly","Four colored lights","A shining star","Little silver bells","Candles a-glowing","Gold and silver tinsel","A guardian angel","Some mistletoe","Gifts for one and all","All their good wishes"]; 
    
    let mut count: usize = 0;

    for day in day_number.iter() {
        println!("On the {} day of christmas", day);
        println!("My good friends brought to me");

        if count == 0 {
            println!("{}", "A song for the Christmas tree");
        } else {
            for index in (0..=count).rev() {
                println!("{}",day_gift[index]);
            }
        }
        count += 1;
    }
}