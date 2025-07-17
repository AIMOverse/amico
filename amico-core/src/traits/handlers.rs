use anyhow::Result;

use crate::events::{GlobalEvent, EventSet, EventSender};

use super::System;

/// The Observer trait is used to observe events.
///
/// Observers watch for an event and process it, without
/// modifying the event and affecting the state of the world.
pub trait Observer {
    /// The event type that the Observer observes.
    type Event: GlobalEvent + 'static;

    /// The observe method is called when the Observer is notified of an event.
    ///
    /// The method receives a reference to the event and returns a Result.
    fn observe(&self, event: &Self::Event) -> Result<()>;

    /// Converts the Observer into a SystemHandler.
    fn to_system(self) -> SystemHandler<Self, PhantomMediator>
    where
        Self: Sized,
    {
        SystemHandler::Observer(self)
    }
}

/// The Mediator trait is used to mediate events.
///
/// Mediators watch for an event and process it, and finally
/// take ownership of the event.
///
/// After processing, Mediators may send new events to the world.
pub trait Mediator {
    /// The event type that the Mediator mediates.
    type Event: GlobalEvent + 'static;

    /// The event type that the Mediator sends.
    type EventsToSend: EventSet;

    /// The mediate method is called when the Mediator is notified of an event.
    ///
    /// The method receives the event and a sender to send
    /// new events to the world.
    fn mediate(
        &self,
        event: Self::Event,
        sender: &mut dyn EventSender,
    ) -> Result<()>;

    /// Converts the Mediator into a SystemHandler.
    fn to_system(self) -> SystemHandler<PhantomObserver, Self>
    where
        Self: Sized,
    {
        SystemHandler::Mediator(self)
    }
}

/// The placeholder event for the observer and mediator to use in SystemHandler.
#[derive(Debug, Clone)]
pub struct PhantomEvent;

impl GlobalEvent for PhantomEvent {}

/// The placeholder observer for SystemHandler.
pub struct PhantomObserver;

/// The placeholder mediator for SystemHandler.
pub struct PhantomMediator;

impl Observer for PhantomObserver {
    type Event = PhantomEvent;

    fn observe(&self, _event: &Self::Event) -> Result<()> {
        Ok(())
    }
}

impl Mediator for PhantomMediator {
    type Event = PhantomEvent;
    type EventsToSend = ();

    fn mediate(
        &self,
        _event: Self::Event,
        _sender: &mut dyn EventSender,
    ) -> Result<()> {
        Ok(())
    }
}

/// The SystemHandler is a middleware to convert an Observer or Mediator into a System.
pub enum SystemHandler<
    O: Observer + 'static = PhantomObserver,
    M: Mediator + 'static = PhantomMediator,
> {
    /// The Observer handler variant.
    Observer(O),

    /// The Mediator handler variant.
    Mediator(M),
}

impl<O, M> System for SystemHandler<O, M>
where
    O: Observer + Send + Sync + 'static,
    M: Mediator + Send + Sync + 'static,
{
    fn register_to(self, mut registry: crate::world::HandlerRegistry) {
        match self {
            SystemHandler::Observer(observer) => {
                registry.register::<O::Event, _>(move |event: &O::Event| {
                    if let Err(err) = observer.observe(event) {
                        tracing::error!("Error in observer: {}", err);
                    }
                    Ok(())
                })
            }
            SystemHandler::Mediator(mediator) => {
                registry.register_mediator::<M::Event, M::EventsToSend, _>(
                    move |event: M::Event, sender: &mut dyn EventSender| {
                        if let Err(err) = mediator.mediate(event, sender) {
                            tracing::error!("Error in mediator: {}", err);
                        }
                        Ok(())
                    },
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::world::HandlerRegistry;
    use crate::events::EventBus;

    use super::*;

    #[derive(Debug, Clone)]
    struct TestEvent(i32);

    impl GlobalEvent for TestEvent {}

    #[derive(Debug, Clone)]
    struct TestEvent2(i32);

    impl GlobalEvent for TestEvent2 {}

    struct TestObserver;

    impl Observer for TestObserver {
        type Event = TestEvent;

        fn observe(&self, event: &Self::Event) -> Result<()> {
            let TestEvent(_) = event;
            Ok(())
        }
    }

    struct TestMediator;

    impl Mediator for TestMediator {
        type Event = TestEvent;
        type EventsToSend = TestEvent2;

        fn mediate(
            &self,
            event: Self::Event,
            sender: &mut dyn EventSender,
        ) -> Result<()> {
            let num = event.0;

            sender.send_global(Box::new(TestEvent2(num * 2)));

            Ok(())
        }
    }

    #[test]
    fn test_mediator() {
        // Initialize event bus.
        let mut event_bus = EventBus::new();
        let mediator = TestMediator;
        let observer = TestObserver;

        // Register handlers to EventBus.
        event_bus.add_handler(|event: &TestEvent| {
            assert_eq!(event.0, 1);
            Ok(())
        });
        event_bus.add_handler(|event: &TestEvent2| {
            assert_eq!(event.0, 2);
            Ok(())
        });

        mediator
            .to_system()
            .register_to(HandlerRegistry { event_bus: &mut event_bus });

        observer
            .to_system()
            .register_to(HandlerRegistry { event_bus: &mut event_bus });

        // Send event to EventBus.
        let _ = event_bus.send(TestEvent(1));
    }
}
