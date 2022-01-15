use crate::core::event::Event;

pub trait AggregateEntity<TEventBase> {
    fn uncommitted_events(&self) -> &Vec<TEventBase>;

    fn hydrate(&mut self, events: Vec<TEventBase>) {
        for event in events {
            self.apply_event(event);
        }
    }

    fn apply_event(&mut self, event: TEventBase);
}