use crate::model::Order;
use crate::orders::handle_order;

pub fn post_order(order: Order) {
    handle_order(order)
}
