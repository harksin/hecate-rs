use super::event_store::{AggregateId, AggregateVersion, EventStore, HecateEvent};

struct MemoryEventStore<Event>
where
    Event: HecateEvent + Copy,
{
    store: Vec<Event>,
}

impl<Event> EventStore<Event> for MemoryEventStore<Event>
where
    Event: HecateEvent + Copy,
{
    fn write_event(&self, aggregateId: AggregateId) -> AggregateVersion {
        unimplemented!();
    }
    fn read_events(&self, aggregateId: AggregateId) -> Vec<Event> {
        self.store
            .clone()
            .into_iter()
            .filter(|evt| evt.aggregate_id() == aggregateId)
            .collect::<Vec<Event>>()
    }
}
