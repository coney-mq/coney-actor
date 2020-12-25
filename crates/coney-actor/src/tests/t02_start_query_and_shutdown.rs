use std::time::Duration;

use ::anyhow::Error as AnyError;
use ::futures::prelude::*;

use crate::prelude::*;
use an_actor::*;

#[tokio::test]
async fn test() {
    let mut actor = AnActor.into_runner();
    let api = actor.api();
    let actor_running = actor.run().map_err(AnyError::from);
    let client_running = run_client(api);

    future::try_join(actor_running, client_running)
        .await
        .unwrap();
}

async fn run_client(mut api: ActorApi<an_actor::Query>) -> Result<(), AnyError> {
    api.tell(Query::One).await?;
    api.tell(Query::Two).await?;
    api.tell(Query::One).await?;
    api.tell(Query::Two).await?;

    let () = ::tokio::time::sleep(Duration::from_millis(100)).await;
    api.shutdown(Default::default()).await?;
    Ok(())
}

mod an_actor {
    use crate::prelude::*;

    #[derive(Debug)]
    pub struct AnActor;

    #[derive(Debug)]
    pub enum Query {
        One,
        Two,
    }

    #[derive(Debug)]
    pub struct State {}

    #[derive(Debug, Error)]
    pub enum Error {}

    #[async_trait::async_trait]
    impl Actor for AnActor {
        type Query = Query;
        type State = State;
        type Error = Error;

        async fn init<Ctx: Context>(
            &mut self,
            context: &mut Ctx,
        ) -> Result<Self::State, Self::Error> {
            println!("AnActor::init [ctx: {:?}]", context);
            Ok(State {})
        }

        async fn handle_query<Ctx: Context>(
            state: &mut Self::State,
            context: &mut Ctx,
            query: Self::Query,
        ) -> Result<(), Self::Error> {
            println!(
                "AnActor::handle_query [state: {:?}, ctx: {:?}, query: {:?}]",
                state, context, query
            );
            Ok(())
        }

        async fn shutdown<Ctx: Context>(
            &mut self,
            state: Self::State,
            context: &mut Ctx,
            failure: Option<&Self::Error>,
        ) {
            println!(
                "AnActor::shutdown [self: {:?}, state: {:?}, ctx: {:?}, failure: {:?}]",
                self, state, context, failure
            );
        }
    }
}
