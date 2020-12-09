use std::time::Duration;

use ::futures::prelude::*;

use crate::prelude::*;

#[derive(Debug)]
enum Q {
    One,
    Two,
}

#[derive(Debug)]
struct A;
struct B;

#[async_trait::async_trait]
impl ActorHandler for A {
    type Value = ();
    type Query = Q;
    type Error = std::convert::Infallible;

    async fn on_start(
        &mut self,
        ctx: &mut Context<Self::Query>,
    ) -> Result<StartHandled<Self::Value>, Self::Error> {
        let _ = ctx.child_run(Actor::create(B), None).await;

        Ok(StartHandled::Proceed)
    }

    async fn handle_query(
        &mut self,
        _ctx: &mut Context<Self::Query>,
        query: Self::Query,
    ) -> Result<QueryHandled<Self::Value>, Self::Error> {
        println!("A handle_query: {:?}", query);
        Ok(QueryHandled::Continue)
    }

    async fn on_failure(
        &mut self,
        _ctx: &mut Context<Self::Query>,
        reason: &ActorFailure<Self::Error>,
    ) {
        println!("A on_failure: {}", reason);
    }
}

#[async_trait::async_trait]
impl ActorHandler for B {
    type Value = ();
    type Query = Q;
    type Error = std::convert::Infallible;

    async fn handle_query(
        &mut self,
        _ctx: &mut Context<Self::Query>,
        query: Self::Query,
    ) -> Result<QueryHandled<Self::Value>, Self::Error> {
        println!("B handle_query: {:?}", query);
        Ok(QueryHandled::Continue)
    }

    async fn on_failure(
        &mut self,
        _ctx: &mut Context<Self::Query>,
        reason: &ActorFailure<Self::Error>,
    ) {
        println!("B on_failure: {}", reason);
    }
}

#[tokio::test]
async fn test() {
    let a = A;
    let a = Actor::create(a);
    let mut api = a.api();

    let client = async move {
        let () = api.tell(Q::One).await;
        let () = api.tell(Q::Two).await;
        let () = api.shutdown().await;
    };

    let server = a.run();

    println!("{:?}", future::join(client, server).await);
    let () = ::tokio::time::sleep(Duration::from_secs(1)).await;
}
