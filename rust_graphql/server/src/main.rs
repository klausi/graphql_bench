use juniper::{FieldResult};
use hyper::{
    rt::{self, Future},
    service::service_fn,
    Body, Method, Response, Server, StatusCode,
};
use juniper::{
    EmptyMutation, RootNode,
};
use std::sync::Arc;
use futures::future;

#[derive(juniper::GraphQLObject)]
struct Article {
    id: i32,
    title: String,
    author: Option<String>,
}

struct Context {
    // Use your real database pool here.
    //pool: DatabasePool,
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

struct Query;

#[juniper::object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    Context = Context,
)]
impl Query {

    fn article(context: &Context, id: i32) -> FieldResult<Article> {
        Ok(Article {
            id: 1,
            title: "Test".to_owned(),
            author: None,
        })
    }
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let context = Arc::new(Context{});
    let root_node = Arc::new(RootNode::new(Query, EmptyMutation::<Context>::new()));

    let new_service = move || {
        let root_node = root_node.clone();
        let context = context.clone();
        service_fn(move |req| -> Box<dyn Future<Item = _, Error = _> + Send> {
            let root_node = root_node.clone();
            let context = context.clone();
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => Box::new(juniper_hyper::graphiql("/graphql_example")),
                (&Method::GET, "/graphql_example") => Box::new(juniper_hyper::graphql(root_node, context, req)),
                (&Method::POST, "/graphql_example") => {
                    Box::new(juniper_hyper::graphql(root_node, context, req))
                }
                _ => {
                    let mut response = Response::new(Body::empty());
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    Box::new(future::ok(response))
                }
            }
        })
    };
    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));
    println!("Listening on http://{}", addr);

    rt::run(server);
}
