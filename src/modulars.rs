//! # Modular Tokens (Lists/Statements)
//! 
//! This library stores the "modular" tokens.
//! 
//! This inludes `Delimited` and `Terminated`.
//! 
//! These types abstract-away a particular type
//! of BNF implementation.
//! 
//! Specifically,
//! 
//! #### Delimited BNF
//! ```text
//! <A>  -> e<A'>
//!       | ε
//! <A'> -> de<A'>
//!       | ε
//! ```
//! 
//! #### Terminated BNF
//! ```text
//! <A>  -> ed<A>
//!       | ε
//! ```
//! 
//! Where `e` and `d` are each the `Expected` item in the list and the `Delimiter` of the list.

use std::{
    io::Write, // Used with the `writeln!` and `write!` macros. Similar to sprintf in c.
    slice::Iter // The standard iterator type over slices.
};

use crate::{
    make_indent,
    Parse,
    ParseDisplay
};

/// Parses expecting a list of items, which are each delimited by a delimiter.
/// 
/// This struct completely encapsulates the implementation of the following BNF
/// #### Delimited BNF
/// ```text
/// <A>  -> e<A'>
///       | ε
/// <A'> -> de<A'>
///       | ε
/// ```
/// 
/// #### Object Structure
/// ```
/// pub struct Delimited<Expected: Parse, Delimiter: Parse> {
///     items: Vec<(Expected, Option<Delimiter>)>
/// }
/// ```
/// 
/// ##### `items: Vec<(Expected, Option<Delimiter>)>`
/// This will be a list of objects, which can be empty.
/// 
/// If it is non-empty, then only the very last tuple of the list will contain
/// `None`, rather than `Some`. This implementation guarentees it.
#[derive(Clone)]
pub struct Delimited<Expected: Parse, Delimiter: Parse> {
    items: Vec<(Expected, Option<Delimiter>)>
}
impl<E: Parse, D: Parse> Delimited<E, D> {
    /// A getter to the delimited items.
    pub fn items(&self) -> &Vec<(E, Option<D>)> {
        &self.items
    }
}
impl<'d, E: Parse, D: Parse> IntoIterator for &'d Delimited<E, D> {
    type Item = &'d (E, Option<D>);

    type IntoIter = Iter<'d, (E, Option<D>)>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter() // get the iterator directly from the internal items
    }
}
/// DO NOT USE THIS UNLESS YOU KNOW WHAT YOU ARE DOING!
/// 
/// To use this safely, you must guarentee that:
/// - for all items in the list, only the last contains `None` as the tuple's second variant.
impl<E: Parse, D: Parse> From<Vec<(E, Option<D>)>> for Delimited<E, D> {
    fn from(items: Vec<(E, Option<D>)>) -> Self {
        Delimited {
            items
        }
    }
}
impl<E: Parse, D: Parse> Parse for Delimited<E, D> {
    fn parse(buffer: &mut crate::ParseBuffer) -> Result<Self, String> {
        // INITIALIZATION
        let mut items = vec![];
        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer

        // ATTEMPT TO GET THE FIRST EXPECTED
        //
        // Empty list is a success or no delimiter is a success.
        let e = match E::parse(&mut fork) {
            Ok(e) => e,
            Err(_) => return Ok(items.into()),
        };
        match D::parse(&mut fork) {
            Ok(d) => items.push((e, Some(d))),
            Err(_) => {
                items.push((e, None));
                *buffer = fork; // parse was successful: setting the buffer to the fork
                return Ok(items.into());
            },
        }

        // test for any additional items
        loop {
            // EXPECT THE EXPECTED
            let e = match E::parse(&mut fork) {
                Ok(e) => e,
                Err(err) => {
                    // construct error message
                    let mut err_msg = Vec::new();
                    writeln!(&mut err_msg, "While parsing {}...", Self::parse_label()).unwrap();
                    write!(&mut err_msg, "    {err}").unwrap();

                    // return error
                    return Err(String::from_utf8(err_msg).unwrap());
                },
            };

            // A successful delimiter implies another iteration...
            match D::parse(&mut fork) {
                Ok(d) => items.push((e, Some(d))),
                Err(_) => {
                    items.push((e, None));
                    *buffer = fork; // parse was successful: setting the buffer to the fork
                    return Ok(items.into());
                },
            }
        }
    }
    
    fn parse_label() -> String {
        format!("Delimited Sequence of `{}` by `{}`", E::parse_label(), D::parse_label())
    }
}
impl<E, D> ParseDisplay for Delimited<E, D>
where 
    E: Parse,
    D: Parse
{
    /// Label is recommended...
    fn display(&self, depth: usize, label: Option<String>) {
        let indent = make_indent(depth);
        let label = label.unwrap_or(Self::parse_label());
        let lexemes_label = self.lexeme_signature();
        println!("{indent}{label}: {lexemes_label}");

        for (e, _d) in self {
            e.display(depth+1, None);
        }
    }

    fn lexeme_signature(&self) -> String {
        let mut sigg = String::new();
        
        let mut iter = self.items.iter().peekable();
        
        // if the list is empty, return the empty string
        if iter.peek().is_none() {
            return "".into();
        }
        
        // otherwise, list out all of the tokens, leveraging assumptions made about the structure of the items
        loop {
            let (e, maybe_d) = iter.next().unwrap();
            
            sigg.extend(e.lexeme_signature().chars());
            
            if let Some(d) = maybe_d {
                assert!(iter.peek().is_some()); // guarentees we must adhere to
                sigg.extend(d.lexeme_signature().chars());
                sigg.extend(" ".chars());
            } else {
                assert!(iter.peek().is_none()); // guarentees we must adhere to
                break; // No more items, exit out of loop
            }
        }

        sigg
    }
}

/// Parses expecting a list of items, each terminated by a delimiter.
/// 
/// This struct completely encapsulates the implementation of the following BNF
/// #### Terminated BNF
/// ```text
/// <A>  -> ed<A>
///       | ε
/// ```
/// 
/// #### Object Structure
/// ```
/// pub struct Terminated<Expected: Parse, Delimiter: Parse> {
///     items: Vec<(Expected, Delimiter)>,
/// }
/// ```
/// 
/// ##### `items: Vec<(Expected, Delimiter)>`
/// This will be a list of objects, which can be empty.
#[derive(Clone)]
pub struct Terminated<Expected: Parse, Delimiter: Parse> {
    items: Vec<(Expected, Delimiter)>,
}
impl<'t, E: Parse, D: Parse> Terminated<E, D> {
    /// A getter for the terminating items
    pub fn items(&self) -> &Vec<(E, D)> {
        &self.items
    }
}
impl<'t, E: Parse, D: Parse> IntoIterator for &'t Terminated<E, D> {
    type Item = &'t (E, D);

    type IntoIter = Iter<'t, (E, D)>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter() // get the iterator directly from the internal items
    }
}
/// Would not recommend using, but fine nonetheless
impl<E: Parse, D: Parse> From<Vec<(E, D)>> for Terminated<E, D> {
    fn from(items: Vec<(E, D)>) -> Self {
        Terminated {
            items,
        }
    }
}
impl<E: Parse, D: Parse> Parse for Terminated<E, D> {
    fn parse(buffer: &mut crate::ParseBuffer) -> Result<Self, String> {
        // INITALIZATION
        let mut items = vec![];
        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer

        // ATTEMPT TO GET THE FIRST EXPECTED AND DELIMITED
        // Empty list (no first expected) is a success
        let e = match E::parse(&mut fork) {
            Ok(e) => e,
            Err(_) => return Ok(items.into()),
        };
        match D::parse(&mut fork) {
            Ok(d) => items.push((e, d)),
            Err(err) => {
                let mut err_msg = Vec::new();
                writeln!(&mut err_msg, "While parsing {}...", Self::parse_label()).unwrap();
                write!(&mut err_msg, "    {err}").unwrap();
                return Err(String::from_utf8(err_msg).unwrap());
            },
        }

        // CONSUME UNTIL SATISFIED
        loop {
            // ATTEMPT TO GET THE NEXT EXPECTED AND DELIMITED
            // Return at first failed expected,
            // but error at first failed delimiter
            let e = match E::parse(&mut fork) {
                Ok(e) => e,
                Err(_) => return {
                    *buffer = fork; // parse was successful: setting the buffer to the fork
                    Ok(items.into())
                },
            };
            match D::parse(&mut fork) {
                Ok(d) => items.push((e, d)), // store, and parse again
                
                // a delimiter is non-optional: failure at first parse
                Err(err) => {
                    // create the error message
                    let mut err_msg = Vec::new();
                    writeln!(&mut err_msg, "While parsing {}...", Self::parse_label()).unwrap();
                    write!(&mut err_msg, "    {err}").unwrap();
                    
                    return Err(String::from_utf8(err_msg).unwrap());
                },
            }
        }
    }
    
    fn parse_label() -> String {
        format!("Terminated Sequence of `{}` by `{}`", E::parse_label(), D::parse_label())
    }
}
impl<E, D> ParseDisplay for Terminated<E, D>
where 
    E: Parse,
    D: Parse
{
    /// A label is recommended...
    fn display(&self, depth: usize, label: Option<String>) {
        let indent = make_indent(depth);
        let label = label.unwrap_or(Self::parse_label());
        let lexemes_label = self.lexeme_signature();
        println!("{indent}{label}: {lexemes_label}");

        // displays each expected item, ignoring the delimiter as redundant
        for (e, _d) in self {
            e.display(depth+1, None);
        }
    }

    fn lexeme_signature(&self) -> String {
        let mut sigg = String::new();
        
        let mut iter = self.into_iter().peekable(); // a raw *peekable* iterator over the items
        while let Some((e, d)) = iter.next() {
            // always include the expected and delimited
            sigg.extend(e.lexeme_signature().chars());
            sigg.extend(d.lexeme_signature().chars());
            
            // only if there will be a next item, include a space
            if iter.peek().is_some() {
                sigg.extend(" ".chars());
            }
        }
        sigg
    }
}