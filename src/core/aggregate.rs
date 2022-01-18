pub trait AggregateEntity<TEventBase> {
    fn uncommitted_events(&self) -> &Vec<TEventBase>;
    fn uncommitted_events_mut(&mut self) -> &mut Vec<TEventBase>;

    fn hydrate<TEntity: AggregateEntity<TEventBase> + Default>(events: Vec<TEventBase>) -> TEntity {
        let mut agg: TEntity = Default::default();

        for event in &events {
            agg.apply(event);
        }

        agg
    }

    fn apply(&mut self, event: &TEventBase);

    fn apply_event(&mut self, event: TEventBase) {
        self.apply(&event);
        self.uncommitted_events_mut().push(event);
    }
}
