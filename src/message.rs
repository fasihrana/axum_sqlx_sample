use crate::dbpool;
use uuid;

pub struct Message{
    id: uuid::Uuid,
    message: String
}

impl Message {
    pub fn new(val:String) -> Self {
        Message{
            id: uuid::Builder::nil().into_uuid(),
            message: val,
        }
    }

    pub fn id(&mut self, val:uuid::Uuid){
        self.id = val;
    }

    pub fn message(&mut self, val:String){
        self.message = val;
    }
}

impl dbpool::DBPool {
    pub async fn add_message(&self,message: String) -> Result<uuid::Uuid, dbpool::ModError>{
        //the following line is causing issues
        let conn = self.pool();

        //TODO:insert the message into DB and return the uuid of the message

        return Ok(uuid::Builder::nil().into_uuid());
    }
}
