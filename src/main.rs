use actix::prelude::*;
use std::time::{Duration, Instant};

use actix_web::*;
use actix_web_actors::ws;

use actix_files as fs;
use tera::{Tera, Context};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct Ws {
    hb: Instant,
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl Ws {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");

                ctx.stop();

                return;
            }

            ctx.ping(b"");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(
        &mut self,msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
                println!("{:?}", &msg)
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => ctx.text(text),            
            _ => (),
        }
    }
}

async fn ws(req: HttpRequest,stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    let resp = ws::start(Ws::new(), &req, stream);
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

#[actix_web::main]
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
