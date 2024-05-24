
use serde_json::json;
use uuid::Uuid;
use ws::Sender;

pub fn send_message(uuid:Uuid,payload:String,out:Sender){
    println!("{} sent {payload}",uuid.to_string());
    let send_out = json!({"uuid":uuid.to_string(),"data":{"event":"RECEIVE_MESSAGE","payload":payload}});
    match out.broadcast(send_out.to_string()) {
        Ok(_) => println!("message broadcasted"),
        Err(er) => eprintln!("BROADCAST FAILED {er}")
    }
}

// pub fn recive_message(uuid:Uuid,payload:String){
    
// }