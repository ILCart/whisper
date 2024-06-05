
use serde_json::json;
use uuid::Uuid;
use ws::Sender;

#[derive(Eq, PartialEq, Debug)]
pub struct UserMessage{
    data:String,
    user_id:Uuid,
    message_id:Uuid
}

pub fn send_message(uuid:Uuid,payload:String,out:Sender) -> UserMessage{
    println!("{} sent {payload}",uuid.to_string());
    let msg_id = Uuid::new_v4();
    let send_out = json!({"uuid":uuid.to_string(),"data":{"event":"RECEIVE_MESSAGE","msgid":msg_id.to_string(),"payload":payload}});
    match out.broadcast(send_out.to_string()) {
        Ok(_) => println!("message broadcasted"),
        Err(er) => eprintln!("BROADCAST FAILED {er}")
    }
    UserMessage {data: payload, user_id:uuid,message_id:msg_id}
}

pub fn reply_to_message(sender_uuid:Uuid,reply_uuid:Uuid,payload:String,out:Sender){
    println!("{} replyed to {} sent {payload}",sender_uuid.to_string(),reply_uuid.to_string());
    let send_out = json!({"uuid":sender_uuid.to_string(),"data":{"event":"REPLY_MESSAGE","payload":payload,"reply_uuid":reply_uuid.to_string()}});
    match out.broadcast(send_out.to_string()) {
        Ok(_) => println!("message broadcasted"),
        Err(er) => eprintln!("BROADCAST FAILED {er}")
    }
}


// pub fn recive_message(uuid:Uuid,payload:String){
    
// }