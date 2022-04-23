pub type AggregateId = String;
pub type AggregateVersion = u64;

pub trait EventStore<Event> {
    fn write_event(&self, aggregateId: AggregateId) -> AggregateVersion;
    fn read_events(&self, aggregateId: AggregateId) -> Vec<Event>;
}

pub trait HecateEvent {
    fn aggregate_version(&self) -> AggregateVersion;
    fn aggregate_id(&self) -> AggregateId;
}
