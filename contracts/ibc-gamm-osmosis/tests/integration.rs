//! This integration test tries to run and call the generated wasm.
//! It depends on a Wasm build being available, which you can create with `cargo wasm`.
//! Then running `cargo integration-test` will validate we can properly call into that generated Wasm.
//!
//! You can easily convert unit tests to integration tests.
//! 1. First copy them over verbatum,
//! 2. Then change
//!      let mut deps = mock_dependencies(20, &[]);
//!    to
//!      let mut deps = mock_instance(WASM, &[]);
//! 3. If you access raw storage, where ever you see something like:
//!      deps.storage.get(CONFIG_KEY).expect("no data stored");
//!    replace it with:
//!      deps.with_storage(|store| {
//!          let data = store.get(CONFIG_KEY).expect("no data stored");
//!          //...
//!      });
//! 4. Anywhere you see query(&deps, ...) you must replace it with query(&mut deps, ...)

use cosmwasm_std::testing::{
    mock_ibc_channel_connect_ack, mock_ibc_channel_open_init, mock_ibc_channel_open_try,
    mock_ibc_packet_recv, mock_wasmd_attr,
};
use cosmwasm_std::{
    attr, coins, BankMsg, ContractResult, CosmosMsg, Event, IbcBasicResponse, IbcOrder,
    IbcReceiveResponse, Reply, Response, SubMsgExecutionResponse, SubMsgResult, WasmMsg,
};
use cosmwasm_vm::testing::{
    ibc_channel_connect, ibc_channel_open, ibc_packet_receive, instantiate, mock_env, mock_info,
    mock_instance, query, reply, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_vm::{from_slice, Instance};

use ibc_gamm_osmosis::msg::{
    AcknowledgementMsg, InstantiateMsg,
    IbcSwapPacket, IbcSwapResponse
};


// This line will test the output of cargo wasm
static WASM: &[u8] = include_bytes!("/home/pegasus/gamm-bounty/target/wasm32-unknown-unknown/release/ibc_gamm_osmosis.wasm");

const CREATOR: &str = "creator";

const DESERIALIZATION_LIMIT: usize = 20_000;

fn setup() -> Instance<MockApi, MockStorage, MockQuerier> {
    let mut deps = mock_instance(WASM, &[]);
    let msg = InstantiateMsg {
    };
    let info = mock_info(CREATOR, &[]);
    let res: Response = instantiate(&mut deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    deps
}

fn fake_events(reflect_addr: &str) -> Vec<Event> {
    let event = Event::new("instantiate").add_attributes(vec![
        attr("code_id", "17"),
        // We have to force this one to avoid the debug assertion against _
        mock_wasmd_attr("_contract_address", reflect_addr),
    ]);
    vec![event]
}

// connect will run through the entire handshake to set up a proper connect and
// save the account (tested in detail in `proper_handshake_flow`)
fn connect(
    deps: &mut Instance<MockApi, MockStorage, MockQuerier>,
    channel_id: &str,
    account: impl Into<String>,
) {
    let account: String = account.into();
    // first we try to open with a valid handshake
    let handshake_open = mock_ibc_channel_open_init(channel_id, IbcOrder::Ordered, "ibc-gamm-1");
    ibc_channel_open(deps, mock_env(), handshake_open).unwrap();

    // then we connect (with counter-party version set)
    let handshake_connect =
        mock_ibc_channel_connect_ack(channel_id, IbcOrder::Ordered, "ibc-gamm-1");
    let res: IbcBasicResponse = ibc_channel_connect(deps, mock_env(), handshake_connect).unwrap();
    // assert_eq!(1, res.messages.len());
    // assert_eq!(1, res.events.len());
    // assert_eq!(
    //     Event::new("ibc").add_attribute("channel", "connect"),
    //     res.events[0]
    // );
}



#[test]
fn handle_dispatch_packet() {
    let mut deps = setup();

    let channel_id = "channel-123";
    let account = "acct-123";

 

    let ibc_msg = IbcSwapPacket {
        pool_id: 1,
        in_amount: "10".to_owned(),
        in_denom: "test".to_owned(),
        min_out_amount: "1".to_owned(), 
        out_denom: "test".to_owned(), 
        to_address: "addr".to_owned(),
    };

    // register the channel
    connect(&mut deps, channel_id, account);

    // receive a packet for an unregistered channel returns app-level error (not Result::Err)
    let msg = mock_ibc_packet_recv(channel_id, &ibc_msg).unwrap();
    let res: IbcReceiveResponse = ibc_packet_receive(&mut deps, mock_env(), msg).unwrap();

    let ack: AcknowledgementMsg<IbcSwapResponse> =
        from_slice(&res.acknowledgement, DESERIALIZATION_LIMIT).unwrap();
    ack.unwrap();

    // // and we dispatch the BankMsg
    // assert_eq!(1, res.messages.len());
    // assert_eq!(RECEIVE_DISPATCH_ID, res.messages[0].id);

    // // parse the output, ensuring it matches
    // if let CosmosMsg::Wasm(WasmMsg::Execute {
    //     contract_addr,
    //     msg,
    //     funds,
    // }) = &res.messages[0].msg
    // {
    //     assert_eq!(account, contract_addr.as_str());
    //     assert_eq!(0, funds.len());
    //     // parse the message - should callback with proper channel_id
    //     let rmsg: ReflectExecuteMsg = from_slice(msg, DESERIALIZATION_LIMIT).unwrap();
    //     assert_eq!(
    //         rmsg,
    //         ReflectExecuteMsg::ReflectMsg {
    //             msgs: msgs_to_dispatch
    //         }
    //     );
    // } else {
    //     panic!("invalid return message: {:?}", res.messages[0]);
    // }

    // // invalid packet format on registered channel also returns app-level error
    // let bad_data = InstantiateMsg {
    //     reflect_code_id: 12345,
    // };
    // let msg = mock_ibc_packet_recv(channel_id, &bad_data).unwrap();
    // let res: IbcReceiveResponse = ibc_packet_receive(&mut deps, mock_env(), msg).unwrap();
    // // we didn't dispatch anything
    // assert_eq!(0, res.messages.len());
    // // acknowledgement is an error
    // let ack: AcknowledgementMsg<DispatchResponse> =
    //     from_slice(&res.acknowledgement, DESERIALIZATION_LIMIT).unwrap();
    // assert_eq!(ack.unwrap_err(), "invalid packet: Error parsing into type ibc_reflect::msg::PacketMsg: unknown variant `reflect_code_id`, expected one of `dispatch`, `who_am_i`, `balances`");
}
