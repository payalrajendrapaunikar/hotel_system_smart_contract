

use cosmwasm_std::{
    DepsMut,Env, Event, MessageInfo, Response, StdResult
};


use crate::booking_stae::BOOKING_ID;
use crate::hotel_state::{HotelItem, HOTEL, ROOM_COUNT};
use crate::hotel_msg::InstantiateMsgForHotel;
use crate::room_state::ADD_ROOM_COUNT;

pub fn instantiate_hotel_msg(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsgForHotel,
) -> StdResult<Response> {
   
    let hotel = HotelItem{
        total_rooms : msg.total_rooms.clone(),
        name : msg.hotel_name.clone(),
        description:msg.description,
        location_address : msg.location_address,
        image_hash : msg.image_hash,
        owner : info.sender,
        hotel_city:msg.hotel_city
    };

    HOTEL.save(deps.storage, &hotel)?;

    ROOM_COUNT.save(deps.storage, &msg.total_rooms)?;

    let added_room_count = 0;

    ADD_ROOM_COUNT.save(deps.storage, &added_room_count)?;

    let booking_id = 0;

    BOOKING_ID.save(deps.storage, &booking_id)?;

    let event = Event::new("hote_add")
         .add_attribute("hotel_name", msg.hotel_name);

        let resp = Response::new()
           .add_event(event);

        Ok(resp)

}