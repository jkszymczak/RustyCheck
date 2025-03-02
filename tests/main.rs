use rusty_check::rusty_check;
rusty_check!{
    case testing {
        given {
            a = 22,
            b = 33
        }
        check {
            a less than b
        }
    }
}
