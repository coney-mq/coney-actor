use ::futures::prelude::*;

use crate::prelude::*;

#[tokio::test]
async fn test() {
    #[derive(Debug)]
    enum Q {
        One,
        Two,
    }

    #[derive(Debug)]
    struct A {}

    impl A {
        pub fn create() -> Self {
            Self {}
        }
    }

    #[async_trait::async_trait]
    impl ActorHandler for A {
        type Value = ();
        type Query = Q;
        type Error = std::convert::Infallible;

        async fn handle_query(
            &mut self,
            _ctx: &mut ActorContext<Self::Query>,
            query: Self::Query,
        ) -> Result<QueryHandled<Self::Value>, Self::Error> {
            println!("query: {:?}", query);
            Ok(QueryHandled::Continue)
        }
    }

    let a = A::create();
    let a = Actor::create(a);
    let mut api = a.api();

    let client = async move {
        let () = api.tell(Q::One).await;
        let () = api.tell(Q::Two).await;
        let () = api.shutdown().await;
    };

    let server = a.run();

    println!("{:?}", future::join(client, server).await);
}
