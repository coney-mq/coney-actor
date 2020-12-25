use ::anyhow::Error as AnyError;
use ::futures::prelude::*;

use crate::prelude::*;

#[tokio::test]
async fn test() {
    use an_actor::*;

    let mut actor = AnActor.into_runner();
    let mut api = actor.api();
    let actor_running = actor.run().map_err(AnyError::from);

    let client_running = async move {
        api.shutdown(Default::default())
            .map_err(AnyError::from)
            .await
    };

    future::try_join(actor_running, client_running)
        .await
        .unwrap();
}

mod an_actor {
    use crate::prelude::*;

    #[derive(Debug)]
    pub struct AnActor;

    #[derive(Debug)]
    pub enum Query {}

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
            println!("AnActor::init [self: {:?}, ctx: {:?}]", self, context);
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
