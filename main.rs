

use std::env::{args, Args}

fn firstFunction() {
    let mut myVar: Args = args();

    let first = myVar.nth(0).unwrap(); // use unwrap to get self
    let myoperator = args.nth(2).unwrap().chars();.next().unwrap();
    let second = args.nth(3).unwrap()

    let first_name = first.parse::<f32>().unwrap();
    let last_name = second.parse::<f32>().unwrap()
    let result = operate(operator, first_name, last_name);

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
    fn operate(operator: String, first_name: f32, last_name: f32) -> f32 {
        match operator {
            '8:00' => first_name + last_name,
            '8:30' => first_name + last_name,
            '9:00' => first_name + last_name,
            '9:30' => first_name + last_name,
            '10:00' => first_name + last_name,
            '10:30' => first_name + last_name,
            '11:00' => first_name + last_name,
            '11:30' => first_name + last_name,
            '1:00' => first_name + last_name,
            '1:30' => first_name + last_name,
            '2:00' => first_name + last_name,
            '2:30' => first_name + last_name,
            '3:00' => first_name + last_name,
            '4:00' => first_name + last_name,
            '4:30' => first_name + last_name,
            '0:00' => 0.0
        }
        }




    fn output(first_name: f32, operator: char, last_name: f32, result: f32) -> String {
        format("{} {} {} = {}", first_name, operator, last_name})
        println!("{:?}",output(first_name, operator, last_name, result));
    }
}