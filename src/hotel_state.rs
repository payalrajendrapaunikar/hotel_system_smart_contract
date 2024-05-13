


use cosmwasm_std::Addr;
use serde::{Serialize,Deserialize};
use cw_storage_plus::Item;


//Hotel data stuructures
#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
pub struct HotelItem{
    pub total_rooms : u64,
    pub name : String,
    pub description : String,
    pub location_address : String,
    pub image_hash : String,
    pub owner : Addr,
    pub hotel_city : String,

}


pub const  HOTEL : Item<HotelItem> = Item::new("hotels");
pub const  ROOM_COUNT : Item<u64> = Item::new("rooms_count");

