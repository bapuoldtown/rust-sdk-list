pub enum OperationCatalogue{
    Add {a: usize, b: usize},
    Subtract {a: usize, b: usize},
    Multiply {a: usize, b: usize},
    Divide {a: usize, b: usize},

}

pub fn handle_operation(op: OperationCatalogue) -> usize{
    match op{
        OperationCatalogue::Add {a, b} => return a+b,
        OperationCatalogue::Subtract{a, b} => {if a> b{return a-b} else {return b-a}},
        OperationCatalogue::Multiply {a,b} => return a*b,
        OperationCatalogue::Divide{a,b} => {if a==0 ||b==0 {return 0} else {return a/b}}
    }
}