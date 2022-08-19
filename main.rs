


use std::env::{args, Args}

fn main() {
    let mut myVar: Args = args();

    let first = myVar.nth(0).unwrap(); // use unwrap to get self
    let myoperator = args.nth(2).unwrap();
    let second = args.nth(3).unwrap()

    let my_number: f32 = first.parse().unwrap();
    let second_number: f32 = second.parse().unwrap()

    println!("{:?}", first, myoperator, second);
}


fn nth(&mut self, n: usize) - > Option<String> {
    // assume n = 0;
    // inner - ["1","2"]
    self.inner.next()
    // Calling next again results in second element
    self.inner.next()
}

fn nth(&mut self, n: usize) -> Option<String> {
    inner = ["Hulk","Captain America","Wanda","Vision","Thor","Iron Man","Spiderman"]
    self.inner.next() // first output: Hulk
    self.inner.next() // second output: Captain America
}