

use cw_storage_plus::Item;
use serde::{Serialize,Deserialize};
use cosmwasm_std::{Addr, Timestamp};

#[derive(Serialize,Deserialize,Debug,PartialEq,Clone)]
pub struct BookingItem{

     pub booking_id : u64,
     pub room_id : u64,
     pub booking_date : Timestamp,
     pub amount_paid : u64,
     pub no_of_day_stay: u64,
     pub  tenant : Addr,


}


pub const BOOKING_STATE : Item<Vec<BookingItem>> = Item::new("booking");
pub const BOOKING_ID : Item<u64> = Item::new("booking_id"); 