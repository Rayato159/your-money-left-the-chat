// @generated automatically by Diesel CLI.

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
    monthly_spending,
    my_ledger,
    tax_deductions_list,
);
