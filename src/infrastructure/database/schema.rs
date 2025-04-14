// @generated automatically by Diesel CLI.

diesel::table! {
    bitcoin_buy_ledger (id) {
        id -> Integer,
        amount -> Float,
        price -> Float,
        cost -> Float,
        date -> Text,
    }
}

diesel::table! {
    bitcoin_sell_ledger (id) {
        id -> Integer,
        amount -> Float,
        price -> Float,
        cost -> Float,
        date -> Text,
    }
}

diesel::table! {
    debt_ledger (id) {
        id -> Integer,
        amount -> Float,
        category -> Text,
        description -> Text,
        who -> Text,
        date -> Text,
    }
}

diesel::table! {
    monthly_spending (id) {
        id -> Integer,
        title -> Text,
        amount -> Float,
        due_date -> Text,
    }
}

diesel::table! {
    my_ledger (id) {
        id -> Integer,
        amount -> Float,
        category -> Text,
        description -> Text,
        date -> Text,
    }
}

diesel::table! {
    tax_deductions_list (id) {
        id -> Integer,
        title -> Text,
        amount -> Float,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bitcoin_buy_ledger,
    bitcoin_sell_ledger,
    debt_ledger,
    monthly_spending,
    my_ledger,
    tax_deductions_list,
);
