#[cfg(test)]
mod tests {
    use rusty_check_macros::rusty_check;
    rusty_check!{
        case tests {
            given {
                a=2,
                b=2
            }
            check {
                a equal b
            }
        }
    }
}
