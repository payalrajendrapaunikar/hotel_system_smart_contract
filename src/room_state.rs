





use serde::{Deserialize,Serialize};
use cw_storage_plus::Item;

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct RoomItem{
    pub room_id : u64,
    pub total_beds : u64,
    pub price_per_nigth : u64,
    pub is_booked : bool,
    pub room_number : u64,
                                                                      
}


pub const ROOMS : Item<Vec<RoomItem>> = Item::new("rooms");
pub const ADD_ROOM_COUNT : Item<u64> = Item::new("added_room_count");
