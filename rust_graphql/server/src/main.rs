use futures::future;
use hyper::{
    rt::{self, Future},
    service::service_fn,
    Body, Method, Response, Server, StatusCode,
};
use juniper::FieldResult;
use juniper::{EmptyMutation, RootNode};
use mysql::params;
use std::sync::Arc;

#[derive(juniper::GraphQLObject)]
struct Article {
    id: i32,
    title: String,
    author: Option<String>,
}

struct Context {
    pool: mysql::Pool,
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
    fn article(context: &Context, id: i32) -> FieldResult<Option<Article>> {
        let query_result = context
            .pool
            .prep_exec(
                "SELECT n.nid, n.title, u.name FROM node_field_data n
                LEFT JOIN users_field_data u ON n.uid = u.uid
                WHERE n.nid = :nid",
                params! {"nid" => id},
            )
            .unwrap();
        for row in query_result {
            let (id, title, name) = mysql::from_row::<(i32, String, Option<String>)>(row.unwrap());
            let author = match name {
                Some(name) => match name.as_ref() {
                    "" => Some("Anonymous".to_owned()),
                    _ => Some(name),
                },
                None => None,
            };
            return Ok(Some(Article { id, title, author }));
        }
        Ok(None)
    }
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let pool = mysql::Pool::new("mysql://db:db@127.0.0.1:32768/db").unwrap();

    let context = Arc::new(Context { pool });
    let root_node = Arc::new(RootNode::new(Query, EmptyMutation::<Context>::new()));

    let new_service = move || {
        let root_node = root_node.clone();
        let context = context.clone();
        service_fn(move |req| -> Box<dyn Future<Item = _, Error = _> + Send> {
            let root_node = root_node.clone();
            let context = context.clone();
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => Box::new(juniper_hyper::graphiql("/graphql_example")),
                (&Method::GET, "/graphql_example") => {
                    Box::new(juniper_hyper::graphql(root_node, context, req))
                }
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
