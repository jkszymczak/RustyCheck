use rusty_check_macros::{compose_mocks, rusty_check, rustymock};
#[cfg(feature = "mocking")]
#[rustymock(composable)]
trait TestMockTrait {
    fn test_method(&self) -> String;
}
#[cfg(feature = "mocking")]
#[rustymock(composable)]
trait TestMockTraitSecond {
    fn test_method_second(&self, next: String) -> String;
}
#[cfg(feature = "mocking")]
compose_mocks!(TestMockTrait, TestMockTraitSecond, ComposedMocks);

#[cfg(feature = "mocking")]
fn test_funcs<T: TestMockTrait + TestMockTraitSecond>(a: T, test_val: String) -> String {
    a.test_method();
    a.test_method_second(test_val)
}
#[cfg(feature = "mocking")]
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

rusty_check! {
    global {
        vars {
            x: u32 = 1,
        }

    }

}
