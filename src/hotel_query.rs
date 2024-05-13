use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

use crate::{booking_query_impl, hotel_msg::HotelQuryMsg};





pub fn hotel_qury_msg(
    deps:Deps,
    _env:Env,
    msg: HotelQuryMsg
)-> StdResult<Binary>{

    use HotelQuryMsg::*;

    match msg {

       GetAllRooms {  }  => to_json_binary(&hotel_qury_msg_impl::get_all_rooms(deps)?),
       DisplayAvailableRoomForBooking {  } => to_json_binary(&booking_query_impl::show_available_room(deps)?),
       DisplayAllBookedRoom {  } => to_json_binary(&hotel_qury_msg_impl::dispaly_all_booked_room(deps)?)
        
    }

}


mod hotel_qury_msg_impl{
    
       use crate::{booking_stae::{BookingItem, BOOKING_STATE}, room_state::{RoomItem, ROOMS}};


    use super::*;

    pub fn get_all_rooms(deps:Deps)->StdResult<Vec<RoomItem>>{

        let rooms = ROOMS.load(deps.storage)?;

        Ok(rooms)
    }


    pub fn dispaly_all_booked_room(deps:Deps)->StdResult<Vec<BookingItem>>{

        let booked_room = BOOKING_STATE.load(deps.storage)?;

        Ok(booked_room)

    }
}