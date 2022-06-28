use actix::{Actor, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;

use serde_json::{Value, json};


struct UgcWs;

impl Actor for UgcWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let hw = json!({
            "type": "hello"
        });
        ctx.text(hw.to_string());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for UgcWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let v: Value = serde_json::from_str(&text).unwrap();
                if v["type"] == "login" {
                    if v["data"]["token"] == "123" {
                        let data = json!({
                            "type": "login",
                            "data": {
                                "success": true,
                            }
                        });
                        ctx.text(data.to_string());
                    } else {
                        let data: Value = json!({
                            "type": "login",
                            "data": {
                                "success": false,
                            }
                        });
                        ctx.close(None);
                    }
                }
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(UgcWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}