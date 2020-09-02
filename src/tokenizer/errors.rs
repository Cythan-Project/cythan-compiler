use super::stage1::Position;

pub enum Errors {
    FunctionNotFound {
        function_name: String,
        position: Position,
    },
    VariableNotFound {
        variable_name: String,
        position: Position,
    },
    LabelNotFound {
        label_name: String,
        position: Position,
    },
    EmptyVariable {
        varname: String,
        position: Position,
    },
    UndefinedVariable {
        varname: String,
        position: Position,
    },
    UnableToReadLitteral {
        litteral: String,
        position: Position,
    },
    SelfExpressionMissingNumberBeforeQuestionMark {
        expression: String,
        position: Position,
    },
    SelfExpressionMissingNumber {
        expression: String,
        position: Position,
    },
    SelfExpressionYNotNumber {
        expression: String,
        position: Position,
    },
    SelfExpressionXNotNumber {
        expression: String,
        position: Position,
    },
    EqualNotPrecededByLitteral {
        position: Position,
    },
    LiteralAfterAssignement {
        literal: String,
        assignement: String,
        position: Position,
    },
    BlockAfterAssignement {
        assignement: String,
        position: Position,
    },
    BlockMustBePrecededByLiteral {
        position: Position,
    },
    ParenthesisNotInAssignementOrFunctionCall {
        position: Position,
    },
    AssignementFollowedByAnotherAssignement {
        assignement1: String,
        assignement2: String,
        position: Position,
    },
}

impl std::fmt::Debug for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_pretty_print())
    }
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_pretty_print())
    }
}

impl Errors {
    pub fn to_pretty_print(&self) -> String {
        let mut out = String::new();
        match self {
            Errors::FunctionNotFound {
                position,
                function_name,
            } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str(&format!("Function `{}` was not found. \r\n", function_name));
                out.push_str(" - Verify that the function is created before used.\r\n");
                out.push_str(" - Check for similar names.\r\n");
                out.push_str(" - Create the function using:\r\n");
                out.push_str(&format!(" fn {} {{\r\n", function_name));
                out.push_str("    <code>\r\n");
                out.push_str(" }");
            }
            Errors::VariableNotFound {
                position,
                variable_name,
            } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str(&format!("Variable `{}` was not found. \r\n", variable_name));
                out.push_str("Try to init your variable with a value:\r\n");
                out.push_str(&format!(" {} = (<value>)\r\n", variable_name));
                out.push_str("Or check if:\r\n");
                out.push_str(" - The name is correct\r\n");
                out.push_str(" - This is not normally a label\r\n");
                out.push_str(&format!("  Use: `'{}` instead\r\n", variable_name));
                out.push_str(" - This is not normally a function\r\n");
                out.push_str(&format!("  Use: `{}()` instead", variable_name));
            }
            Errors::LabelNotFound {
                position,
                label_name,
            } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str(&format!("Label `'{}` was not found. \r\n", label_name));
                out.push_str("Try to init your label at an index:\r\n");
                out.push_str(&format!(" '{}:<case value>\r\n", label_name));
                out.push_str("Or check if:\r\n");
                out.push_str(" - The name is correct\r\n");
                out.push_str(" - The label is declared when this runs\r\n");
                out.push_str(" - The label is owned by a function\r\n");
                out.push_str(&format!("  Use: `'#{}` instead\r\n", label_name));
                out.push_str(" - The label is not global\r\n");
                out.push_str(&format!("  Use: `'#{}` instead", label_name));
            }
            Errors::EmptyVariable { position, varname } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str(&format!("Variable `{}` is empty \r\n", varname));
                out.push_str("Try to init your variable with a value:\r\n");
                out.push_str(&format!(" {} = (<value>)", varname));
            }
            Errors::UndefinedVariable { position, varname } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str(&format!("Variable `{}` is undefined \r\n", varname));
                out.push_str("Try to init your variable with a value:\r\n");
                out.push_str(&format!(" {} = (<value>)", varname));
            }
            Errors::UnableToReadLitteral { position, litteral } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str(&format!("Can't compile litteral {} \r\n", litteral));
                out.push_str("Is your synthax correct ?\r\n");
                out.push_str("Examples of litterals:\r\n");
                out.push_str(" 0 \r\n");
                out.push_str(" ~-1 \r\n");
                out.push_str(" self.0..10?9 \r\n");
                out.push_str(" 'label:var1 \r\n");
                out.push_str(" 'label:'label2-1");
            }
            Errors::SelfExpressionMissingNumberBeforeQuestionMark {
                position,
                expression,
            } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str(&format!("Can't compute expression {} \r\n", expression));
                out.push_str("You need to add a number before the `?`\r\n");
                out.push_str("Examples:\r\n");
                out.push_str(" self.15?0\r\n");
                out.push_str(" self.15?\r\n");
                out.push_str(" self.15..32?1\r\n");
                out.push_str(" self...32?3\r\n");
                out.push_str(" self.15..?");
            }
            Errors::SelfExpressionMissingNumber {
                position,
                expression,
            } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str(&format!("Can't compute expression {} \r\n", expression));
                out.push_str("You need to have a number after `self.`\r\n");
                out.push_str("Examples:\r\n");
                out.push_str(" self.15\r\n");
                out.push_str(" self.15?0\r\n");
                out.push_str(" self.15?\r\n");
                out.push_str(" self.15..32?1\r\n");
                out.push_str(" self...32?3\r\n");
                out.push_str(" self.15..?");
            }
            Errors::SelfExpressionYNotNumber {
                position,
                expression,
            } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str(&format!("Can't compute expression {} \r\n", expression));
                out.push_str("In a `self.x..y`, y must be a number\r\n");
                out.push_str("Examples:\r\n");
                out.push_str(" self.15\r\n");
                out.push_str(" self.15?0\r\n");
                out.push_str(" self.15?\r\n");
                out.push_str(" self.15..32?1\r\n");
                out.push_str(" self...32?3\r\n");
                out.push_str(" self.15..?");
            }
            Errors::SelfExpressionXNotNumber {
                position,
                expression,
            } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str(&format!("Can't compute expression {} \r\n", expression));
                out.push_str("In a `self.x..y`, x must be a number\r\n");
                out.push_str("Examples:\r\n");
                out.push_str(" self.15\r\n");
                out.push_str(" self.15?0\r\n");
                out.push_str(" self.15?\r\n");
                out.push_str(" self.15..32?1\r\n");
                out.push_str(" self...32?3\r\n");
                out.push_str(" self.15..?");
            }
            Errors::EqualNotPrecededByLitteral { position } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str("a `=` must be followed by a litteral\r\n");
                out.push_str("Example:\r\n");
                out.push_str(" var2 = (0 20 var1)\r\n");
                out.push_str("Please add a name for the variable or remove the =");
            }
            Errors::LiteralAfterAssignement {
                literal,
                assignement,
                position,
            } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str("Can't place a literal after an assignement\r\n");
                out.push_str("Have you forgotten parenthesis ?\r\n");
                out.push_str("Try to replace this:\r\n");
                out.push_str(&format!(" {} = {}\r\n", assignement, literal));
                out.push_str("By this:\r\n");
                out.push_str(&format!(" {} = ({})\r\n", assignement, literal));
                out.push_str("Or remove the `=`");
            }
            Errors::BlockAfterAssignement {
                assignement,
                position,
            } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str("Can't place a block after an assignement\r\n");
                out.push_str(&format!(" > {} = {{\r\n", &assignement));
                out.push_str("Try removing the `=`:\r\n");
                out.push_str(&format!(" {} {{\r\n", &assignement));
                out.push_str("Block example:\r\n");
                out.push_str("test {\r\n");
                out.push_str(" 0 5 6 self\r\n");
                out.push_str("}");
            }
            Errors::BlockMustBePrecededByLiteral { position } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str("A block must have a name\r\n");
                out.push_str("Please add a name to the block\r\n");
                out.push_str("Block example:\r\n");
                out.push_str("test {\r\n");
                out.push_str(" 0 5 6 self\r\n");
                out.push_str("}");
            }
            Errors::ParenthesisNotInAssignementOrFunctionCall { position } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str("A code function call must be preceded by a function name or add a = to make an assignement.\r\n");
                out.push_str("Example:\r\n");
                out.push_str(" test(0 1 26 var1)\r\n");
                out.push_str("Example:\r\n");
                out.push_str(" var1 = (0 1 26 var2)\r\n");
                out.push_str("Please add a litteral to make a function call, add a litteral and a `=` to make a variable assignement or remove both parenthesis");
            }
            Errors::AssignementFollowedByAnotherAssignement {
                assignement1,
                assignement2,
                position,
            } => {
                out.push_str(&position.to_str());
                out.push_str("\r\n");
                out.push_str("Can't place a assignement after an assignement\r\n");
                out.push_str(&format!(
                    " HERE > {} = {} =\r\n",
                    &assignement1, &assignement2
                ));
                out.push_str("Try to unwrap your statement\r\n");
                out.push_str(&format!(" {} = ...\r\n", assignement1));
                out.push_str(&format!(" {} = ...\r\n", assignement2));
            }
        }
        out
    }
}