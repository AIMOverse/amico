use anyhow::Result;
use evenio::event::EventMut;

use crate::ecs;

use super::System;

/// The Observer trait is used to observe events.
///
/// Observers watch for an event and process it, without
/// modifying the event and affecting the state of the world.
pub trait Observer {
    /// The event type that the Observer observes.
    type Event: ecs::GlobalEvent + 'static;

    /// The observe method is called when the Observer is notified of an event.
    ///
    /// The method receives a reference to the event and returns a Result.
    fn observe(&self, event: &<Self::Event as ecs::Event>::This<'_>) -> Result<()>;

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
    type Event: ecs::GlobalEvent + ecs::Event<Mutability = ecs::Mutable>;

    /// The event type that the Mediator sends.
    type EventsToSend: ecs::EventSet;

    /// The mediate method is called when the Mediator is notified of an event.
    ///
    /// The method receives a mutable reference to the event and a sender to send
    /// new events to the world.
    fn mediate(
        &self,
        event: &mut EventMut<'_, Self::Event>,
        sender: ecs::Sender<Self::EventsToSend>,
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
#[derive(ecs::GlobalEvent)]
pub struct PhantomEvent;

/// The placeholder observer for SystemHandler.
pub struct PhantomObserver;

/// The placeholder mediator for SystemHandler.
pub struct PhantomMediator;

impl Observer for PhantomObserver {
    type Event = PhantomEvent;

    fn observe(&self, _event: &<Self::Event as ecs::Event>::This<'_>) -> Result<()> {
        Ok(())
    }
}

impl Mediator for PhantomMediator {
    type Event = PhantomEvent;
    type EventsToSend = ();

    fn mediate(
        &self,
        _event: &mut EventMut<'_, Self::Event>,
        _sender: ecs::Sender<Self::EventsToSend>,
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
    O: Observer + 'static,
    M: Mediator + 'static,
{
    fn register_to(self, mut registry: crate::world::HandlerRegistry) {
        match self {
            SystemHandler::Observer(observer) => {
                registry.register(move |r: ecs::Receiver<O::Event>| {
                    if let Err(err) = observer.observe(r.event) {
                        tracing::error!("Error in observer: {}", err);
                    }
                })
            }
            SystemHandler::Mediator(mediator) => {
                registry.register(
                    move |mut r: ecs::ReceiverMut<M::Event>,
                          sender: ecs::Sender<M::EventsToSend>| {
                        if let Err(err) = mediator.mediate(&mut r.event, sender) {
                            tracing::error!("Error in mediator: {}", err);
                        }

                        // Take ownership of the event after handling it.
                        EventMut::take(r.event);
                    },
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use evenio::prelude::*;

    use crate::world::HandlerRegistry;

    use super::*;

    #[derive(ecs::GlobalEvent)]
    struct TestEvent(i32);

    #[derive(ecs::GlobalEvent)]
    struct TestEvent2(i32);

    struct TestObserver;

    impl Observer for TestObserver {
        type Event = TestEvent;

        fn observe(&self, event: &<Self::Event as ecs::Event>::This<'_>) -> Result<()> {
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
            event: &mut EventMut<'_, Self::Event>,
            mut sender: ecs::Sender<Self::EventsToSend>,
        ) -> Result<()> {
            let num = event.0;

            sender.send(TestEvent2(num * 2));

            Ok(())
        }
    }

    #[test]
    fn test_mediator() {
        // Initialize world.
        let mut world = World::new();
        let mediator = TestMediator;
        let observer = TestObserver;

        // Register handlers to World.
        world.add_handler(|r: ecs::Receiver<TestEvent>| {
            assert_eq!(r.event.0, 1);
        });
        world.add_handler(|r: ecs::Receiver<TestEvent2>| {
            assert_eq!(r.event.0, 2);
        });

        mediator
            .to_system()
            .register_to(HandlerRegistry { world: &mut world });

        observer
            .to_system()
            .register_to(HandlerRegistry { world: &mut world });

        // Send event to World.
        world.send(TestEvent(1));
    }
}
