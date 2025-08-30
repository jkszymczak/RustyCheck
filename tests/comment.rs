use rusty_check::rusty_check;

rusty_check! {
    global {
        cfg {
            unstable = true,
            create module = true,
            cfg = feature = "pls",
        }
    }
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

    mod data_tests{
    #[derive(Debug,PartialEq)]
    pub enum MyEnum {
        Value,
        No,
    }
        #[derive(Debug)]
        pub struct MyStruct {
            pub val: i32
        }

        impl MyStruct {
            pub fn add_five(&self) -> i32{
                self.val + 5
            }
        }
    }
    case test_enum {
        given {
            v = data_tests::MyEnum::Value
        }
        check {
            v equal data_tests::MyEnum::Value
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

    case test_method {
        given {
            v = data_tests::MyStruct { val: 10}
        }
        check {
            v.add_five() equal v.val+5
        }
    }
    case test_unstable {
        cfg {
            unstable = true
        }
        check {
            1 equal 2
        }
    }

}
