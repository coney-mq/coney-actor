use coney_actor::prelude::*;

use crate::prelude::*;

#[test]
fn het_spec_01() {
    use crate::get_spec::GetSpec;

    use actor_a::ActorA;
    use actor_b::ActorB;
    use actor_c::ActorC;

    let spec = Spec::new(ActorA);
    let spec = spec.and(ActorB);
    let spec = spec.and(ActorC);

    let a: &Spec<ActorA> = spec.get();
    let b: &Spec<ActorB> = spec.get();
    let c: &Spec<ActorC> = spec.get();

    println!("a: {:?}; b: {:?}; c: {:?}", a, b, c);
}

mod actor_a {
    use coney_actor::prelude::*;

    #[derive(Debug)]
    pub struct ActorA;

    #[async_trait::async_trait]
    impl ActorHandler for ActorA {
        type State = ();
        type Query = ();
        type Value = ();
        type Error = std::convert::Infallible;

        async fn start(
            &mut self,
            _ctx: &mut Context<Self::Query>,
        ) -> Result<StartHandled<Self::State, Self::Value>, Self::Error> {
            println!("start");
            Ok(StartHandled::Proceed(()))
        }

        async fn handle_query(
            &mut self,
            _state: &mut Self::State,
            _ctx: &mut Context<Self::Query>,
            query: Self::Query,
        ) -> Result<QueryHandled<Self::Value>, Self::Error> {
            println!("handle_query [q: {:?}]", query);
            Ok(QueryHandled::Continue)
        }
    }
}

mod actor_b {
    use coney_actor::prelude::*;

    #[derive(Debug)]
    pub struct ActorB;

    #[async_trait::async_trait]
    impl ActorHandler for ActorB {
        type State = ();
        type Query = ();
        type Value = ();
        type Error = std::convert::Infallible;

        async fn start(
            &mut self,
            _ctx: &mut Context<Self::Query>,
        ) -> Result<StartHandled<Self::State, Self::Value>, Self::Error> {
            println!("start");
            Ok(StartHandled::Proceed(()))
        }

        async fn handle_query(
            &mut self,
            _state: &mut Self::State,
            _ctx: &mut Context<Self::Query>,
            query: Self::Query,
        ) -> Result<QueryHandled<Self::Value>, Self::Error> {
            println!("handle_query [q: {:?}]", query);
            Ok(QueryHandled::Continue)
        }
    }
}

mod actor_c {
    use coney_actor::prelude::*;

    #[derive(Debug)]
    pub struct ActorC;

    #[async_trait::async_trait]
    impl ActorHandler for ActorC {
        type State = ();
        type Query = ();
        type Value = ();
        type Error = std::convert::Infallible;

        async fn start(
            &mut self,
            _ctx: &mut Context<Self::Query>,
        ) -> Result<StartHandled<Self::State, Self::Value>, Self::Error> {
            println!("start");
            Ok(StartHandled::Proceed(()))
        }

        async fn handle_query(
            &mut self,
            _state: &mut Self::State,
            _ctx: &mut Context<Self::Query>,
            query: Self::Query,
        ) -> Result<QueryHandled<Self::Value>, Self::Error> {
            println!("handle_query [q: {:?}]", query);
            Ok(QueryHandled::Continue)
        }
    }
}
