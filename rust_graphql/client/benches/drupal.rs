use bencher::{benchmark_group, benchmark_main, Bencher};
use graphql_client::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug"
)]
struct Article;

fn graphql_drupal(b: &mut Bencher) {
    let q = Article::build_query(article::Variables { id: 1 });

    let client = reqwest::Client::new();

    b.iter(move || {
        let mut res = client
            .post("https://drupal-graphql.ddev.site/graphql_example")
            .json(&q)
            .send()
            .unwrap();

        let response_body: Response<article::ResponseData> = res.json().unwrap();
        assert_eq!(1, response_body.data.unwrap().article.unwrap().id);
    });
}

fn html_drupal(b: &mut Bencher) {
    let client = reqwest::Client::new();

    b.iter(move || {
        let mut res = client
            .get("https://drupal-graphql.ddev.site/node/1")
            .send()
            .unwrap();

        let text = res.text().unwrap();
        assert_eq!("<!DOCTYPE html", &text[0..14]);
    });
}

fn jsonapi_drupal(b: &mut Bencher) {
    let client = reqwest::Client::new();

    b.iter(move || {
        let json: serde_json::Value = client
        .get("https://drupal-graphql.ddev.site/jsonapi/node/article/6997b22e-36ef-4d6e-9683-af23f4e7f137")
        .send().unwrap()
        .json().unwrap();

        assert_eq!("6997b22e-36ef-4d6e-9683-af23f4e7f137", json["data"]["id"]);
    });
}

fn graphql_rust(b: &mut Bencher) {
    let q = Article::build_query(article::Variables { id: 1 });

    let client = reqwest::Client::new();

    b.iter(move || {
        let mut res = client
            .post("http://localhost:3000/graphql_example")
            .json(&q)
            .send()
            .unwrap();

        let response_body: Response<article::ResponseData> = res.json().unwrap();
        assert_eq!(1, response_body.data.unwrap().article.unwrap().id);
    });
}

benchmark_group!(
    benches,
    graphql_drupal,
    html_drupal,
    jsonapi_drupal,
    graphql_rust
);
benchmark_main!(benches);
