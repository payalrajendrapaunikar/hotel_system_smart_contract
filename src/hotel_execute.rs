

use cosmwasm_std::{DepsMut, Env,  MessageInfo, Response, StdError, StdResult};

use crate::{hotel_msg::HotelExecuteMsg, room_state::RoomItem};



pub fn execute_hotel_msg(

    deps : DepsMut,
    env:Env,
    info:MessageInfo,
    msg: HotelExecuteMsg
)-> Result<Response,StdError>{

     use HotelExecuteMsg::*;
      
     match msg {

        AddRoom { total_beds , price_per_nigth, room_number }
        => excute_hotel_msg_impl::add_room(deps, info, total_beds, price_per_nigth, room_number),
        BookedRoom { room_id, amount_paid, no_of_day_stay }
         => excute_hotel_msg_impl::book_room(deps, info, room_id, env, amount_paid, no_of_day_stay),
         CheckOutRoom { room_id }
         => excute_hotel_msg_impl::ckeck_out_room(room_id, deps, info)
     }

      

}


mod excute_hotel_msg_impl{
    


    use cosmwasm_std::Event;

    use crate::{booking_stae::{BookingItem, BOOKING_ID, BOOKING_STATE}, hotel_state::{HOTEL, ROOM_COUNT}, room_state::{RoomItem, ADD_ROOM_COUNT, ROOMS}};

    use super::*;


   pub fn add_room(
        deps : DepsMut,
        info:MessageInfo,
        total_beds : u64,
        price_per_nigth : u64,
        room_number : u64,
    )-> Result<Response,StdError> {


        let hotel = HOTEL.load(deps.storage)?;

        if info.sender != hotel.owner {

         return  Err(StdError::generic_err("Owner is unauthorized user!"));
        }

        let room_count = ADD_ROOM_COUNT.load(deps.storage)?;

        if room_count == 0 {

            let new_room = RoomItem{
                room_id : 1,
                total_beds,
                price_per_nigth,
                is_booked : false,
                room_number
            };

            let rooms : Vec<RoomItem> = vec![new_room.clone()];

            ROOMS.save(deps.storage, &rooms)?;

             ADD_ROOM_COUNT.save(deps.storage, &new_room.room_id)?;

            let add_room_count = ADD_ROOM_COUNT.load(deps.storage)?;

            let event = Event::new("room_add")
                .add_attribute("room_count",&add_room_count.to_string());

            let resp = Response::new()
                   .add_event(event);

            return  Ok(resp);
        }


        let mut rooms = ROOMS.load(deps.storage)?;

        let add_room_count = ADD_ROOM_COUNT.load(deps.storage)?;

        let hotel_room_count = ROOM_COUNT.load(deps.storage)?;

         if add_room_count + 1 > hotel_room_count {

            return  Err(StdError::generic_err("room can not be added because hotel doesn't have this no of room in their hotel"));
         }

         let new_room = RoomItem{
            room_id : add_room_count + 1 ,
            total_beds,
            price_per_nigth,
            is_booked : false,
            room_number
         };

         rooms.push(new_room.clone());

         ROOMS.save(deps.storage, &rooms)?;

         let event = Event::new("add_another_room")
              .add_attribute("room_id", &new_room.room_id.to_string());

            let resp = Response::new()
                  .add_event(event);

                return Ok(resp);


    }



    pub fn book_room(
        deps : DepsMut,
        info:MessageInfo,
        room_id : u64,
        env : Env,
        amount_paid : u64,
        no_of_day_stay: u64,
         )-> Result<Response,StdError>{

        
         let mut rooms = ROOMS.load(deps.storage)?;

         let room = get_room_details_by_id(&rooms, room_id)?;

         let room_is_booked = check_room_is_book_using_room(&room);

        // println!("room is booked : {}",room_is_booked);

         if room_is_booked {

           return Err(StdError::generic_err("room is booked"))
         }

         
          let price_for_user_stay = no_of_day_stay * room.price_per_nigth;

          if price_for_user_stay !=  amount_paid {

           return Err(StdError::generic_err("amount pay by tenant is not valid"))
              
          }

          let booking_id = BOOKING_ID.load(deps.storage)?;

          if booking_id == 0 {

            let booking_room = BookingItem{
                booking_id : booking_id+1,
                booking_date : env.block.time,
                room_id,
                amount_paid,
                no_of_day_stay,
                tenant : info.sender.clone()
              };

              let update_booking_id = booking_id + 1;

              let booking_data = vec![booking_room];

          BOOKING_STATE.save(deps.storage, &booking_data)?;

          BOOKING_ID.save(deps.storage, &update_booking_id)?;

          let update_rooms =  rooms.iter_mut()
          .map(|room| {
              if room.room_id == room_id{
                  room.is_booked = true
              }
              room.clone()
          }).collect();

         

          ROOMS.save(deps.storage,&update_rooms)?;

          let event = Event::new("book_room")
               .add_attribute("room_id", room.room_id.to_string())
               .add_attribute("tenant_name", info.sender.to_owned());


         let resp = Response::new()
                 .add_event(event);
      
      
  
        return Ok(resp)              

          

    
          }


          let booking_room = BookingItem{
            booking_id : booking_id+1,
            booking_date : env.block.time,
            room_id,
            amount_paid,
            no_of_day_stay,
            tenant : info.sender.clone()
          };

          let mut booking_data = BOOKING_STATE.load(deps.storage)?;

          booking_data.push(booking_room.clone());

          BOOKING_STATE.save(deps.storage, &booking_data)?;

          BOOKING_ID.save(deps.storage, &booking_room.booking_id)?;

          let booking = BOOKING_STATE.load(deps.storage)?;

          for booking_item in booking  {

            println!(" booking is {:?}",booking_item);
              
          }

          let update_rooms =  rooms.iter_mut()
                .map(|room| {
                    if room.room_id == room_id{
                        room.is_booked = true
                    }
                    room.clone()
                }).collect();

               

                ROOMS.save(deps.storage,&update_rooms)?;

            //    for room in update_rooms {

            //     println!("room id : {}",room.room_id);
            //     println!("is_booked : {}",room.is_booked);
                   
            //    }
       


                let event = Event::new("book_room")
                     .add_attribute("room_id", room.room_id.to_string())
                     .add_attribute("tenant_name", info.sender.to_owned());
    

               let resp = Response::new()
                       .add_event(event);
            
            
        
              Ok(resp)              

                

    }


    pub fn ckeck_out_room(room_id : u64, deps: DepsMut,info:MessageInfo)-> Result<Response,StdError>{

    let mut rooms = ROOMS.load(deps.storage)?;    

    let room = get_room_details_by_id(&rooms, room_id)?;

     let check_room_booking = check_room_is_book_using_room(&room);

     if check_room_booking == false {

        return Err(StdError::generic_err("We can not check-out room because room is not booked"));
         
     }

     let booking_states = BOOKING_STATE.load(deps.storage)?;

      let mut is_tenant_fire_msg_checkout_authorized : bool = false;

      for booking_item in booking_states {


          println!("boking tenant is : {}",booking_item.tenant.to_owned());
          if booking_item.tenant == info.sender {
              
              is_tenant_fire_msg_checkout_authorized = true;
              break;
          }
      }

      if is_tenant_fire_msg_checkout_authorized  == false{
          
        return Err(StdError::generic_err("Unauthorized tenant"));  
      }

      let update_rooms =  rooms.iter_mut()
                .map(|room| {
                    if room.room_id == room_id{
                        room.is_booked = false
                    }
                    room.clone()
                }).collect();

               

                ROOMS.save(deps.storage,&update_rooms)?;

                println!("room is : {:?}",rooms);

                let event = Event::new("check_out_room")
                .add_attribute("room_id", room.room_id.to_string())
                .add_attribute("tenant_name", info.sender.to_owned());


          let resp = Response::new()
                  .add_event(event);
       
       
   
         Ok(resp)   

    }

}


pub fn check_room_is_book_using_room(room : &RoomItem )-> bool{

   if room.is_booked{

       return  true;    
   }

   return false;

}


pub fn get_room_details_by_id(rooms: &Vec<RoomItem>,room_id : u64)-> StdResult<RoomItem>{

    let mut room :Option<RoomItem> = None;

    for get_room in rooms  {

        if get_room.room_id == room_id{
         room = Some(get_room.clone());
         break;  
        }
        
    }

     match room {

        Some(get_room)=> Ok(get_room),
        None => Err(StdError::generic_err("room is not found"))      
     }

     

}


