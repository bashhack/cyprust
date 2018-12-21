extern crate chrono;
extern crate env_logger;
extern crate iron;
extern crate logger;
extern crate router;
extern crate serde;
extern crate uuid;

#[macro_use]
extern crate serde_derive;

mod database;
mod handlers;
mod models;

use iron::prelude::Chain;
use iron::Iron;
use logger::Logger;
use router::Router;
use uuid::Uuid;

use crate::database::Database;
use crate::handlers::*;
use crate::models::*;

fn main() {
    env_logger::init();
    let (logger_before, logger_after) = Logger::new(None);

    let mut db = Database::new();
    let p1 = Post::new(
        "The first post",
        "This is the first post in our API",
        "Bashhack",
        chrono::offset::Utc::now(),
        Uuid::new_v4(),
    );
    db.add_post(p1);
    let p2 = Post::new(
        "The second post",
        "This is the second post in our API",
        "Geddy",
        chrono::offset::Utc::now(),
        Uuid::new_v4(),
    );
    db.add_post(p2);

    let handlers = Handlers::new(db);
    let json_content_middleware = JsonAfterMiddleware;

    let mut router = Router::new();
    router.get("/post_feed", handlers.post_feed, "post_feed");
    router.post("/post", handlers.post_post, "post_post");
    router.get("/post/:id", handlers.post, "post");

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(json_content_middleware);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:8000").unwrap();
}
