

#[cfg(test)]
mod test{

    

    use std::vec;

    use cosmwasm_std::{coins, Addr};
    use cw_multi_test::{App,ContractWrapper,Executor};

    use crate::{ booking_stae::BookingItem, contract::{execute_hotel, hotel_qury, instantiate}, hotel_msg::{HotelExecuteMsg, HotelQuryMsg, InstantiateMsgForHotel}, room_state::RoomItem};


    #[test]
    fn to_test_add_room(){
        

        let mut app = App::default();

        let code = ContractWrapper::new(
            execute_hotel,instantiate,hotel_qury);

            
            let code_id = app.store_code(Box::new(code));

            let addr = app
                .instantiate_contract(
                    code_id,
                    Addr::unchecked("payal"),
                     &InstantiateMsgForHotel{
                        total_rooms : 2,
                        hotel_name : "taj_palace".to_owned(),
                        description: "hotel is very clean and view is plesant".to_owned(),
                        location_address: "bajaj nagar nagpur 440027".to_owned(),
                        image_hash : "12345678".to_owned(),
                        hotel_city : "nagpur".to_owned()
                     }, 
                     &[], 
                     "HotelSmartContract",
                      None
                    ).unwrap();

               
               let excute_add_room_msg = app
               .execute_contract(
                Addr::unchecked("payal"), 
                addr.clone(), 
                &HotelExecuteMsg::AddRoom { 
                    total_beds: 2,
                     price_per_nigth: 100,
                      room_number: 01
                    }, 
                     &[]
                ).unwrap();

         let event = excute_add_room_msg
                 .events
                 .iter()
                  .find(|event|event.ty== "wasm-room_add")
                  .expect("event not foun at add romm execute message");                   

        let attribute = event
                    .attributes
                     .iter()
                     .find(|attr|attr.key == "room_count")
                     .expect("attribute not found at add room exute msg");

                    assert_eq!(
                        attribute.value,
                        "1"
                    );

 
                    let excute_add_room_msg = app
                    .execute_contract(
                     Addr::unchecked("payal"), 
                     addr.clone(), 
                     &HotelExecuteMsg::AddRoom { 
                         total_beds: 2,
                          price_per_nigth: 100,
                           room_number: 02
                         }, 
                          &[]
                     ).unwrap();

                     let event = excute_add_room_msg
                     .events
                     .iter()
                      .find(|event|event.ty== "wasm-add_another_room")
                      .expect("event not foun at add romm execute message");                   
    
            let attribute = event
                        .attributes
                         .iter()
                         .find(|attr|attr.key == "room_id")
                         .expect("attribute not found at add room exute msg");
    
                        assert_eq!(
                            attribute.value,
                            "2"
                        );


                 let query_msg : Vec<RoomItem> = app
                      .wrap()
                      .query_wasm_smart(
                        addr.clone(), 
                        &HotelQuryMsg::GetAllRooms {  }
                    ).unwrap();  


                    assert_eq!(
                        query_msg,
                        vec![
                            RoomItem{
                                room_id : 1,
                                total_beds: 2,
                                price_per_nigth: 100,
                                 room_number: 01,
                                 is_booked : false,        
                            },
                            RoomItem{
                                room_id : 2,
                                total_beds: 2,
                                price_per_nigth: 100,
                                 room_number: 02,
                                 is_booked : false,        
                            },

                        ]
                    )    


    }



     #[test]
    fn test_all_available_room_for_book(){
      
        let mut app = App::default();

        let code = ContractWrapper::new(
            execute_hotel,instantiate,hotel_qury);

            
            let code_id = app.store_code(Box::new(code));

            let addr = app
                .instantiate_contract(
                    code_id,
                    Addr::unchecked("payal"),
                     &InstantiateMsgForHotel{
                        total_rooms : 2,
                        hotel_name : "taj_palace".to_owned(),
                        description: "hotel is very clean and view is plesant".to_owned(),
                        location_address: "bajaj nagar nagpur 440027".to_owned(),
                        image_hash : "12345678".to_owned(),
                        hotel_city : "nagpur".to_owned()
                     }, 
                     &[], 
                     "HotelSmartContract",
                      None
                    ).unwrap();

               
               let excute_add_room_msg = app
               .execute_contract(
                Addr::unchecked("payal"), 
                addr.clone(), 
                &HotelExecuteMsg::AddRoom { 
                    total_beds: 2,
                     price_per_nigth: 100,
                      room_number: 01
                    }, 
                     &[]
                ).unwrap();

         let event = excute_add_room_msg
                 .events
                 .iter()
                  .find(|event|event.ty== "wasm-room_add")
                  .expect("event not foun at add romm execute message");                   

        let attribute = event
                    .attributes
                     .iter()
                     .find(|attr|attr.key == "room_count")
                     .expect("attribute not found at add room exute msg");

                    assert_eq!(
                        attribute.value,
                        "1"
                    );

 
                    let excute_add_room_msg = app
                    .execute_contract(
                     Addr::unchecked("payal"), 
                     addr.clone(), 
                     &HotelExecuteMsg::AddRoom { 
                         total_beds: 2,
                          price_per_nigth: 100,
                           room_number: 02
                         }, 
                          &[]
                     ).unwrap();

                     let event = excute_add_room_msg
                     .events
                     .iter()
                      .find(|event|event.ty== "wasm-add_another_room")
                      .expect("event not foun at add romm execute message");                   
    
            let attribute = event
                        .attributes
                         .iter()
                         .find(|attr|attr.key == "room_id")
                         .expect("attribute not found at add room exute msg");
    
                        assert_eq!(
                            attribute.value,
                            "2"
                        );


                 let query_msg : Vec<RoomItem> = app
                      .wrap()
                      .query_wasm_smart(
                        addr.clone(), 
                        &HotelQuryMsg::GetAllRooms {  }
                    ).unwrap();  


                    assert_eq!(
                        query_msg,
                        vec![
                            RoomItem{
                                room_id : 1,
                                total_beds: 2,
                                price_per_nigth: 100,
                                 room_number: 01,
                                 is_booked : false,        
                            },
                            RoomItem{
                                room_id : 2,
                                total_beds: 2,
                                price_per_nigth: 100,
                                 room_number: 02,
                                 is_booked : false,        
                            },

                        ]
                    );


                let query_msg_show_available_room : Vec<RoomItem>= app
                                                     .wrap()
                                                     .query_wasm_smart(addr, &HotelQuryMsg::DisplayAvailableRoomForBooking {  })
                                                      .unwrap();


                       assert_eq!(
                        query_msg_show_available_room,
                        vec![
                            RoomItem{
                                room_id : 1,
                                total_beds: 2,
                                price_per_nigth: 100,
                                 room_number: 01,
                                 is_booked : false,        
                            },
                            RoomItem{
                                room_id : 2,
                                total_beds: 2,
                                price_per_nigth: 100,
                                 room_number: 02,
                                 is_booked : false,        
                            },

                        ]

                       );                                




    }


    #[test]
    fn test_booked_room(){
      

        let mut app = App::new(|router,
            _api,storage|{
  
              router
              .bank
              .init_balance(storage, &Addr::unchecked("shubham"),
               coins(3000, "eth"))
               .unwrap();

               router
               .bank
               .init_balance(storage, &Addr::unchecked("prachi"),
                coins(5000, "eth"))
                .unwrap()
            });
        let code = ContractWrapper::new(
            execute_hotel,instantiate,hotel_qury);

            
            let code_id = app.store_code(Box::new(code));

            let addr = app
                .instantiate_contract(
                    code_id,
                    Addr::unchecked("payal"),
                     &InstantiateMsgForHotel{
                        total_rooms : 2,
                        hotel_name : "taj_palace".to_owned(),
                        description: "hotel is very clean and view is plesant".to_owned(),
                        location_address: "bajaj nagar nagpur 440027".to_owned(),
                        image_hash : "12345678".to_owned(),
                        hotel_city : "nagpur".to_owned()
                     }, 
                     &[], 
                     "HotelSmartContract",
                      None
                    ).unwrap();

               
               let excute_add_room_msg = app
               .execute_contract(
                Addr::unchecked("payal"), 
                addr.clone(), 
                &HotelExecuteMsg::AddRoom { 
                    total_beds: 2,
                     price_per_nigth: 100,
                      room_number: 01
                    }, 
                     &[]
                ).unwrap();

         let event = excute_add_room_msg
                 .events
                 .iter()
                  .find(|event|event.ty== "wasm-room_add")
                  .expect("event not foun at add romm execute message");                   

        let attribute = event
                    .attributes
                     .iter()
                     .find(|attr|attr.key == "room_count")
                     .expect("attribute not found at add room exute msg");

                    assert_eq!(
                        attribute.value,
                        "1"
                    );

 
                    let excute_add_room_msg = app
                    .execute_contract(
                     Addr::unchecked("payal"), 
                     addr.clone(), 
                     &HotelExecuteMsg::AddRoom { 
                         total_beds: 2,
                          price_per_nigth: 100,
                           room_number: 02
                         }, 
                          &[]
                     ).unwrap();

                     let event = excute_add_room_msg
                     .events
                     .iter()
                      .find(|event|event.ty== "wasm-add_another_room")
                      .expect("event not foun at add romm execute message");                   
    

            let attribute = event
                        .attributes
                         .iter()
                         .find(|attr|attr.key == "room_id")
                         .expect("attribute not found at add room exute msg");
    
                        assert_eq!(
                            attribute.value,
                            "2"
                        );


                let query_msg_show_available_room : Vec<RoomItem>= app
                                                     .wrap()
                                                     .query_wasm_smart(addr.clone(), &HotelQuryMsg::DisplayAvailableRoomForBooking {  })
                                                      .unwrap();


                       assert_eq!(
                        query_msg_show_available_room,
                        vec![
                            RoomItem{
                                room_id : 1,
                                total_beds: 2,
                                price_per_nigth: 100,
                                 room_number: 01,
                                 is_booked : false,        
                            },
                            RoomItem{
                                room_id : 2,
                                total_beds: 2,
                                price_per_nigth: 100,
                                 room_number: 02,
                                 is_booked : false,        
                            },

                        ]

                       ); 

              let booked_room = app
                       .execute_contract(
                        Addr::unchecked("shubham"),
                         addr.clone(),
                          &HotelExecuteMsg::BookedRoom { 
                            room_id: 1, 
                            amount_paid: 200,
                             no_of_day_stay: 2 
                            }, 
                            &coins(200, "eth")
                        ).unwrap();


                    let booked_room_event = booked_room
                             .events
                              .iter()
                               .find(|event|event.ty == "wasm-book_room")
                               .expect("event not found at the booked room function"); 

                   

                     let attr = booked_room_event
                              .attributes
                               .iter()
                               .find(|att|att.key == "room_id")
                               .expect("attributed of room id booked room not found");         

                       
                        assert_eq!(
                            attr.value,
                            "1".to_owned()
                        );

                       
                        let attr = booked_room_event
                        .attributes
                         .iter()
                         .find(|att|att.key == "tenant_name")
                         .expect("attributed tenant name in booked room not found");   

                        assert_eq!(
                            attr.value,
                            "shubham"
                        );      
                                                  

                    let booked_room_second = app
                        .execute_contract(
                         Addr::unchecked("prachi"),
                          addr.clone(),
                           &HotelExecuteMsg::BookedRoom { 
                             room_id: 2, 
                             amount_paid: 100,
                              no_of_day_stay: 1
                             }, 
                             &coins(100, "eth")
                         ).unwrap();


                         let booked_room_event = booked_room_second
                         .events
                          .iter()
                           .find(|event|event.ty == "wasm-book_room")
                           .expect("event not found at the booked room function"); 

                        

                         let attr = booked_room_event
                         .attributes
                          .iter()
                          .find(|att|att.key == "room_id")
                          .expect("attributed in booked room not found");         
                                                   

                   assert_eq!(
                       attr.value,
                       "2".to_owned()
                   );

                   let attr = booked_room_event
                   .attributes
                    .iter()
                    .find(|att|att.key == "tenant_name")
                    .expect("attributed in booked room not found");   

                   assert_eq!(
                       attr.value,
                       "prachi"
                   );    


                 

    }

     #[test]
    fn check_out_room(){

        let mut app = App::new(|router,
            _api,storage|{
  
              router
              .bank
              .init_balance(storage, &Addr::unchecked("shubham"),
               coins(3000, "eth"))
               .unwrap();

               router
               .bank
               .init_balance(storage, &Addr::unchecked("prachi"),
                coins(5000, "eth"))
                .unwrap()
            });
        let code = ContractWrapper::new(
            execute_hotel,instantiate,hotel_qury);

            
            let code_id = app.store_code(Box::new(code));

            let addr = app
                .instantiate_contract(
                    code_id,
                    Addr::unchecked("payal"),
                     &InstantiateMsgForHotel{
                        total_rooms : 2,
                        hotel_name : "taj_palace".to_owned(),
                        description: "hotel is very clean and view is plesant".to_owned(),
                        location_address: "bajaj nagar nagpur 440027".to_owned(),
                        image_hash : "12345678".to_owned(),
                        hotel_city : "nagpur".to_owned()
                     }, 
                     &[], 
                     "HotelSmartContract",
                      None
                    ).unwrap();

               
               let excute_add_room_msg = app
               .execute_contract(
                Addr::unchecked("payal"), 
                addr.clone(), 
                &HotelExecuteMsg::AddRoom { 
                    total_beds: 2,
                     price_per_nigth: 100,
                      room_number: 01
                    }, 
                     &[]
                ).unwrap();

         let event = excute_add_room_msg
                 .events
                 .iter()
                  .find(|event|event.ty== "wasm-room_add")
                  .expect("event not foun at add romm execute message");                   

        let attribute = event
                    .attributes
                     .iter()
                     .find(|attr|attr.key == "room_count")
                     .expect("attribute not found at add room exute msg");

                    assert_eq!(
                        attribute.value,
                        "1"
                    );

 
                    let excute_add_room_msg = app
                    .execute_contract(
                     Addr::unchecked("payal"), 
                     addr.clone(), 
                     &HotelExecuteMsg::AddRoom { 
                         total_beds: 2,
                          price_per_nigth: 100,
                           room_number: 02
                         }, 
                          &[]
                     ).unwrap();

                     let event = excute_add_room_msg
                     .events
                     .iter()
                      .find(|event|event.ty== "wasm-add_another_room")
                      .expect("event not foun at add romm execute message");                   
    

            let attribute = event
                        .attributes
                         .iter()
                         .find(|attr|attr.key == "room_id")
                         .expect("attribute not found at add room exute msg");
    
                        assert_eq!(
                            attribute.value,
                            "2"
                        );


                let query_msg_show_available_room : Vec<RoomItem>= app
                                                     .wrap()
                                                     .query_wasm_smart(addr.clone(), &HotelQuryMsg::DisplayAvailableRoomForBooking {  })
                                                      .unwrap();


                       assert_eq!(
                        query_msg_show_available_room,
                        vec![
                            RoomItem{
                                room_id : 1,
                                total_beds: 2,
                                price_per_nigth: 100,
                                 room_number: 01,
                                 is_booked : false,        
                            },
                            RoomItem{
                                room_id : 2,
                                total_beds: 2,
                                price_per_nigth: 100,
                                 room_number: 02,
                                 is_booked : false,        
                            },

                        ]

                       ); 

              let booked_room = app
                       .execute_contract(
                        Addr::unchecked("shubham"),
                         addr.clone(),
                          &HotelExecuteMsg::BookedRoom { 
                            room_id: 1, 
                            amount_paid: 200,
                             no_of_day_stay: 2 
                            }, 
                            &coins(200, "eth")
                        ).unwrap();


                    let booked_room_event = booked_room
                             .events
                              .iter()
                               .find(|event|event.ty == "wasm-book_room")
                               .expect("event not found at the booked room function"); 

                   

                     let attr = booked_room_event
                              .attributes
                               .iter()
                               .find(|att|att.key == "room_id")
                               .expect("attributed of room id booked room not found");         

                       
                        assert_eq!(
                            attr.value,
                            "1".to_owned()
                        );

                       
                        let attr = booked_room_event
                        .attributes
                         .iter()
                         .find(|att|att.key == "tenant_name")
                         .expect("attributed tenant name in booked room not found");   

                        assert_eq!(
                            attr.value,
                            "shubham"
                        );      
                                                  

                    let booked_room_second = app
                        .execute_contract(
                         Addr::unchecked("prachi"),
                          addr.clone(),
                           &HotelExecuteMsg::BookedRoom { 
                             room_id: 2, 
                             amount_paid: 100,
                              no_of_day_stay: 1
                             }, 
                             &coins(100, "eth")
                         ).unwrap();


                         let booked_room_event = booked_room_second
                         .events
                          .iter()
                           .find(|event|event.ty == "wasm-book_room")
                           .expect("event not found at the booked room function"); 

                        

                         let attr = booked_room_event
                         .attributes
                          .iter()
                          .find(|att|att.key == "room_id")
                          .expect("attributed in booked room not found");         
                                                   

                   assert_eq!(
                       attr.value,
                       "2".to_owned()
                   );

                   let attr = booked_room_event
                   .attributes
                    .iter()
                    .find(|att|att.key == "tenant_name")
                    .expect("attributed in booked room not found");   

                   assert_eq!(
                       attr.value,
                       "prachi"
                   ); 

                    let query_msg_show_available_room : Vec<RoomItem>= app
                                                     .wrap()
                                                     .query_wasm_smart(addr.clone(), &HotelQuryMsg::DisplayAvailableRoomForBooking {  })
                                                      .unwrap();

                     assert_eq!(
                        query_msg_show_available_room,
                        vec![
                          
                        ]

                     );

                let execute_msg_for_check_out = app
                                .execute_contract(
                                    Addr::unchecked("prachi"), 
                                    addr.clone(),
                                     &HotelExecuteMsg::CheckOutRoom { 
                                        room_id: 2}, 
                                        &[]
                                    ).unwrap();

                          let check_out_room_event = execute_msg_for_check_out
                                    .events
                                     .iter()
                                      .find(|event|event.ty == "wasm-check_out_room")
                                      .expect("event not found at the booked room function"); 
       
                          
       
                            let attr = check_out_room_event
                                     .attributes
                                      .iter()
                                      .find(|att|att.key == "room_id")
                                      .expect("attributed of room id booked room not found");         
       
                              
                               assert_eq!(
                                   attr.value,
                                   "2".to_owned()
                               );
       
                              
                               let attr = check_out_room_event
                               .attributes
                                .iter()
                                .find(|att|att.key == "tenant_name")
                                .expect("attributed tenant name in booked room not found");   
       
                               assert_eq!(
                                   attr.value,
                                   "prachi"
                               );      
                                                                 
                    let available_room : Vec<RoomItem>= app
                                 .wrap()
                                 .query_wasm_smart(addr, &HotelQuryMsg::DisplayAvailableRoomForBooking {  })
                                 .unwrap();                
                 

                  assert_eq!(
                      available_room,
                      vec![
                        RoomItem{
                            room_id : 2,
                            total_beds: 2,
                            price_per_nigth: 100,
                             room_number: 02,
                             is_booked : false,        
                        },
                      ]
                  )

    }


     #[test]
    fn dispaly_all_booked_room(){

        let mut app = App::new(|router,
            _api,storage|{
  
              router
              .bank
              .init_balance(storage, &Addr::unchecked("shubham"),
               coins(3000, "eth"))
               .unwrap();

               router
               .bank
               .init_balance(storage, &Addr::unchecked("prachi"),
                coins(5000, "eth"))
                .unwrap()
            });
        let code = ContractWrapper::new(
            execute_hotel,instantiate,hotel_qury);

            
            let code_id = app.store_code(Box::new(code));

            let addr = app
                .instantiate_contract(
                    code_id,
                    Addr::unchecked("payal"),
                     &InstantiateMsgForHotel{
                        total_rooms : 2,
                        hotel_name : "taj_palace".to_owned(),
                        description: "hotel is very clean and view is plesant".to_owned(),
                        location_address: "bajaj nagar nagpur 440027".to_owned(),
                        image_hash : "12345678".to_owned(),
                        hotel_city : "nagpur".to_owned()
                     }, 
                     &[], 
                     "HotelSmartContract",
                      None
                    ).unwrap();

               
               let excute_add_room_msg = app
               .execute_contract(
                Addr::unchecked("payal"), 
                addr.clone(), 
                &HotelExecuteMsg::AddRoom { 
                    total_beds: 2,
                     price_per_nigth: 100,
                      room_number: 01
                    }, 
                     &[]
                ).unwrap();

         let event = excute_add_room_msg
                 .events
                 .iter()
                  .find(|event|event.ty== "wasm-room_add")
                  .expect("event not foun at add romm execute message");                   

        let attribute = event
                    .attributes
                     .iter()
                     .find(|attr|attr.key == "room_count")
                     .expect("attribute not found at add room exute msg");

                    assert_eq!(
                        attribute.value,
                        "1"
                    );

 
                    let excute_add_room_msg = app
                    .execute_contract(
                     Addr::unchecked("payal"), 
                     addr.clone(), 
                     &HotelExecuteMsg::AddRoom { 
                         total_beds: 2,
                          price_per_nigth: 100,
                           room_number: 02
                         }, 
                          &[]
                     ).unwrap();

                     let event = excute_add_room_msg
                     .events
                     .iter()
                      .find(|event|event.ty== "wasm-add_another_room")
                      .expect("event not foun at add romm execute message");                   
    

            let attribute = event
                        .attributes
                         .iter()
                         .find(|attr|attr.key == "room_id")
                         .expect("attribute not found at add room exute msg");
    
                        assert_eq!(
                            attribute.value,
                            "2"
                        );


                let query_msg_show_available_room : Vec<RoomItem>= app
                                                     .wrap()
                                                     .query_wasm_smart(addr.clone(), &HotelQuryMsg::DisplayAvailableRoomForBooking {  })
                                                      .unwrap();


                       assert_eq!(
                        query_msg_show_available_room,
                        vec![
                            RoomItem{
                                room_id : 1,
                                total_beds: 2,
                                price_per_nigth: 100,
                                 room_number: 01,
                                 is_booked : false,        
                            },
                            RoomItem{
                                room_id : 2,
                                total_beds: 2,
                                price_per_nigth: 100,
                                 room_number: 02,
                                 is_booked : false,        
                            },

                        ]

                       ); 

              let booked_room = app
                       .execute_contract(
                        Addr::unchecked("shubham"),
                         addr.clone(),
                          &HotelExecuteMsg::BookedRoom { 
                            room_id: 1, 
                            amount_paid: 200,
                             no_of_day_stay: 2 
                            }, 
                            &coins(200, "eth")
                        ).unwrap();


                    let booked_room_event = booked_room
                             .events
                              .iter()
                               .find(|event|event.ty == "wasm-book_room")
                               .expect("event not found at the booked room function"); 

                   

                     let attr = booked_room_event
                              .attributes
                               .iter()
                               .find(|att|att.key == "room_id")
                               .expect("attributed of room id booked room not found");         

                       
                        assert_eq!(
                            attr.value,
                            "1".to_owned()
                        );

                       
                        let attr = booked_room_event
                        .attributes
                         .iter()
                         .find(|att|att.key == "tenant_name")
                         .expect("attributed tenant name in booked room not found");   

                        assert_eq!(
                            attr.value,
                            "shubham"
                        );      
                                                  

                    let booked_room_second = app
                        .execute_contract(
                         Addr::unchecked("prachi"),
                          addr.clone(),
                           &HotelExecuteMsg::BookedRoom { 
                             room_id: 2, 
                             amount_paid: 100,
                              no_of_day_stay: 1
                             }, 
                             &coins(100, "eth")
                         ).unwrap();


                         let booked_room_event = booked_room_second
                         .events
                          .iter()
                           .find(|event|event.ty == "wasm-book_room")
                           .expect("event not found at the booked room function"); 

                        

                         let attr = booked_room_event
                         .attributes
                          .iter()
                          .find(|att|att.key == "room_id")
                          .expect("attributed in booked room not found");         
                                                   

                   assert_eq!(
                       attr.value,
                       "2".to_owned()
                   );

                   let attr = booked_room_event
                   .attributes
                    .iter()
                    .find(|att|att.key == "tenant_name")
                    .expect("attributed in booked room not found");   

                   assert_eq!(
                       attr.value,
                       "prachi"
                   );  

           let get_all_booked_room : Vec<BookingItem>= app
                     .wrap()
                     .query_wasm_smart(
                        addr.clone(),
                         &HotelQuryMsg::DisplayAllBookedRoom {  })
                         .unwrap();          


               println!("booked room lenght {}",get_all_booked_room.len());

    }

}