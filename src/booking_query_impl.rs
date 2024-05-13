

use cosmwasm_std::{Deps,StdResult,StdError};

use crate::room_state::{ROOMS,RoomItem};



pub fn show_available_room(deps:Deps)->StdResult<Vec<RoomItem>>{

    let mut available_rooms  : Vec<RoomItem> = Vec::new();

     let not_booked_room = is_rooms_available(deps);

   match not_booked_room {
    Ok(rooms)=> available_rooms = rooms,
    Err(_)=> return  Err(StdError::generic_err("all room is booked")),
       
   }

   Ok(available_rooms)


    }


pub fn is_rooms_available(deps:Deps)-> StdResult<Vec<RoomItem>>{


    let rooms = ROOMS.load(deps.storage)?;

    let mut rooms_isnot_boot_booked: Vec<RoomItem> = Vec::new();

    for get_room in rooms {

        if  get_room.is_booked == false {
            rooms_isnot_boot_booked.push(get_room)
        }


        
    }

    Ok(rooms_isnot_boot_booked)

}





  

