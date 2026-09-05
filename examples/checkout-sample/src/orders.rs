use crate::model::Order;

pub fn database_insert(_order: Order) {}

pub fn validate(_order: &Order) {}

pub fn handle_order(order: Order) {
    validate(&order);
    database_insert(order);
    emit_receipt();
}
