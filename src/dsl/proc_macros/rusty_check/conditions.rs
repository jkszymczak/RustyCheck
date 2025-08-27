use super::super::helpers::{filter_out_streams_with_ident, Comment, ToComment};
use super::configure::CommentType;
use super::{condition::Condition, keywords as kw};
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Token};

/// Represents different types of conditions in the RustyCheck DSL.
///
/// Variants:
/// - `LoopCondition`: A condition that involves iterating over a collection.
/// - `CompoundCondition`: A condition composed of two sub-conditions joined by a logical operator.
/// - `Condition`: A single condition.
///
/// represents grammar from this diagram:
///
#[derive(Clone)]
pub enum Conditions {
    LoopCondition {
        /// The type of loop (e.g., `ForAny` or `ForEach`).
        loop_type: LoopType,
        /// The collection being iterated over.
        collection: syn::Expr,
        /// The element being iterated.
        element: syn::Ident,
        /// The condition applied to each element.
        condition: Box<Conditions>,
    },
    CompoundCondition {
        /// The left-hand side condition.
        left_condition: Condition,
        /// The logical operator joining the conditions (`And` or `Or`).
        join: JoinType,
        /// The right-hand side condition.
        right_condition: Box<Conditions>,
    },
    /// A single condition.
    Condition(Condition),
}

/// Represents the type of loop used in a `LoopCondition`.
///
/// Variants:
/// - `ForAny`: A loop that checks if any element satisfies the condition.
/// - `ForEach`: A loop that checks if all elements satisfy the condition.
#[derive(Clone)]
pub enum LoopType {
    ForAny,
    ForEach,
}

/// Represents the logical operator used in a `CompoundCondition`.
///
/// Variants:
/// - `Or`: Logical OR (`||`).
/// - `And`: Logical AND (`&&`).
#[derive(Clone)]
pub enum JoinType {
    Or,
    And,
}

impl Parse for JoinType {
    /// Parses a `JoinType` from the input stream.
    ///
    /// Recognizes the keywords `and` and `or`.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(_) = input.parse::<kw::and>() {
            Ok(JoinType::And)
        } else {
            input.parse::<kw::or>()?;
            Ok(JoinType::Or)
        }
    }
}

impl ToTokens for JoinType {
    /// Converts a `JoinType` into its token representation.
    ///
    /// - `Or` -> `||`
    /// - `And` -> `&&`
    fn to_tokens(&self, tokens: &mut TS) {
        let join_type = match self {
            JoinType::Or => quote! {||},
            JoinType::And => quote! {&&},
        };
        tokens.extend(join_type);
    }
}

impl ToString for JoinType {
    /// Converts a `JoinType` into a human-readable string.
    ///
    /// - `Or` -> `"or"`
    /// - `And` -> `"and"`
    fn to_string(&self) -> String {
        match self {
            JoinType::Or => "or".to_owned(),
            JoinType::And => "and".to_owned(),
        }
    }
}

impl ToString for LoopType {
    /// Converts a `LoopType` into a human-readable string.
    ///
    /// - `ForAny` -> `"for any"`
    /// - `ForEach` -> `"for each"`
    fn to_string(&self) -> String {
        match self {
            LoopType::ForAny => "for any".to_owned(),
            LoopType::ForEach => "for each".to_owned(),
        }
    }
}

/// Checks if the input stream represents a `for each` loop.
fn is_for_each(input: &syn::parse::ParseStream) -> bool {
    input.peek(Token![for]) && input.peek2(kw::each)
}

/// Checks if the input stream represents a `for any` loop.
fn is_in_any(input: &syn::parse::ParseStream) -> bool {
    input.peek(Token![for]) && input.peek2(kw::any)
}

/// Parses a `for` loop condition from the input stream.
///
/// # Parameters
/// - `input`: The parse stream to read from.
/// - `loop_type`: The type of loop (`ForAny` or `ForEach`).
///
/// # Returns
/// A `Conditions::LoopCondition` representing the parsed loop.
fn parse_for_loop(input: syn::parse::ParseStream, loop_type: LoopType) -> syn::Result<Conditions> {
    input.parse::<Token![for]>()?;
    match loop_type {
        LoopType::ForAny => {
            input.parse::<kw::any>()?;
        }
        LoopType::ForEach => {
            input.parse::<kw::each>()?;
        }
    }
    let element = input.parse::<syn::Ident>()?;
    input.parse::<Token![in]>()?;
    let collection = input.parse::<syn::Expr>()?;
    input.parse::<Token![,]>()?;
    let conditions = input.parse::<Conditions>()?;
    Ok(Conditions::LoopCondition {
        collection: collection,
        element: element,
        loop_type: loop_type,
        condition: Box::new(conditions),
    })
}

/// Parses a loop condition from the input stream.
///
/// Determines whether the loop is a `for each` or `for any` loop and parses accordingly.
fn parse_loop_condition(input: syn::parse::ParseStream) -> syn::Result<Conditions> {
    if is_for_each(&input) {
        parse_for_loop(&input, LoopType::ForEach)
    } else if is_in_any(&input) {
        parse_for_loop(&input, LoopType::ForAny)
    } else {
        Err(input.error("Unknown loop condition"))
    }
}

impl Parse for Conditions {
    /// Parses a `Conditions` instance from the input stream.
    ///
    /// Handles `LoopCondition`, `CompoundCondition`, and `Condition` variants.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if is_for_each(&input) || is_in_any(&input) {
            return parse_loop_condition(input);
        }
        let condition = input.parse::<Condition>()?;
        if input.peek(kw::or) || input.peek(kw::and) {
            let join_type = input.parse::<JoinType>()?;
            return Ok(Conditions::CompoundCondition {
                left_condition: condition,
                join: join_type,
                right_condition: Box::new(input.parse()?),
            });
        }
        Ok(Conditions::Condition(condition))
    }
}

impl ToTokens for Conditions {
    /// Converts a `Conditions` instance into its token representation.
    ///
    /// Generates Rust code for the condition, including loops and compound conditions.
    fn to_tokens(&self, tokens: &mut TS) {
        let conditions = match self {
            Conditions::LoopCondition {
                loop_type: LoopType::ForEach,
                collection,
                element,
                condition,
            } => {
                quote! { (#collection).into_iter().all(|#element|#condition)}
            }
            Conditions::LoopCondition {
                loop_type: LoopType::ForAny,
                collection,
                element,
                condition,
            } => {
                quote! { (#collection).into_iter().any(|#element|#condition ) }
            }
            Conditions::CompoundCondition {
                left_condition,
                join,
                right_condition,
            } => {
                quote! { (#left_condition #join #right_condition) }
            }
            Conditions::Condition(condition) => condition.to_token_stream(),
        };
        tokens.extend(conditions);
    }
}

impl ToString for Conditions {
    /// Converts a `Conditions` instance into a human-readable string.
    ///
    /// Handles all variants, including loops and compound conditions.
    fn to_string(&self) -> String {
        match self {
            Conditions::LoopCondition {
                loop_type,
                collection,
                element,
                condition,
            } => {
                loop_type.to_string()
                    + " "
                    + collection.to_token_stream().to_string().as_str()
                    + " "
                    + "in"
                    + " "
                    + element.to_string().as_str()
                    + ", "
                    + condition.to_string().as_str()
            }
            Conditions::CompoundCondition {
                left_condition,
                join,
                right_condition,
            } => {
                left_condition.to_string()
                    + " "
                    + join.to_string().as_str()
                    + " "
                    + right_condition.to_string().as_str()
            }
            Conditions::Condition(condition) => condition.to_string(),
        }
    }
}

impl ToComment for Conditions {
    /// Converts a `Conditions` instance into a `Comment` object.
    ///
    /// The `Comment` includes a string representation of the condition and its associated values.
    fn to_comment(&self, comment_type: CommentType) -> Comment {
        match comment_type {
            CommentType::Simple => Comment {
                string: self.to_string(),
                values: vec![],
            },
            CommentType::ShowValues => match &self {
                Conditions::LoopCondition {
                    loop_type: _,
                    collection,
                    element,
                    condition,
                } => {
                    let cond_comment = condition.to_comment(comment_type);
                    let comment = self.to_string();
                    let filtered_values =
                        filter_out_streams_with_ident(&cond_comment.values, element)
                            .into_iter()
                            .map(|v| v.clone())
                            .collect();
                    Comment {
                        string: comment,
                        values: vec![vec![collection.to_token_stream()], filtered_values].concat(),
                    }
                }
                Conditions::CompoundCondition {
                    left_condition,
                    join: _,
                    right_condition,
                } => {
                    let left_comment = left_condition.to_comment(comment_type);
                    let right_comment = right_condition.to_comment(comment_type);
                    let comment = self.to_string();
                    let left_value = left_comment.values;
                    let right_value = right_comment.values;
                    Comment {
                        string: comment,
                        values: vec![left_value, right_value].concat(),
                    }
                }
                Conditions::Condition(condition) => condition.to_comment(comment_type),
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    #[test]
    fn test_parse_condition() {
        let input = "x greater than 5";
        let condition = syn::parse_str::<Condition>(input).unwrap();
        assert_eq!(condition.left.to_token_stream().to_string(), "x");
        assert_eq!(condition.symbol.to_token_stream().to_string(), ">");
        assert_eq!(condition.right.to_token_stream().to_string(), "5");
    }

    #[test]
    fn test_parse_conditions() {
        let input = "x greater than 5 and y less than 10";
        let conditions = syn::parse_str::<Conditions>(input).unwrap();
        match conditions {
            Conditions::CompoundCondition {
                left_condition,
                join,
                right_condition,
            } => {
                assert_eq!(left_condition.to_string(), "x greater than 5");
                assert_eq!(join.to_string(), "and");
                assert_eq!(right_condition.to_string(), "y less than 10");
            }
            _ => panic!("Expected CompoundCondition"),
        }
    }

    #[test]
    fn test_parse_loop_condition_for_each() {
        let input = "for each item in items, item greater than 5";
        let conditions = syn::parse_str::<Conditions>(input).unwrap();
        match conditions {
            Conditions::LoopCondition {
                loop_type,
                collection,
                element,
                condition,
            } => {
                assert_eq!(loop_type.to_string(), "for each");
                assert_eq!(collection.to_token_stream().to_string(), "items");
                assert_eq!(element.to_string(), "item");
                assert_eq!(condition.to_string(), "item greater than 5");
            }
            _ => panic!("Expected LoopCondition"),
        }
    }

    #[test]
    fn test_parse_loop_condition_for_any() {
        let input = "for any item in items, item greater than 5";
        let conditions = syn::parse_str::<Conditions>(input).unwrap();
        match conditions {
            Conditions::LoopCondition {
                loop_type,
                collection,
                element,
                condition,
            } => {
                assert_eq!(loop_type.to_string(), "for any");
                assert_eq!(collection.to_token_stream().to_string(), "items");
                assert_eq!(element.to_string(), "item");
                assert_eq!(condition.to_string(), "item greater than 5");
            }
            _ => panic!("Expected LoopCondition"),
        }
    }

    #[test]
    fn test_to_tokens_condition() {
        let input = "x greater than 5";
        let condition = parse_str::<Conditions>(input).unwrap();
        assert_eq!(
            condition.to_token_stream().to_string(),
            parse_str::<TS>("(x > 5)").unwrap().to_string()
        );
    }

    #[test]
    fn test_to_tokens_conditions() {
        let input = "x greater than 5 and y less than 10";
        let conditions = parse_str::<Conditions>(input).unwrap();
        assert_eq!(
            conditions.to_token_stream().to_string(),
            parse_str::<TS>("((x > 5) && (y < 10))")
                .unwrap()
                .to_string()
        );
    }

    #[test]
    fn test_to_tokens_loop_condition_for_each() {
        let input = "for each item in items, item greater than 5";
        let conditions = parse_str::<Conditions>(input).unwrap();
        assert_eq!(
            conditions.to_token_stream().to_string(),
            parse_str::<TS>("(items).into_iter().all(| item| (item > 5))")
                .unwrap()
                .to_string()
        );
    }

    #[test]
    fn test_to_tokens_loop_condition_for_any() {
        let input = "for any item in items, item greater than 5";
        let conditions = parse_str::<Conditions>(input).unwrap();
        assert_eq!(
            conditions.to_token_stream().to_string(),
            parse_str::<TS>("(items).into_iter().any(| item| (item > 5))")
                .unwrap()
                .to_string()
        );
    }
}
