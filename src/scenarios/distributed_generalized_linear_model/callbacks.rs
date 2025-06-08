use crate::internal::core::{Context, Message, log};

use super::{
    algorithms::{receive_concat_r_msg, receive_sum_rows_msg},
    message::{GlmConcatMessage, GlmSumRowsMessage},
};

pub fn on_message_receive(ctx: &mut Context, receiver_id: usize, msg: Box<dyn Message>) {
    if let Some(sum_rows_msg) = msg.downcast_ref::<GlmSumRowsMessage>() {
        receive_sum_rows_msg(ctx, receiver_id, *sum_rows_msg);
    } else if let Some(concat_msg) = msg.downcast_ref::<GlmConcatMessage>() {
        receive_concat_r_msg(ctx, receiver_id, concat_msg.clone());
    } else {
        log::warn(ctx, format!("Wrong message type received {msg:?}"));
    }
}
