use ::futures::prelude::*;

use crate::prelude::*;

#[derive(Debug)]
enum Q {
    One,
    Two,
}

#[derive(Debug)]
struct A;
struct B {
    id: usize,
}

#[async_trait::async_trait]
impl ActorHandler for A {
    type State = ();
    type Value = ();
    type Query = Q;
    type Error = std::convert::Infallible;

    async fn start(
        &mut self,
        ctx: &mut Context<Self::Query>,
    ) -> Result<StartHandled<Self::State, Self::Value>, Self::Error> {
        for id in 0..5 {
            let _ = ctx.child_run(Actor::create(B { id }), None).await;
        }

        Ok(StartHandled::Proceed(()))
    }

    async fn handle_query(
        &mut self,
        _state: &mut Self::State,
        _ctx: &mut Context<Self::Query>,
        query: Self::Query,
    ) -> Result<QueryHandled<Self::Value>, Self::Error> {
        println!("A handle_query: {:?}", query);
        Ok(QueryHandled::Continue)
    }

    async fn pre_stop(
        &mut self,
        _state: &mut Self::State,
        _ctx: &mut Context<Self::Query>,
        result: Result<&mut Self::Value, &ActorFailure<Self::Error>>,
    ) {
        println!("A pre_stop [result: {:?}]", result);
    }
    async fn post_stop(
        &mut self,
        _state: &mut Self::State,
        result: Result<&mut Self::Value, &ActorFailure<Self::Error>>,
    ) {
        println!("A post_stop [result: {:?}]", result);
    }
}

#[async_trait::async_trait]
impl ActorHandler for B {
    type State = usize;
    type Value = ();
    type Query = Q;
    type Error = std::convert::Infallible;

    async fn start(
        &mut self,
        _ctx: &mut Context<Self::Query>,
    ) -> Result<StartHandled<Self::State, Self::Value>, Self::Error> {
        Ok(StartHandled::Proceed(self.id))
    }

    async fn handle_query(
        &mut self,
        state: &mut Self::State,
        _ctx: &mut Context<Self::Query>,
        query: Self::Query,
    ) -> Result<QueryHandled<Self::Value>, Self::Error> {
        println!("B[{:?}] handle_query: {:?}", state, query);
        Ok(QueryHandled::Continue)
    }

    async fn pre_stop(
        &mut self,
        state: &mut Self::State,
        _ctx: &mut Context<Self::Query>,
        result: Result<&mut Self::Value, &ActorFailure<Self::Error>>,
    ) {
        println!("B[{:?}] pre_stop [result: {:?}]", state, result);
    }
    async fn post_stop(
        &mut self,
        state: &mut Self::State,
        result: Result<&mut Self::Value, &ActorFailure<Self::Error>>,
    ) {
        println!("B[{:?}] post_stop [result: {:?}]", state, result);
    }
}

#[tokio::test]
async fn test() {
    let a = A;
    let mut a = Actor::create(a);
    let mut api = a.api();

    let client = async move {
        let () = api.tell(Q::One).await;
        let () = api.tell(Q::Two).await;
        let () = api.shutdown(Default::default()).await;
    };

    let server = a.run();

    println!("{:?}", future::join(client, server).await);
}
