

use  serde::{Serialize,Deserialize};



#[derive(Serialize,Deserialize,Clone,Debug,PartialEq)]
pub struct InstantiateMsgForHotel{
    pub total_rooms : u64,
    pub hotel_name : String,
    pub description : String,
    pub location_address : String,
    pub image_hash : String,
    pub hotel_city : String,
}


#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub enum HotelExecuteMsg {

        AddRoom{
       total_beds : u64,
       price_per_nigth : u64,
       room_number : u64,
        },
        BookedRoom{
            room_id : u64,
           amount_paid : u64,
           no_of_day_stay: u64,
           
        },
       CheckOutRoom{ room_id : u64},
    
}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]

pub enum HotelQuryMsg{

     GetAllRooms{},
     DisplayAvailableRoomForBooking{},
     DisplayAllBookedRoom{},
    

}

