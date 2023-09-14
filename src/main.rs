use std::io;

fn main() {

    let h:Helper = Helper::init();

    println!("If you fill this keg completely you would roughly need {} of wort\n", h.needed_wort(h.keg_size));
	println!("So make sure to leave enough space for your wort in your keg!!");
	println!();

    let mut input = String::new();

    println!("How much beer do you want to put in your keg?");
    io::stdin().read_line(&mut input).expect("failed to read line");

    let beer_amount: f64 = input.trim().parse().expect("invaild input");
    input.clear();




	println!();
	println!("For this keg you need {} ml of Wort\n", h.needed_wort(beer_amount));
	println!("The keg needs {} of pressure\n", h.required_pressure);
	println!("Your alcohol content will be roughly {}%\n", h.alcohol_content());

}


pub struct Helper {
    pub grad_plato: f64,
    pub desired_co2: f64,
    pub beer_gravity: f64,
    pub required_pressure: f64,
    pub keg_size: f64,
}

impl Helper{

    pub fn init() -> Self{

        let mut input = String::new();

        println!("What is the Keg Size? (ml)");
        io::stdin().read_line(&mut input).expect("failed to read line");

        let keg_size: f64 = input.trim().parse().expect("invaild input");
        input.clear();


        println!("What is the current temperature? (˚C)");
        io::stdin().read_line(&mut input).expect("failed to read line");

        let temperature: f64 = input.trim().parse().expect("invaild input");
        input.clear();


        println!("What is your desired CO2 amount?");
        io::stdin().read_line(&mut input).expect("failed to read line");

        let desired_co2: f64 = input.trim().parse().expect("invaild input");
        input.clear();


        println!("How much Grad Plato did it have?");
        io::stdin().read_line(&mut input).expect("failed to read line");

        let grad_plato: f64 = input.trim().parse().expect("invaild input");
        input.clear();


        println!("What is your gravity after fermentation? (Use 2 for estimation before fermentation)");
        io::stdin().read_line(&mut input).expect("failed to read line");

        let beer_gravity: f64 = input.trim().parse().expect("invaild input");
        input.clear();
        
        let required_pressure = desired_co2 / (10.0 * f64::powf(std::f64::consts::E, -10.73797 + (2617.25 / (temperature + 273.15)))) - 1.013;

        Helper{
            grad_plato,
            desired_co2,
            beer_gravity,
            required_pressure,
            keg_size

        }




    }

    fn needed_co2(&self, beer_amount: f64, estimated_wort_amount: f64) -> f64{
        self.needed_co2_gas(beer_amount, estimated_wort_amount) + self.needed_co2_beer(beer_amount)
    }

    fn needed_co2_gas(&self, beer_amount: f64, estimated_wort_amount: f64) -> f64{
        let mut left_over_air = self.keg_size - beer_amount - estimated_wort_amount;
        if left_over_air < 0.0 {
            left_over_air = 0.0
        }
        left_over_air * 1.83 * self.required_pressure
    } 

    fn needed_co2_beer(&self, beer_amount: f64) -> f64 {
        beer_amount * 1.66 * self.required_pressure
    }

    fn needed_sugar(&self, beer_amount: f64, estimated_wort_amount: f64) -> f64{
        self.needed_co2(beer_amount, estimated_wort_amount) * 2.0 * 0.957
    }

    fn real_gravity(&self)-> f64{
        0.1808* self.grad_plato + 0.8192 * self.beer_gravity
    }

    pub fn alcohol_content(&self) -> f64 {        
            round_to_two_decimal_places((1.0 / 0.795 * (self.grad_plato - self.real_gravity()) / (2.0665 - 0.010665 * self.grad_plato)) * 100.0)
    }

    fn needed_wort(&self, beer_amount: f64) -> f64 {
        let estimated_wort_amount = self.needed_sugar(beer_amount, 0.0) / ((self.grad_plato - self.real_gravity()) * 10.0);
        ((self.needed_sugar(beer_amount, estimated_wort_amount) / ((self.grad_plato - self.real_gravity()) * 10.0)) * 1000.0).round()
    }



}

fn round_to_two_decimal_places(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}