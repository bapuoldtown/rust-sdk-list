#[derive(Debug)]
pub enum MyEnum{
    Submitted,
    InProgress,
    Completed,
    Failed,

}
//Rust Enums can carr data apart from labels like normal python Enum class will do
pub enum Message{
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    Color(u8, u8, u8),
}

fn main() {
    println!("Hello, world!");
    let submitted_value = MyEnum::InProgress;
    println!(
        "This is a Rust SDK for the Grafana HTTP API. enum value = {:?}",
        submitted_value
    );
    //match Enu variants 
    match submitted_value{
        MyEnum::Submitted => println!("The submitted value is {:?}", submitted_value),
        MyEnum::InProgress => println!("The value is {:?}", submitted_value),
        MyEnum::Completed => println!("The value is {:?}", submitted_value),
        MyEnum::Failed => println!("The value is {:?}", submitted_value),
    }

    //let my_state = Message::Color(10, 20, 30);
    let my_state = Message::Write(String::from("Guru"));

    match my_state{
        Message::Color(a, b, c) => println!("The pattern matched with tuple struct: {}, {}, {}", a, b, c),
        _ => println!("The pattern did not match with tuple struct"),
    }


}
