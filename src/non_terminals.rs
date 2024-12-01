//! # Non-Terminal Tokens
//! 
//! This library all composite types: types built fundamentally off the terminals.
//! 
//! This library abstracts BNF by the difference in `struct` and `enum`.
//! 
//! If a non-terminal is a:
//! - `struct`, then the BNF for that type has only one variant, which always starts with a terminal.
//! - `enum`, then the BNF for that type has strictly more than one variant, each with at least one inner variant. See `Enum Cont.`
//! to read all the ways that `Enum` is used to abstract BNF.
//! 
//! ### Enum Cont.
//! 
//! As mentioned previously, the BNF for that type has strictly more than one variant, each with at least one inner variant.
//! 
//! Each enum also follows another rule: **all first inner variants of the enum are of the same terminal-class**. That is to say,
//! if any of the variants start with a terminal symbol, then all the variants of the same sum will also start with a terminal, and vice versa.
//! 
//! Another abstraction is optionality. If the enum (let's call it `T`) is only expected optionaly,
//! then the `Parse` trait implementation signature will be
//! ```
//! impl Parse<Option<Self>> for T
//! ```
//! rather than its usual
//! ```
//! impl Parse for T
//! ```
//! 
//! This is to avoid adding an `Empty` variant to each of these enums, and enfore
//! its optionality in parent composite types.

use crate::{
    make_indent,
    Parse,
    ParseBuffer,
    ParseDisplay,
    terminals::*,
    modulars::*,
};

/// A Function Definition
/// 
/// # BNF
/// ```text
/// <FUNCTION DEFINITION> -> type identifier (<FUNCTION PARAMETERS>){<COMPOUND STATEMENTS>}
/// ``` 
#[derive(Clone)] // We cannot derive `Copy` due to modulars, but we can clone
pub struct FunctionDefinition {
    pub type_: Type,
    pub function_name: Identifier,
    pub left_paren: LeftParen,
    pub parameters: FunctionParameters,
    pub right_paren: RightParen,
    pub left_curly: LeftCurly,
    pub compound_statements: CompoundStatements,
    pub right_curly: RightCurly,
}
impl Parse for FunctionDefinition {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        let function_parameter = FunctionDefinition {
            type_: Type::parse(&mut fork)?,
            function_name: Identifier::parse(&mut fork)?,
            left_paren: LeftParen::parse(&mut fork)?,
            parameters: FunctionParameters::parse(&mut fork)?,
            right_paren: RightParen::parse(&mut fork)?,
            left_curly: LeftCurly::parse(&mut fork)?,
            compound_statements: CompoundStatements::parse(&mut fork)?,
            right_curly: RightCurly::parse(&mut fork)?
        };
        *buffer = fork; // parse was successful: setting the buffer to the fork
        return Ok(function_parameter);
    }

    fn parse_label() -> String {
        format!("Function Definition")
    }
}
impl ParseDisplay for FunctionDefinition {
    fn display(&self, depth: usize, _label: Option<String>) {
        let indent = make_indent(depth);
        let label = "Function Definition";
        let lexemes_label = self.lexeme_signature();
        println!("{indent}{label}: {lexemes_label}");

        self.type_.display(depth+1, Some("Funtion Return Type".into()));
        self.function_name.display(depth+1, Some("Function Identifier".into()));
        self.left_paren.display(depth+1, Some("Left Paren".into()));
        self.parameters.display(depth+1, Some("Function Parameters".into()));
        self.right_paren.display(depth+1, Some("Right Paren".into()));
        self.left_curly.display(depth+1, Some("Left Curly".into()));
        self.compound_statements.display(depth+1, Some("Compound Statements".into()));
        self.right_curly.display(depth+1, Some("Right Curly".into()));
    }

    fn lexeme_signature(&self) -> String {
        let mut sigg = String::new();
        sigg.extend(self.type_.lexeme_signature().chars());
        sigg.extend(" ".chars());
        sigg.extend(self.function_name.lexeme_signature().chars());
        sigg.extend(" ".chars());
        sigg.extend(self.left_paren.lexeme_signature().chars());
        sigg.extend(self.parameters.lexeme_signature().chars());
        sigg.extend(self.right_paren.lexeme_signature().chars());
        sigg.extend(" ".chars());
        sigg.extend(self.left_curly.lexeme_signature().chars());
        sigg.extend("....".chars());
        sigg.extend(self.right_curly.lexeme_signature().chars());
        sigg
    }
}

/// A delimited list by Comma of Function Parameter
/// 
/// # BNF
/// ```text
/// <FUNCTION PARAMETERS> -> <FUNCTION PARAMETER><FUNCTION PARAMETERS'>
///                        | ε
/// <FUNCTION PARAMETERS'> -> ,<FUNCTION PARAMETER><FUNCTION PARAMETERS'>
///                         | ε
/// ```
pub type FunctionParameters = Delimited<FunctionParameter, Comma>;

/// A terminated list by Semicolon of Statement
/// 
/// # BNF
/// ```text
/// <COMPOUND STATEMENTS> -> <STATEMENT>;<COMPOUND STATEMENTS>
///                        | ε
/// ```
pub type CompoundStatements = Terminated<Statement, Semicolon>;

/// A Function Parameter
/// 
/// # BNF
/// ```text
/// <FUNCTION PARAMETER> -> type identifier
/// ```
#[derive(Clone, Copy)]
pub struct FunctionParameter {
    pub type_ : Type,
    pub identifier: Identifier,
}
impl Parse for FunctionParameter {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        let function_parameter = FunctionParameter {
            type_: Type::parse(&mut fork)?,
            identifier: Identifier::parse(&mut fork)?,
        };
        *buffer = fork; // parse was successful: setting the buffer to the fork
        return Ok(function_parameter);
    }

    fn parse_label() -> String {
        format!("Function Parameter")
    }
}
impl ParseDisplay for FunctionParameter {
    fn display(&self, depth: usize, _label: Option<String>) {
        let indent = make_indent(depth);
        let label = "Function Parameter";
        let lexemes_label = self.lexeme_signature();
        println!("{indent}{label}: {lexemes_label}");

        self.type_.display(depth+1, Some("Parameter Type".into()));
        self.identifier.display(depth+1, Some("Parameter Identifier".into()));
    }

    fn lexeme_signature(&self) -> String {
        let mut sigg = String::new();
        sigg.extend(self.type_.lexeme_signature().chars());
        sigg.extend(" ".chars());
        sigg.extend(self.identifier.lexeme_signature().chars());
        sigg
    }
}

/// A Statement
/// 
/// # BNF
/// ```text
/// <STATEMENT> -> <ASSIGNMENT STATEMENT>
///              | <RETURN STATEMENT>
/// ```
#[derive(Clone, Copy)]
pub enum Statement {
    Assignment(AssignmentStatement),
    Return(ReturnStatement),
}
impl Parse for Statement {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        match AssignmentStatement::parse(&mut fork) {
            Ok(assignment_statement) => {
                *buffer = fork; // parse was successful: setting the buffer to the fork
                return Ok(Statement::Assignment(assignment_statement));
            },
            Err(_) => (),
        }

        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        match ReturnStatement::parse(&mut fork) {
            Ok(return_statement) => {
                *buffer = fork; // parse was successful: setting the buffer to the fork
                return Ok(Statement::Return(return_statement));
            },
            Err(_) => (),
        }

        Err(format!("Expected either `{} {}` for {}, but found something else instead", AssignmentStatement::parse_label(), ReturnStatement::parse_label(), Self::parse_label()))
    }

    fn parse_label() -> String {
        format!("Statement")
    }
}
impl ParseDisplay for Statement {
    fn display(&self, depth: usize, _label: Option<String>) {
        let indent = make_indent(depth);
        let label = "Statement";
        println!("{indent}{label}:");
        
        match self {
            Statement::Assignment(assignment_statement) => assignment_statement.display(depth+1, None),
            Statement::Return(return_statement) => return_statement.display(depth+1, None),
        }
    }

    fn lexeme_signature(&self) -> String {
        match self {
            Statement::Assignment(assignment_statement) => assignment_statement.lexeme_signature(),
            Statement::Return(return_statement) => return_statement.lexeme_signature(),
        }
    }
}

/// An Assignment Statement
/// 
/// # BNF
/// ```text
/// <ASSIGNMENT STATEMENT> -> identifier = <EXPRESSION>
/// ```
#[derive(Clone, Copy)]
pub struct AssignmentStatement {
    pub lhs_identifier: Identifier,
    pub equals: Equals,
    pub expression: Expression,
}
impl Parse for AssignmentStatement {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        let assignment_statement = AssignmentStatement {
            lhs_identifier: Identifier::parse(&mut fork)?,
            equals: Equals::parse(&mut fork)?,
            expression: Expression::parse(&mut fork)?,
        };
        *buffer = fork; // parse was successful: setting the buffer to the fork
        return Ok(assignment_statement);
    }

    fn parse_label() -> String {
        format!("Assignment Statement")
    }
}
impl ParseDisplay for AssignmentStatement {
    fn display(&self, depth: usize, _label: Option<String>) {
        let indent = make_indent(depth);
        let label = "Assignment Statement";
        let lexemes_label = self.lexeme_signature();
        println!("{indent}{label}: {lexemes_label}");

        self.lhs_identifier.display(depth+1, Some("Identifier".into()));
        self.equals.display(depth+1, Some("Equals".into()));
        self.expression.display(depth+1, None);
    }

    fn lexeme_signature(&self) -> String {
        let mut sigg = String::new();
        sigg.extend(self.lhs_identifier.lexeme_signature().chars());
        sigg.extend(" ".chars());
        sigg.extend(self.equals.lexeme_signature().chars());
        sigg.extend(" ".chars());
        sigg.extend(self.expression.lexeme_signature().chars());
        sigg
    }
}

/// A Return Statement
/// 
/// # BNF
/// ```text
/// return <EXPRESSION>
/// ```
#[derive(Clone, Copy)]
pub struct ReturnStatement {
    pub return_ : Return,
    pub expression: Expression,
}
impl Parse for ReturnStatement {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        let return_statement = ReturnStatement {
            return_: Return::parse(&mut fork)?,
            expression: Expression::parse(&mut fork)?,
        };
        *buffer = fork; // parse was successful: setting the buffer to the fork
        return Ok(return_statement);
    }

    fn parse_label() -> String {
        format!("Return Statement")
    }
}
impl ParseDisplay for ReturnStatement {
    fn display(&self, depth: usize, _label: Option<String>) {
        let indent = make_indent(depth);
        let label = "Return Statement";
        let lexemes_label = self.lexeme_signature();
        println!("{indent}{label}: {lexemes_label}");

        self.return_.display(depth+1, Some("Return".into()));
        self.expression.display(depth+1, None);
    }

    fn lexeme_signature(&self) -> String {
        let mut sigg = String::new();
        sigg.extend(self.return_.lexeme_signature().chars());
        sigg.extend(" ".chars());
        sigg.extend(self.expression.lexeme_signature().chars());
        sigg
    }
}

/// An Expression
/// 
/// # BNF
/// ```text
/// <EXPRESSION> -> <ARITHMETIC EXPRESSION>
///               | <TYPECAST EXPRESSION>
/// ```
#[derive(Clone, Copy)]
pub enum Expression {
    Arithmetic(ArithmeticExpression),
    Typecast(TypecastExpression),
}
impl Parse for Expression {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        match ArithmeticExpression::parse(&mut fork) {
            Ok(arithmetic_expression) => {
                *buffer = fork; // parse was successful: setting the buffer to the fork
                return Ok(Expression::Arithmetic(arithmetic_expression));
            },
            Err(_) => (),
        }

        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        match TypecastExpression::parse(&mut fork) {
            Ok(typecast_expression) => {
                *buffer = fork; // parse was successful: setting the buffer to the fork
                return Ok(Expression::Typecast(typecast_expression));
            },
            Err(_) => (),
        }

        Err(format!("Expected either `{} {}` for {}, but found something else instead", ArithmeticExpression::parse_label(), TypecastExpression::parse_label(), Self::parse_label()))
    }

    fn parse_label() -> String {
        format!("Expression")
    }
} 
impl ParseDisplay for Expression {
    fn display(&self, depth: usize, _label: Option<String>) {
        let indent = make_indent(depth);
        let label = "Expression";
        println!("{indent}{label}:");

        match self {
            Expression::Arithmetic(arithmetic_expression) => arithmetic_expression.display(depth+1, None),
            Expression::Typecast(typecast_expression) => typecast_expression.display(depth+1, None),
        }
    }

    fn lexeme_signature(&self) -> String {
        match self {
            Expression::Arithmetic(arithmetic_expression) => arithmetic_expression.lexeme_signature(),
            Expression::Typecast(typecast_expression) => typecast_expression.lexeme_signature(),
        }
    }
}

/// A Typecast Expression
/// 
/// # BNF
/// ```text
/// <TYPECAST EXPRESSION> -> (type)identifier
/// ```
#[derive(Clone, Copy)]
pub struct TypecastExpression {
    pub left_paren: LeftParen,
    pub type_: Type,
    pub right_paren: RightParen,
    pub ident: Identifier,
}
impl Parse for TypecastExpression {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        let typecast_expression = TypecastExpression {
            left_paren: LeftParen::parse(&mut fork)?,
            type_: Type::parse(&mut fork)?,
            right_paren: RightParen::parse(&mut fork)?,
            ident: Identifier::parse(&mut fork)?
        };
        *buffer = fork; // parse was successful: setting the buffer to the fork
        return Ok(typecast_expression);
    }

    fn parse_label() -> String {
        format!("Typecast Expression")
    }
}
impl ParseDisplay for TypecastExpression {
    fn display(&self, depth: usize, _label: Option<String>) {
        let indent = make_indent(depth);
        let label = "Typecast Expression";
        let lexemes_label = self.lexeme_signature();
        println!("{indent}{label}: {lexemes_label}");

        self.left_paren.display(depth+1, Some("Left Paren".into()));
        self.type_.display(depth+1, Some("Cast Type".into()));
        self.right_paren.display(depth+1, Some("Right Paren".into()));
        self.ident.display(depth+1, Some("Cast Indentifier".into()));
    }

    fn lexeme_signature(&self) -> String {
        let mut sigg = String::new();
        sigg.extend(self.left_paren.lexeme_signature().chars());
        sigg.extend(self.type_.lexeme_signature().chars());
        sigg.extend(self.right_paren.lexeme_signature().chars());
        sigg.extend(self.ident.lexeme_signature().chars());
        sigg
    }
}

/// An Arithmetic Expression
/// 
/// # BNF
/// ```text
/// <ARITHMETIC EXPRESSION> -> <TERM><TERM'>
/// ```
#[derive(Clone, Copy)]
pub struct ArithmeticExpression {
    pub lhs_term: Term,
    pub extend: Option<TermExtend>
}
impl Parse for ArithmeticExpression {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        let arithmetic_expression = ArithmeticExpression {
            lhs_term: Term::parse(&mut fork)?,
            extend: TermExtend::parse(&mut fork)?
        };
        *buffer = fork; // parse was successful: setting the buffer to the fork
        return Ok(arithmetic_expression);
    }

    fn parse_label() -> String {
        format!("Arithmetic Expression")
    }
}
impl ParseDisplay for ArithmeticExpression {
    fn display(&self, depth: usize, _label: Option<String>) {
        
        let indent = make_indent(depth);
        let label = "Arithmetic Expression";
        let lexemes_label = self.lexeme_signature();
        println!("{indent}{label}: {lexemes_label}");
        
        self.lhs_term.display(depth+1, None);
        self.extend.as_ref().map(|extend| extend.display(depth+1, None));
    }

    fn lexeme_signature(&self) -> String {
        let mut sigg = String::new();
        sigg.extend(self.lhs_term.lexeme_signature().chars());
        if let Some(ref extend) = self.extend {
            sigg.push(' ');
            sigg.extend(extend.lexeme_signature().chars());
        }
        sigg
    }
}

/// A Term
/// 
/// This is basically something maybe seperated by + or -.
/// 
/// # BNF
/// ```text
/// <TERM> -> <FACTOR><FACTOR'>
/// ```
#[derive(Clone, Copy)]
pub struct Term {
    pub factor: Factor,
    pub extend: Option<FactorExtend>
}
impl Parse for Term {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        let term = Term {
            factor: Factor::parse(&mut fork)?,
            extend: FactorExtend::parse(&mut fork)?,
        };
        *buffer = fork; // parse was successful: setting the buffer to the fork
        return Ok(term);
    }

    fn parse_label() -> String {
        format!("Term")
    }
}
impl ParseDisplay for Term {
    fn display(&self, depth: usize, _label: Option<String>) {
        

        let indent = make_indent(depth);
        let label = "Term";
        let lexemes_label = self.lexeme_signature();
        println!("{indent}{label}: {lexemes_label}");

        self.factor.display(depth+1, None);
        self.extend.as_ref().map(|extend| extend.display(depth+1, None));
    }

    fn lexeme_signature(&self) -> String {
        let mut sigg = String::new();
        sigg.extend(self.factor.lexeme_signature().chars());
        if let Some(ref extend) = self.extend {
            sigg.push(' ');
            sigg.extend(extend.lexeme_signature().chars());
        }
        sigg
    }
}

/// A Term's Extension
/// 
/// This changes a statement to a statement with an addition.
/// 
/// # BNF
/// ```text
/// <TERM'> -> +<TERM>
///          | -<TERM>
///          | ε
/// ```
/// 
/// **Note:** the enum encapsulates the first two non-empty cases.
/// The ε option is encapsulated as the `Option<Self>` in the `Parse` implementation
/// signature
/// ```
/// impl Parse<Option<Self>> for TermExtend
/// ```
#[derive(Clone, Copy)]
pub enum TermExtend {
    Add(Plus, Term),
    Subtract(Minus, Term),
}
impl Parse<Option<Self>> for TermExtend {
    fn parse(buffer: &mut crate::ParseBuffer) -> Result<Option<Self>, String> {
        if buffer.peek().is_none() {
            return Ok(None);
        }

        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        match Plus::parse(&mut fork) {
            Ok(plus) => return Term::parse(&mut fork).map(|term| {
                *buffer = fork; // parse was successful: setting the buffer to the fork
                Some(TermExtend::Add(plus, term))
            }),
            Err(_) => ()
        }
        
        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        match Minus::parse(&mut fork) {
            Ok(minus) => return Term::parse(&mut fork).map(|term| {
                *buffer = fork; // parse was successful: setting the buffer to the fork
                Some(TermExtend::Subtract(minus, term))
            }),
            Err(_) => ()
        }

        Ok(None)
    }

    fn parse_label() -> String {
        format!("Term Extention")
    }
}
impl ParseDisplay for TermExtend {
    fn display(&self, depth: usize, _label: Option<String>) {
        
        let indent = make_indent(depth);

        // Stay at the same depth for Term: We have already been here
        match self {
            TermExtend::Add(plus, term) => {
                println!("{indent}Operator: {}", plus.lexeme_signature());
                term.display(depth, None);
            },
            TermExtend::Subtract(minus, term) => {
                println!("{indent}Operator: {}", minus.lexeme_signature());
                term.display(depth, None);
            },
        }
    }

    fn lexeme_signature(&self) -> String {
        let mut sigg = String::new();
        match self {
            TermExtend::Add(plus, term) => {
                sigg.extend(plus.lexeme_signature().chars());
                sigg.extend(" ".chars());
                sigg.extend(term.lexeme_signature().chars());
            },
            TermExtend::Subtract(minus, term) => {
                sigg.extend(minus.lexeme_signature().chars());
                sigg.extend(" ".chars());
                sigg.extend(term.lexeme_signature().chars());
            },
        };
        sigg
    }
}

/// A Factor
/// 
/// This is either a number or a literal.
/// 
/// # BNF
/// ```text
/// <FACTOR> -> identifier
///           | literal
/// ```
#[derive(Clone, Copy)]
pub enum Factor {
    Identifier(Identifier),
    Literal(Literal),
}
impl Parse for Factor {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        match Identifier::parse(&mut fork) {
            Ok(identifier) => {
                *buffer = fork; // parse was successful: setting the buffer to the fork
                return Ok(Factor::Identifier(identifier));
            },
            Err(_) => (),
        }

        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        match Literal::parse(&mut fork) {
            Ok(literal) => {
                *buffer = fork; // parse was successful: setting the buffer to the fork
                return Ok(Factor::Literal(literal));
            },
            Err(_) => (),
        }

        Err(format!("Expected either `{} {}` for {}, but found something else instead", Identifier::parse_label(), Literal::parse_label(), Self::parse_label()))
    }

    fn parse_label() -> String {
        format!("Factor")
    }
}
impl ParseDisplay for Factor {
    fn display(&self, depth: usize, _label: Option<String>) {
        let indent = make_indent(depth);
        let label = "Factor";
        let lexemes_label = self.lexeme_signature();
        println!("{indent}{label}: {lexemes_label}");

        match self {
            Factor::Identifier(identifier) => {
                identifier.display(depth+1, Some("Variable".into()));
            },
            Factor::Literal(literal) => {
                literal.display(depth+1, Some("Literal".into()));
            },
        }
    }

    fn lexeme_signature(&self) -> String {
        match self {
            Factor::Identifier(identifier) => identifier.lexeme_signature(),
            Factor::Literal(literal) => literal.lexeme_signature(),
        }
    }
}

/// A Factor's Extension
/// 
/// This changes a statement to a statement with a multiplication or division.
/// 
/// # BNF
/// ```text
/// <FACTOR'> -> *<FACTOR>
///            | /<FACTOR>
///            | ε
/// ```
/// 
/// **Note:** the enum encapsulates the first two non-empty cases.
/// The ε option is encapsulated as the `Option<Self>` in the `Parse` implementation
/// signature
/// ```
/// impl Parse<Option<Self>> for FactorExtend
/// ```
#[derive(Clone, Copy)]
pub enum FactorExtend {
    Multiply(Multiply, Factor),
    Divide(Divide, Factor),
}
impl Parse<Option<Self>> for FactorExtend {
    fn parse(buffer: &mut crate::ParseBuffer) -> Result<Option<Self>, String> {
        if buffer.peek().is_none() {
            return Ok(None);
        }

        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        match Multiply::parse(&mut fork) {
            Ok(multiply) => return Factor::parse(&mut fork).map(|factor| {
                *buffer = fork; // parse was successful: setting the buffer to the fork
                Some(FactorExtend::Multiply(multiply, factor))
            }),
            Err(_) => ()
        }
        
        let mut fork = buffer.fork(); // this is to make parse attempts without modifying the original buffer
        match Divide::parse(&mut fork) {
            Ok(divide) => return Factor::parse(&mut fork).map(|factor| {
                *buffer = fork; // parse was successful: setting the buffer to the fork
                Some(FactorExtend::Divide(divide, factor))
            }),
            Err(_) => ()
        }

        Ok(None)
    }

    fn parse_label() -> String {
        format!("Factor Extention")
    }
}
impl ParseDisplay for FactorExtend {
    fn display(&self, depth: usize, _label: Option<String>) {
        let indent = make_indent(depth);

        // Stay at the same depth for Term: We have already been here
        match self {
            FactorExtend::Multiply(multiply, factor) => {
                println!("{indent}Operator: {}", multiply.lexeme_signature());
                factor.display(depth, None);
            },
            FactorExtend::Divide(divide, factor) => {
                println!("{indent}Operator: {}", divide.lexeme_signature());
                factor.display(depth, None);
            },
        }
    }

    fn lexeme_signature(&self) -> String {
        let mut sigg = String::new();
        match self {
            FactorExtend::Multiply(multiply, factor) => {
                sigg.extend(multiply.lexeme_signature().chars());
                sigg.extend(" ".chars());
                sigg.extend(factor.lexeme_signature().chars());
            },
            FactorExtend::Divide(divide, factor) => {
                sigg.extend(divide.lexeme_signature().chars());
                sigg.extend(" ".chars());
                sigg.extend(factor.lexeme_signature().chars());
            },
        };
        sigg
    }
}
