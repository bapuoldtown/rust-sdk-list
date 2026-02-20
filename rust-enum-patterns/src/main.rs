mod enum1;
use enum1::{ConnectionState, DatabaseError};
mod enum2;
fn main() {
    println!("Hello, world!");
    println!("The enum value is {:?}", ConnectionState::Connecting);
    println!("The enum value is {:?}", DatabaseError::ConnectionRefused);
    //kets do some matching on the enum values
    let latest_state= DatabaseError::Timeout { after_secs: 90 };

    //match the enum state
    match latest_state{
        DatabaseError::Timeout {after_secs} => println!("Database timed out after {} seconds", after_secs),
        _ => println!("Some other database error"),
    }

    let error_message : &str = match latest_state{
        DatabaseError::Timeout{after_secs: _} => "Database timeout error",
        _=>"Generic error",
    };
    println!("Error message: {}", error_message);

    println!("Result of operation: {}", enum2::handle_operation(enum2::OperationCatalogue::Add {a: 10, b: 20}));
    println!("Result of operation: {}", enum2::handle_operation(enum2::OperationCatalogue::Subtract {a: 10, b: 20}));
    println!("Result of operation: {}", enum2::handle_operation(enum2::OperationCatalogue::Multiply {a: 10, b: 20}));
    println!("Result of operation: {}", enum2::handle_operation(enum2::OperationCatalogue::Divide {a: 10, b: 2}));

}
