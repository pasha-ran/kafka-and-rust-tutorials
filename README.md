# kafka-and-rust-tutorials
a collection of demos, tutorials, and simple systems to demonstrate understanding of and key features in kafka and rust.


Kafka:
produce.py and consume.py are simple systems to demonstrate the core functionality of kafka - event streaming. Data can be sent asynchronously from produce to consume, also making use of utf-8 serialization. It also shows how the data is iterable and can be used in mongo type processes

demo2 is a similar system, but also shows other information in the stream such as the particular offset.

RUST:
parkinglot.rs demonstrates safe multi-threading in rust. Cars will concurrently try to enter a shared parking lot. They will succeed or fail depending on the capacity available. It uses std features such as Mutex, Arc, .clone()

parkinglot2.rs demonstrates a parking lot management system which operates through command line and standard output. It stores data on cars and respective parking spaces, prevents parking cars in a spot that is already occupied. It makes use of Rust's approach to dynamic memory and ownership, also uses reads from user input.
