use rusty_check_macros::rusty_check;
rusty_check! {
    case testing {
        given {
            mut a = 22,
            b = 33
        }
        do {
           a = a+b;
        }
        check {
            a greater than b
        }
    }
    case testing2 {
        given {
            col = vec![1,2,3,4],
        }
        check {
            for each c in col, c equal c
        }
    }

    case testing3 {
        given {
            col = vec![1,2,3,4],
        }
        check {
            for any c in col, c equal 2
        }
    }
}
