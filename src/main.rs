use actix::{Actor, StreamHandler};

use actix_web::*;
use actix_web_actors::ws;

use actix_files as fs;
use tera::{Tera, Context};



struct Ws;

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(
        &mut self,msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            _ => (),
        }
    }
}

async fn ws(req: HttpRequest,stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(Ws {}, &req, stream);
    println!("{:?}", resp);
    resp
}

async fn index() -> impl Responder {
    let tera =
        Tera::new(
            concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")
        ).unwrap();

    let ctx = Context::new();
    let rendered = tera.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()

        .service(
            fs::Files::new("/static", "./static")
           .show_files_listing()
        )

        .service(
            web::resource("/ws/").to(ws)
        )
        .service(
            web::resource("/").to(index)
        )
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
