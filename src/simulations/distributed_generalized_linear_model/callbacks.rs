use crate::{
    internal::core::{Context, message::Message},
    simulations::distributed_generalized_linear_model::algorithms::{receive_concat_r_msg, receive_sum_rows_msg},
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
        receive_concat_r_msg(ctx, receiver_id, concat_msg.clone());
    } else {
        // TODO: Log that wrong message type was received
    }
}
