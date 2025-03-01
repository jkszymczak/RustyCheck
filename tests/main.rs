use rusty_check::rusty_check;
#[test]
fn test_one(){
    rusty_check!{
        check {
            1 equal 2 or 2 equal 3 and 2 less than 3
        }
    }
}
