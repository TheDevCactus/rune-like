mod discord_client;
mod env_manager;

#[tokio::main]
async fn main() {
    env_manager::initialize_env().expect("Could not initialize environment");

    let mut dc = discord_client::DiscordClient::new();
    let messages = dc
        .get_authors_posts()
        .await
        .expect("Could not get messages");

    let mut handlebars = handlebars::Handlebars::new();
    handlebars
        .register_template_file("index", "./templates/index.handlebars")
        .expect("failed to register template");
    handlebars
        .register_partial(
            "post",
            std::fs::read_to_string("./templates/post.handlebars").unwrap(),
        )
        .expect("failed to register partial");
    handlebars
        .register_partial(
            "side_nav_item",
            std::fs::read_to_string("./templates/side_nav_item.handlebars").unwrap(),
        )
        .expect("failed to register partial");

    let render_result = handlebars.render(
        "index",
        serde_json::json!({ "posts": messages })
            .as_object()
            .unwrap(),
    );

    if let Err(err) = render_result {
        eprintln!("Could not render HTML Template: {:?}", err);
        return;
    }

    let index_file = render_result.unwrap();

    std::fs::write("./public/index.html", index_file).expect("Failed to write index file to disk.");
}
