use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No arguments passed");
    }

    let income = &args[1];
    let income_num;

    //Checks if argument passed is a number
    match income.parse::<i32>() {
        Ok(n) => income_num = n,
        Err(_e) => panic!("Not a valid number entered!"),
    }

    // Calculates tax and returns the value
    let tax = calculate_tax(income_num);
    let new_income = income_num - tax;

    //Prints to screen
    println!("Your yearly income after tax will be €{}", new_income);
    println!(
        "Your monthly income after tax will be €{}",
        new_income / 12
    );
    println!(
        "Your weekly income after tax will be €{}",
        new_income / 52
    );
}

fn calculate_tax(income: i32) -> i32 {
    if income > 34550 {
        let higher_tax_band = income - 34550;
        let higher_tax_band_result = higher_tax_band * 40 / 100;
        let lower_tax_band_result = 34550 * 20 / 100;

        return higher_tax_band_result + lower_tax_band_result;
    }

    return income * 34 / 100;
}
