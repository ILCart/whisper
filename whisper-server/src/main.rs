mod events;
extern crate env_logger;
extern crate ws;

use ws::{listen, CloseCode, Handler, Handshake, Message, Request, Response, Result, Sender};
use uuid::Uuid;
use serde_json;
use serde::{Serialize, Deserialize};

struct Server {
    out: Sender,
    clients: Vec<Client>,
    created_ids: Vec<Uuid>,
}

struct Client{
    uuid: Uuid,
}




impl Server{
    fn create_uuid(&mut self) -> Uuid{
        let mut new_uuid = Uuid::new_v4();
        while self.created_ids.contains(&new_uuid) {
            new_uuid = Uuid::new_v4();
        }
        self.created_ids.push(new_uuid);
        new_uuid
    }
    // fn is_uuid_dead(&self,needle: &Uuid) -> bool{
    //     self.dead_ids.contains(needle)
    // }
    // fn kill_uuid(&self,needle: &Uuid){
    //     match self.ids.contains(needle) {
    //         true => self.ids.retain(|&x| x != *needle),
    //         false => ()
    //     }
    // }
}
impl Handler for Server{
    fn on_request(&mut self, req: &Request) -> Result<Response> {
        match req.resource() {
            "/ws" => Response::from_request(req),
            "/" => Ok(Response::new(200, "OK", b"<p>Gyat</p>".to_vec())),
            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }
    
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        let id: Uuid = self.create_uuid();
        let connected_client: Client = Client {uuid:id};
        self.clients.push(connected_client);
        println!("Connection opened to client: {}, Their UUID is : {}",shake.peer_addr.unwrap(),id);
        self.out.send(id.to_string())
    }
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Connection closed to client. Closed with code: '{0:#?}' Reason : '{1}'",code,reason);
    }
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Client sent message: {:#?}",msg);
        self.out.send("received")
    }
}



fn main(){
    env_logger::init();
    let port = "3001";
    let mut addr = "127.0.0.1:".to_owned();
    addr.push_str(port);
    listen(addr, |out| Server {out, clients: Vec::new(), created_ids: Vec::new() }).unwrap();
}

