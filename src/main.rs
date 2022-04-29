use askama::Template;
use async_std::task::block_on;
use tide::{Request, Response, Result};

#[derive(Template, Debug)]
#[template(path = "index.html")]
struct IndexPage {
    name: String,
}

async fn index_page(req: Request<()>) -> Result<Response> {
    let res: Response = IndexPage {
        name: req.param("name").unwrap_or("").to_owned(),
    }
    .into();
    Ok(res)
}

fn main() {
    dotenv::dotenv().ok();

    let mut server = tide::with_state(());

    server.at("/").get(index_page);
    server.at("/:name").get(index_page);

    block_on(server.listen("0.0.0.0:8080")).unwrap();
}
