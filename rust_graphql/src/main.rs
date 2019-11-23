use graphql_client::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug"
)]
struct Article;

fn main() -> Result<(), failure::Error> {
    let q = Article::build_query(article::Variables { id: 1 });

    let client = reqwest::Client::new();

    let mut res = client
        .post("https://drupal-graphql.ddev.site/graphql_example")
        .json(&q)
        .send()?;

    dbg!(&res);
    let response_body: Response<article::ResponseData> = res.json()?;
    dbg!(response_body);

    let text = client
        .get("https://drupal-graphql.ddev.site/node/1")
        .send()?
        .text()?;

    dbg!(text);

    Ok(())
}
