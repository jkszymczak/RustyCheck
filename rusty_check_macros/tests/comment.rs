use rusty_check_macros::rusty_check;

rusty_check! {
    case test_cond_comment {
        given {
            a = 2,
            b = 3
        }
        check {
            a less than b
        }
    }
    case test_comp_comment {
        given {
            a = 2,
            b = 3,
            c = 4

        }
        check {
            a equal b or b less than c
        }
    }

    case test_loop_comment {
        given {
            a = vec![1,2,3],
            b = 4

        }
        check {
            for each v in &a, *v less than b
        }
    }
    case test_negation {
        given {
            a = 2,
            b = 3
        }
        check {
            a not greater than b
        }
    }
}
