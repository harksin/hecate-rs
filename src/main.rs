mod hecate_core;

fn main() {
    println!("Hello, world!");
}

// trait HecateIds {
//     type Ids;
//     fn new() -> Self::Ids;
//     fn toString() -> String;
// }

struct AggregateRoot<Root> {
    agg: Option<Root>,
}

trait Context {
    type Aggregate;
    type Command;
    type Event;
    type AggError;
    fn empty() -> AggregateRoot<Self::Aggregate> {
        AggregateRoot { agg: None }
    }
    fn apply(value: Self::Aggregate) -> AggregateRoot<Self::Aggregate> {
        AggregateRoot { agg: Some(value) }
    }

    fn execute_command(
        aggregate: AggregateRoot<Self::Aggregate>,
        command: Self::Command,
    ) -> Result<Self::AggError, Vec<Self::Event>>;

    fn apply_event(
        aggregate: AggregateRoot<Self::Aggregate>,
        event: Self::Event,
    ) -> AggregateRoot<Self::Aggregate>;
    fn apply_history(
        aggregate: AggregateRoot<Self::Aggregate>,
        event: Vec<Self::Event>,
    ) -> AggregateRoot<Self::Aggregate>;
}
