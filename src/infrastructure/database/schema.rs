// @generated automatically by Diesel CLI.

diesel::table! {
    bitcoin_buy_ledger (id) {
        id -> Nullable<Integer>,
        amount -> Float,
        price -> Float,
        cost -> Float,
        date -> Text,
    }
}

diesel::table! {
    bitcoin_sell_ledger (id) {
        id -> Nullable<Integer>,
        amount -> Float,
        price -> Float,
        cost -> Float,
        date -> Text,
    }
}

diesel::table! {
    debt_ledger (id) {
        id -> Nullable<Integer>,
        amount -> Float,
        category -> Text,
        description -> Nullable<Text>,
        who -> Text,
        date -> Text,
    }
}

diesel::table! {
    my_ledger (id) {
        id -> Nullable<Integer>,
        amount -> Float,
        category -> Text,
        description -> Nullable<Text>,
        date -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bitcoin_buy_ledger,
    bitcoin_sell_ledger,
    debt_ledger,
    my_ledger,
);
