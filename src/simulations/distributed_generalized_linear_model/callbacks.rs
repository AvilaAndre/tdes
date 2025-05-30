use crate::{
    internal::{context::Context, message::Message},
    simulations::distributed_generalized_linear_model::algorithms::receive_sum_rows_msg,
};

use super::message::{GlmConcatMessage, GlmSumRowsMessage};

pub fn on_message_receive(ctx: &mut Context, receiver_id: usize, msg: Option<Box<dyn Message>>) {
    if msg.is_none() {
        return;
    }
    let msg = msg.unwrap();

    if let Some(sum_rows_msg) = msg.downcast_ref::<GlmSumRowsMessage>() {
        receive_sum_rows_msg(ctx, receiver_id, sum_rows_msg.clone());
    } else if let Some(concat_msg) = msg.downcast_ref::<GlmConcatMessage>() {
        println!("Matched GlmConcatMessage : {:?}", concat_msg);
    } else {
        // TODO: Log that wrong message type was received
    }
}
