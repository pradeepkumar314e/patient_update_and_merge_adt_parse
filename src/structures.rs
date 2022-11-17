//use std::collections::HashMap;
use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize, PartialEq, Default,Clone)]
pub struct ParMsg {
    pub msgtype:String,
    pub components:Vec<MsgTypeField>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Default,Clone)]
pub struct MsgTypeField{
    pub segmentheader:String,
    pub values:Vec<(String,String)>,
}


#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Msg {
    pub segments:  Vec<Segment>,
    pub separator: char,
}


#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Segment {
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq, Default,Deserialize, Serialize)]
pub struct Field {
    pub components: Vec<String>,
}

