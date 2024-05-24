
use uuid::Uuid;
use ws::Sender;

pub fn send_message(uuid:Uuid,payload:String,out:Sender){
    println!("{} sent {payload}",uuid.to_string());
    out.broadcast(format!("uuid:{},payload:{}",uuid.to_string(),payload));
}

// pub fn recive_message(uuid:Uuid,payload:String){
    
// }