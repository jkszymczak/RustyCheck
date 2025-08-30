use rusty_check::rusty_check;
#[cfg(true)]
rusty_check! {
    case complex_conf{
        cfg {
            unstable = true,
            cfg = true
        }
        check {
            1 equal 2
        }
    }
    case complex_conf_rev{
        cfg {
            cfg = true,
            unstable = true,
        }
        check {
            1 equal 2
        }
    }
    case always_off{
        cfg {
            cfg = false,
            unstable = true,
        }
        check {
            1 equal 2
        }
    }
    case always_off_complex{
        cfg {
            cfg = all(true,true,false),
            unstable = true,
        }
        check {
            1 equal 2
        }
    }

    case comment_option{
        cfg {
            comment=simple,

        }
        check {
            1 equal 2
        }
    }
}
