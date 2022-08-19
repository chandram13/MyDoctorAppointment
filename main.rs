

use std::env::{args, Args}

fn firstFunction() {
    let mut myVar: Args = args();

    let first = myVar.nth(0).unwrap(); // use unwrap to get self
    let myoperator = args.nth(2).unwrap().chars();.next().unwrap();
    let second = args.nth(3).unwrap()

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap()
    let result = operate(operator, my_number, second_number);

    println!("{:?}", result);
}



fn nth(&mut self, n: usize) -> Option<String> {
    doctorChoice = ["Doctor 1","Doctor 2","Doctor 3","Doctor 4"]
    self.inner.next() 
    self.inner.next() 
    self.inner.next()
    self.inner.next()
}

    // matching expression technique (Rust)
    fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
        match operator {
            '8:00' => first_number + second_number,
            '8:30' => first number + second_number,
            '9:00' => first_number + second_number,
            '9:30' => first_number + second_number,
            '10:00' => first_number + second_number,
            '10:30' => first_number + second_number,
            '11:00' => first_number + second_number,
            '11:30' => first_number + second_number,
            '1:00' => first_number + second_number,
            '1:30' => first_number + second_number,
            '2:00' => first_number + second_number,
            '2:30' => first_number + second_number,
            '3:00' => first_number + second_number,
            '4:00' => first_number + second_number,
            '4:30' => first_number + second_number,
            '0:00' => 0.0
        }
        }




    fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
        format("{} {} {} = {}", first_number, operator, second_number})
        println!("{:?}",output(first_number, operator, second_number, result));
    }
}