use q1_lib::lexer::Token;
use q1_lib::lexer::Symbol as Sym;

use crate::make_indent;
use crate::ParseDisplay;
use crate::{modulars::{Delimited, Terminated}, terminals::*, Parse, ParseBuffer};

pub struct FunctionDefinition {
    type_: Type,
    function_name: Identifier,
    left_paren: LeftParen,
    parameters: Delimited<FunctionParameter, Comma>,
    right_paren: RightParen,
    left_curly: LeftCurly,
    compound_statements: Terminated<Statement, Semicolon>,
    right_curly: RightCurly,
}
impl Parse for FunctionDefinition {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork();
        let function_parameter = FunctionDefinition {
            type_: Type::parse(&mut fork)?,
            function_name: Identifier::parse(&mut fork)?,
            left_paren: LeftParen::parse(&mut fork)?,
            parameters: Delimited::parse(&mut fork)?,
            right_paren: RightParen::parse(&mut fork)?,
            left_curly: LeftCurly::parse(&mut fork)?,
            compound_statements: Terminated::parse(&mut fork)?,
            right_curly: RightCurly::parse(&mut fork)?
        };
        *buffer = fork;
        return Ok(function_parameter);
    }

    fn parse_label() -> String {
        todo!()
    }
}
impl ParseDisplay for FunctionDefinition {
    fn display(&self, depth: usize, label: Option<String>) {
        let indent = make_indent(depth);
        let label = "Function Parameter";
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

struct FunctionParameter {
    type_ : Type,
    identifier: Identifier,
}
impl Parse for FunctionParameter {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork();
        let function_parameter = FunctionParameter {
            type_: Type::parse(&mut fork)?,
            identifier: Identifier::parse(&mut fork)?,
        };
        *buffer = fork;
        return Ok(function_parameter);
    }

    fn parse_label() -> String {
        format!("Function Parameter")
    }
}
impl ParseDisplay for FunctionParameter {
    fn display(&self, depth: usize, label: Option<String>) {
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

enum Statement {
    Assignment(AssignmentStatement),
    Return(ReturnStatement),
}
impl Parse for Statement {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork();
        match AssignmentStatement::parse(&mut fork) {
            Ok(assignment_statement) => {
                *buffer = fork;
                return Ok(Statement::Assignment(assignment_statement));
            },
            Err(_) => (),
        }

        let mut fork = buffer.fork();
        match ReturnStatement::parse(&mut fork) {
            Ok(return_statement) => {
                *buffer = fork;
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
    fn display(&self, depth: usize, label: Option<String>) {
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

struct AssignmentStatement {
    lhs_identifier: Identifier,
    equals: Equals,
    expression: Expression,
}
impl Parse for AssignmentStatement {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork();
        let assignment_statement = AssignmentStatement {
            lhs_identifier: Identifier::parse(&mut fork)?,
            equals: Equals::parse(&mut fork)?,
            expression: Expression::parse(&mut fork)?,
        };
        *buffer = fork;
        return Ok(assignment_statement);
    }

    fn parse_label() -> String {
        format!("Assignment Statement")
    }
}
impl ParseDisplay for AssignmentStatement {
    fn display(&self, depth: usize, label: Option<String>) {
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

struct ReturnStatement {
    return_ : Return,
    expression: Expression,
}
impl Parse for ReturnStatement {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork();
        let return_statement = ReturnStatement {
            return_: Return::parse(&mut fork)?,
            expression: Expression::parse(&mut fork)?,
        };
        *buffer = fork;
        return Ok(return_statement);
    }

    fn parse_label() -> String {
        format!("Return Statement")
    }
}
impl ParseDisplay for ReturnStatement {
    fn display(&self, depth: usize, label: Option<String>) {
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

enum Expression {
    Arithmetic(ArithmeticExpression),
    Typecast(TypecastExpression),
}
impl Parse for Expression {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork();
        match ArithmeticExpression::parse(&mut fork) {
            Ok(arithmetic_expression) => {
                *buffer = fork;
                return Ok(Expression::Arithmetic(arithmetic_expression));
            },
            Err(_) => (),
        }

        let mut fork = buffer.fork();
        match TypecastExpression::parse(&mut fork) {
            Ok(typecast_expression) => {
                *buffer = fork;
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
    fn display(&self, depth: usize, label: Option<String>) {
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

struct TypecastExpression {
    left_paren: LeftParen,
    type_: Type,
    right_paren: RightParen,
    ident: Identifier,
}
impl Parse for TypecastExpression {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork();
        let typecast_expression = TypecastExpression {
            left_paren: LeftParen::parse(&mut fork)?,
            type_: Type::parse(&mut fork)?,
            right_paren: RightParen::parse(&mut fork)?,
            ident: Identifier::parse(&mut fork)?
        };
        *buffer = fork;
        return Ok(typecast_expression);
    }

    fn parse_label() -> String {
        format!("Typecast Expression")
    }
}
impl ParseDisplay for TypecastExpression {
    fn display(&self, depth: usize, label: Option<String>) {
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

pub struct ArithmeticExpression {
    lhs_term: Term,
    extend: Option<TermExtend>
}
impl Parse for ArithmeticExpression {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork();
        let arithmetic_expression = ArithmeticExpression {
            lhs_term: Term::parse(&mut fork)?,
            extend: TermExtend::parse(&mut fork)?
        };
        *buffer = fork;
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

struct Term {
    factor: Factor,
    extend: Option<FactorExtend>
}
impl Parse for Term {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork();
        let term = Term {
            factor: Factor::parse(&mut fork)?,
            extend: FactorExtend::parse(&mut fork)?,
        };
        *buffer = fork;
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

enum TermExtend {
    Add(Plus, Term),
    Subtract(Minus, Term),
}
impl Parse<Option<Self>> for TermExtend {
    fn parse(buffer: &mut crate::ParseBuffer) -> Result<Option<Self>, String> {
        if buffer.peek().is_none() {
            return Ok(None);
        }

        let mut fork = buffer.fork();
        match Plus::parse(&mut fork) {
            Ok(plus) => return Term::parse(&mut fork).map(|term| {
                *buffer = fork;
                Some(TermExtend::Add(plus, term))
            }),
            Err(_) => ()
        }
        
        let mut fork = buffer.fork();
        match Minus::parse(&mut fork) {
            Ok(minus) => return Term::parse(&mut fork).map(|term| {
                *buffer = fork;
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

enum Factor {
    Identifier(Identifier),
    Literal(Literal),
}
impl Parse for Factor {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, String> {
        if buffer.peek().is_none() {
            Err(format!("Expected `{}`, but found nothing instead", Self::parse_label()))?
        }

        let mut fork = buffer.fork();
        match Identifier::parse(&mut fork) {
            Ok(identifier) => {
                *buffer = fork;
                return Ok(Factor::Identifier(identifier));
            },
            Err(_) => (),
        }

        let mut fork = buffer.fork();
        match Literal::parse(&mut fork) {
            Ok(literal) => {
                *buffer = fork;
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

enum FactorExtend {
    Multiply(Multiply, Factor),
    Divide(Divide, Factor),
}
impl Parse<Option<Self>> for FactorExtend {
    fn parse(buffer: &mut crate::ParseBuffer) -> Result<Option<Self>, String> {
        if buffer.peek().is_none() {
            return Ok(None);
        }

        let mut fork = buffer.fork();
        match Multiply::parse(&mut fork) {
            Ok(multiply) => return Factor::parse(&mut fork).map(|factor| {
                *buffer = fork;
                Some(FactorExtend::Multiply(multiply, factor))
            }),
            Err(_) => ()
        }
        
        let mut fork = buffer.fork();
        match Divide::parse(&mut fork) {
            Ok(divide) => return Factor::parse(&mut fork).map(|factor| {
                *buffer = fork;
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
    fn display(&self, depth: usize, label: Option<String>) {
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
