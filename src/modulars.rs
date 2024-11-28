use std::{io::Write, marker::PhantomData, slice::Iter};

use crate::{make_indent, Parse, ParseDisplay};

/// Parses expecting a list of items, which are each delimited by a delimiter.
/// 
/// This will not parse a hanging delimiter. For instance, delimiter will parse the function parameters of
/// 
/// `int hello(int x, float y)`
/// 
/// but fail at
/// `int hello(int x, float y,)`
pub struct Delimited<Expected: Parse, Delimiter: Parse> {
    items: Vec<(Expected, Option<Delimiter>)>
}
impl<'delimited, E: Parse, D: Parse> IntoIterator for &'delimited Delimited<E, D> {
    type Item = &'delimited (E, Option<D>);

    type IntoIter = Iter<'delimited, (E, Option<D>)>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
impl<E: Parse, D: Parse> From<Vec<(E, Option<D>)>> for Delimited<E, D> {
    fn from(items: Vec<(E, Option<D>)>) -> Self {
        Delimited {
            items
        }
    }
}
impl<E: Parse, D: Parse> Parse for Delimited<E, D> {
    fn parse(buffer: &mut crate::ParseBuffer) -> Result<Self, String> {
        let mut items = vec![];
        let mut fork = buffer.fork();

        // test if the list is going to be empty
        let mut e =match E::parse(&mut fork) {
            Ok(e) => e,
            Err(_) => return Ok(items.into()),
        };

        match D::parse(&mut fork) {
            Ok(d) => items.push((e, Some(d))),
            Err(_) => {
                items.push((e, None));
                *buffer = fork;
                return Ok(items.into());
            },
        }

        // test for any additional items
        loop {
            let e = match E::parse(&mut fork) {
                Ok(e) => e,
                Err(err) => {
                    let mut err_msg = Vec::new();
                    writeln!(&mut err_msg, "While parsing {}...", Self::parse_label()).unwrap();
                    write!(&mut err_msg, "    {err}").unwrap();
                    return Err(String::from_utf8(err_msg).unwrap());
                },
            };

            match D::parse(&mut fork) {
                Ok(d) => items.push((e, Some(d))),
                Err(_) => {
                    items.push((e, None));
                    *buffer = fork;
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
    E: ParseDisplay + Parse,
    D: ParseDisplay + Parse
{
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
        if iter.peek().is_none() {
            return "".into();
        }
        loop {
            let (e, maybe_d) = iter.next().unwrap();
            
            sigg.extend(e.lexeme_signature().chars());
            
            if let Some(d) = maybe_d {
                assert!(iter.peek().is_some());
                sigg.extend(d.lexeme_signature().chars());
                sigg.extend(" ".chars());
            } else {
                assert!(iter.peek().is_none());
                break;
            }
        }
        sigg
    }
}

/// Parses expecting a list of items, which are each delimited by a delimiter.
/// 
/// This will ONLY parse a hanging delimiter. For instance, terminated will parse the statements such as of
/// 
/// ```
/// hello();
/// hello();
/// ```
/// 
/// but fail at
/// ```
/// hello();
/// hello() // <-- MISSING `;` delimiter!
/// ```
pub struct Terminated<Expected: Parse, Delimiter: Parse> {
    items: Vec<(Expected, Delimiter)>,
}
impl<'t, E: Parse, D: Parse> IntoIterator for &'t Terminated<E, D> {
    type Item = &'t (E, D);

    type IntoIter = Iter<'t, (E, D)>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
impl<E: Parse, D: Parse> From<Vec<(E, D)>> for Terminated<E, D> {
    fn from(items: Vec<(E, D)>) -> Self {
        Terminated {
            items,
        }
    }
}
impl<E: Parse, D: Parse> Parse for Terminated<E, D> {
    fn parse(buffer: &mut crate::ParseBuffer) -> Result<Self, String> {
        let mut items = vec![];
        let mut fork = buffer.fork();

        // test if the list is going to be empty
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

        // test for any additional items
        loop {
            let e = if let Ok(e) = E::parse(&mut fork) {
                e
            } else {
                *buffer = fork;
                return Ok(items.into());
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
        }
    }
    
    fn parse_label() -> String {
        format!("Terminated Sequence of `{}` by `{}`", E::parse_label(), D::parse_label())
    }
}
impl<E, D> ParseDisplay for Terminated<E, D>
where 
    E: ParseDisplay + Parse,
    D: ParseDisplay + Parse
{
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
        for (e, d) in self {
            sigg.extend(e.lexeme_signature().chars());
            sigg.extend(d.lexeme_signature().chars());
            sigg.extend(" ".chars());
        }
        sigg
    }
}