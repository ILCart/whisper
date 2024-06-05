mod events;
extern crate env_logger;
extern crate ws;

use std::collections::{hash_set, HashSet};

use events::{reply_to_message, send_message};
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use uuid::Uuid;
use ws::{
    listen, CloseCode, Handler, Handshake, Message, Request, Response, Result as ws_result, Sender,
};


#[derive(Eq, Hash, PartialEq, Debug)]
struct UserMessage{
    data:String,
    user_id:Uuid,
    message_id:Uuid
}

struct Server {
    out: Sender,
    clients: HashSet<Client>,
    created_ids: Vec<Uuid>,
    messages: HashSet<UserMessage>
}
#[derive(Eq, Hash, PartialEq)]
struct Client {
    uuid: Uuid,
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientData {
    event: String,
    reply_uuid: String,
    payload: String
}

#[derive(Serialize, Deserialize)]
struct ClientMessage {
    data: ClientData,
    uuid: String,
}

impl Server {
    fn create_uuid(&mut self) -> Uuid {
        let mut new_uuid = Uuid::new_v4();
        while self.created_ids.contains(&new_uuid) {
            new_uuid = Uuid::new_v4();
        }
        self.created_ids.push(new_uuid);
        new_uuid
    }
    fn create_message_uuid(&mut self) -> Uuid{
        let mut new_uuid = Uuid::new_v4();
        while self.messages.iter().any(|msg| msg.message_id == new_uuid) {
            new_uuid = Uuid::new_v4();
        }
        self.created_ids.push(new_uuid);
        new_uuid
    }
    fn uuid_is_client(&self, uuid: Uuid) -> bool {
        let client_index = self.clients.iter().find(|x| x.uuid == uuid);
        match client_index {
            Some(client) => self.clients.contains(client),
            None => false
        }
    }
    fn get_client_from_uuid(&self,s: Uuid) -> &Client {
        self.clients.iter().find(|p| p.uuid == s).expect("Expected Valid UUID (eval before this)")
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

fn is_uuid(s: &str) -> Result<Uuid, String> {
    match Uuid::parse_str(s) {
        Ok(f) => Ok(f),
        Err(_e) => Err("NOT A UUID".to_string()),
    }
}

fn match_event(client:&Client,message:&ClientMessage,out:Sender) {
    match message.data.event.as_str() {
        "SEND_MESSAGE" => send_message(client.uuid,message.data.payload.to_owned(),out,),
        "REPLY_MESSAGE" => {
            match is_uuid(&message.data.reply_uuid)  {
                Ok(uuid_reply) => reply_to_message(client.uuid,uuid_reply,message.data.payload.to_owned(),out),
                Err(er) => eprintln!("{er}")
            }
        }

        // "RECEIVE_MESSAGE" => recive_message(client.uuid,message.data.payload.to_owned()),

        _ => eprintln!("NO EVENT")
    }
}

impl Handler for Server {
    fn on_request(&mut self, req: &Request) -> ws_result<Response> {
        match req.resource() {
            "/ws" => Response::from_request(req),
            "/" => Ok(Response::new(200, "OK", b"<p>Gyat</p>".to_vec())),
            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    fn on_open(&mut self, shake: Handshake) -> ws_result<()> {
        let id: Uuid = self.create_uuid();
        let connected_client: Client = Client {
            uuid: id,
            username: "Test".to_owned(),
        };
        self.clients.insert(connected_client);
        println!(
            "Connection opened to client: {}, Their UUID is : {}",
            shake.peer_addr.unwrap(),
            id
        );
        let send_back = json!({"uuid":id.to_string()});
        self.out.send(send_back.to_string())
    }
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!(
            "Connection closed to client. Closed with code: '{0:#?}' Reason : '{1}'",
            code, reason
        );
    }
    fn on_message(&mut self, msg: Message) -> ws_result<()> {
        println!("Client sent message: {:#?}", msg);
        let message: ClientMessage = serde_json::from_str(msg.as_text().expect("Expected text"))
            .expect("Expected 'uuid' and 'data'");
  
        let parsed_uuid = is_uuid(&message.uuid) ;
        match parsed_uuid {
            Ok(uuid) => {
                if self.uuid_is_client(uuid){
                    let client = self.get_client_from_uuid(uuid);
                    
                    match_event(client,&message,self.out.to_owned());
                    println!("{} ({}): {:#?}", client.username, client.uuid, &message.data);
                }
                let send_out = json!({"uuid":uuid.to_string(),"data":{"event":"STATUS","payload":"204"}});

                self.out.send(send_out.to_string())
            }
            Err(msg) => {
                eprintln!("{msg} ({})", &message.uuid);
                let send_out = json!({"uuid":message.uuid.to_string(),"data":{"event":"STATUS","payload":"401"}});
                self.out.send(send_out.to_string())

            }
        }
    }
}

fn main() {
    env_logger::init();
    let port = "3001";
    let mut addr = "127.0.0.1:".to_owned();
    addr.push_str(port);
    listen(addr, |out| Server {
        out,
        clients: HashSet::new(),
        created_ids: Vec::new(),
        messages: HashSet::new()
    })
    .unwrap();
}
