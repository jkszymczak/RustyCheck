use rusty_check_macros::rusty_check;

rusty_check! {
    global {
    }
    case test_compose_mocks {
        configure = all(feature = "mocking", feature="unstable")
        given {
            return_val = "i runned".to_string(),
            check_result = return_val.clone()

        }
        check {
            check_result equal check_result
        }
    }

}
