use rusty_check_macros::rusty_check;
rusty_check! {
    case loop_cond_on_vec {
        given {
            v = vec![1,2]
        }
        check {
            for each n in v.clone(), n greater than 0
        }
    }

    case loop_cond_on_vec_with_borrowing {
        given {
            v = vec![1,2]
        }
        check {
            for each n in &v, *n greater than 0
        }
    }
    case loop_cond_on_arr {
        given {
            arr = [1;5]
        }
        check {
            for each n in arr, n greater than 0
        }
    }
    case loop_cond_on_arr_with_borrowing {
        given {
            arr = [1;5]
        }
        check {
            for each n in &arr, *n greater than 0
        }
    }
    case loop_cond_on_slice {
        given {
            v = vec![10, 20, 30, 40, 50],
            full_slice: &[i32] = &v,
            part_slice: &[i32] = &v[1..4],
        }
        check {
            for each n in part_slice, *n greater than 0 and for any n in full_slice, *n less than 100
        }
    }

    mod enum_tests{
    #[derive(Debug,PartialEq)]
    pub enum MyEnum {
        Value,
        No,
    }}
    case test_enum {
        given {
            v = enum_tests::MyEnum::Value
        }
        check {
            v equal enum_tests::MyEnum::Value
        }
    }
    fn test_nothing(x:i32) -> i32{
        println!("I runn");
        x
    }
    case test_with_fun {
        given {
            v = 1
        }
        check {
            v equal test_nothing(v)
        }
    }



}
