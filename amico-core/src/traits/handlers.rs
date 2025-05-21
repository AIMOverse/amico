use anyhow::Result;
use evenio::event::EventMut;

use crate::ecs;

use super::System;

#[derive(ecs::GlobalEvent)]
pub struct PhantomEvent;

pub enum SystemHandler<
    E: ecs::GlobalEvent + 'static = PhantomEvent,
    M: ecs::GlobalEvent + ecs::Event<Mutability = ecs::Mutable> + 'static = PhantomEvent,
    S: ecs::EventSet + 'static = (),
> {
    Observer(Box<dyn Observer<Event = E> + 'static>),
    Mediator(Box<dyn Mediator<Event = M, EventsToSend = S> + 'static>),
}

impl<E, M, S> SystemHandler<E, M, S>
where
    E: ecs::GlobalEvent + 'static,
    M: ecs::GlobalEvent + ecs::Event<Mutability = ecs::Mutable> + 'static,
    S: ecs::EventSet + 'static,
{
    pub fn from_observer<O: Observer<Event = E> + 'static>(observer: O) -> Self {
        SystemHandler::Observer(Box::new(observer))
    }

    pub fn from_mediator<Med: Mediator<Event = M, EventsToSend = S> + 'static>(
        mediator: Med,
    ) -> Self {
        SystemHandler::Mediator(Box::new(mediator))
    }
}

pub trait Observer {
    type Event: ecs::GlobalEvent + 'static;

    fn observe(&self, event: &<Self::Event as ecs::Event>::This<'_>) -> Result<()>;

    fn to_system(self) -> SystemHandler<Self::Event, PhantomEvent, ()>
    where
        Self: Sized + 'static,
    {
        SystemHandler::from_observer(self)
    }
}

pub trait Mediator {
    type Event: ecs::GlobalEvent + ecs::Event<Mutability = ecs::Mutable>;
    type EventsToSend: ecs::EventSet;

    fn mediate(
        &self,
        event: &mut EventMut<'_, Self::Event>,
        sender: ecs::Sender<Self::EventsToSend>,
    ) -> Result<()>;

    fn to_system(self) -> SystemHandler<PhantomEvent, Self::Event, Self::EventsToSend>
    where
        Self: Sized + 'static,
    {
        SystemHandler::from_mediator(self)
    }
}

impl<E, M, S> System for SystemHandler<E, M, S>
where
    E: ecs::GlobalEvent + 'static,
    M: ecs::GlobalEvent + ecs::Event<Mutability = ecs::Mutable> + 'static,
    S: ecs::EventSet + 'static,
{
    fn register_to(self, mut registry: crate::world::HandlerRegistry) {
        match self {
            SystemHandler::Observer(observer) => registry.register(move |r: ecs::Receiver<E>| {
                if let Err(err) = observer.observe(&r.event) {
                    tracing::error!("Error in observer: {}", err);
                }
            }),
            SystemHandler::Mediator(mediator) => {
                registry.register(move |mut r: ecs::ReceiverMut<M>, sender: ecs::Sender<S>| {
                    if let Err(err) = mediator.mediate(&mut r.event, sender) {
                        tracing::error!("Error in mediator: {}", err);
                    }

                    // Take ownership of the event after handling it.
                    EventMut::take(r.event);
                })
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

    // #[test]
    // fn test_into_system() {
    //     // Compile-time tests
    //     let _: Box<dyn System> = Box::new(TestMediator);
    //     let _: Box<dyn System> = Box::new(TestObserver);
    // }

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
