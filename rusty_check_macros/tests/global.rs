use rusty_check_macros::rusty_check;
rusty_check! {
    global {
        vars {
            SOME_VAL: u32 = 21
        }
        consts {
            MY_CONST: u32 = 10
        }
    }

    case use_global_value {
        check {
            SOME_VAL equal 21
        }
    }
}
