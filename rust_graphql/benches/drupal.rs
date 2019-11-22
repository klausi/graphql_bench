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
    let q = Article::build_query(article::Variables { id: 23 });

    let client = reqwest::Client::new();

    b.iter(move || {
        let mut res = client
            .post("https://jobiqo8.ddev.site/graphql_example")
            .json(&q)
            .send()
            .unwrap();

        let response_body: Response<article::ResponseData> = res.json().unwrap();
        assert_eq!(23, response_body.data.unwrap().article.unwrap().id);
    });
}

fn html_drupal(b: &mut Bencher) {
    let client = reqwest::Client::new();

    b.iter(move || {
        let mut res = client
            .get("https://jobiqo8.ddev.site/node/23")
            .send()
            .unwrap();

        let text = res.text().unwrap();
        assert_eq!("<!DOCTYPE html", &text[0..14]);
    });
}

benchmark_group!(benches, graphql_drupal, html_drupal);
benchmark_main!(benches);
