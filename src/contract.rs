

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut,
     Env, MessageInfo, Response, StdResult,StdError
};


use crate::hotel_instantiate::instantiate_hotel_msg;
use crate:: hotel_msg::{InstantiateMsgForHotel,HotelExecuteMsg,HotelQuryMsg};
use crate::hotel_execute::execute_hotel_msg;
use crate::hotel_query::hotel_qury_msg;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsgForHotel,
) -> StdResult<Response> {
    
    instantiate_hotel_msg(deps, env, info, msg)
}







#[entry_point]
pub fn execute_hotel(

    deps : DepsMut,
    env:Env,
    info:MessageInfo,
    msg: HotelExecuteMsg
)-> Result<Response,StdError>{

   execute_hotel_msg(deps, env, info, msg)

}


pub fn hotel_qury(
    deps:Deps,
    env:Env,
    msg: HotelQuryMsg
)-> StdResult<Binary>{
 
    hotel_qury_msg(deps, env, msg)
  
}

