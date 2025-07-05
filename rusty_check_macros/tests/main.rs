use rusty_check_macros::{compose_mocks, rusty_check, rustymock};
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

#[rustymock(composable)]
trait TestMockTrait {
    fn test_method(&self) -> String;
}
#[rustymock(composable)]
trait TestMockTraitSecond {
    fn test_method_second(&self, next: String) -> String;
}
compose_mocks!(TestMockTrait, TestMockTraitSecond, ComposedMocks);

fn test_funcs<T: TestMockTrait + TestMockTraitSecond>(a: T, test_val: String) -> String {
    a.test_method();
    a.test_method_second(test_val)
}
rusty_check! {
    case test_compose_mocks {
        given {
            mut mock_a = MockTestMockTrait::new(),
            mut mock_b = MockTestMockTraitSecond::new(),
            return_val = "i runned".to_string(),
            check_result = return_val.clone()

        }
        do {
            mock_a.expect_test_method()
            .returning(move || {
                println!("🔧 Mocked `test_method` called!");
                return_val.clone()
            });
            mock_b.expect_test_method_second().returning(|next| {dbg!("{}",&next);next});
            let composed = ComposedMocks::new(mock_a, mock_b);
        }
        check {
            test_funcs(composed, check_result.clone()) equal check_result
        }
    }

}
