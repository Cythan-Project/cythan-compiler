pub enum Errors {
    FunctionNotFound {
        function_name: String,
    },
    VariableNotFound {
        variable_name: String,
    },
    LabelNotFound {
        label_name: String,
    },
    EmptyVariable {
        varname: String,
    },
    UndefinedVariable {
        varname: String,
    },
    UnableToReadLitteral {
        litteral: String,
    },
    SelfExpressionMissingNumberBeforeQuestionMark {
        expression: String,
    },
    SelfExpressionMissingNumber {
        expression: String,
    },
    SelfExpressionYNotNumber {
        expression: String,
    },
    SelfExpressionXNotNumber {
        expression: String,
    },
    EqualNotPrecededByLitteral,
    LiteralAfterAssignement {
        literal: String,
        assignement: String,
    },
    BlockAfterAssignement {
        assignement: String,
    },
    BlockMustBePrecededByLiteral,
    ParenthesisNotInAssignementOrFunctionCall,
    AssignementFollowedByAnotherAssignement {
        assignement1: String,
        assignement2: String,
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
            Errors::FunctionNotFound { function_name } => {
                out.push_str(&format!("Function `{}` was not found. \r\n", function_name));
                out.push_str(&format!(
                    " - Verify that the function is created before used.\r\n"
                ));
                out.push_str(&format!(" - Check for similar names.\r\n"));
                out.push_str(&format!(" - Create the function using:\r\n"));
                out.push_str(&format!(" fn {} {{\r\n", function_name));
                out.push_str(&format!("    <code>\r\n"));
                out.push_str(&format!(" }}"));
            }
            Errors::VariableNotFound { variable_name } => {
                out.push_str(&format!("Variable `{}` was not found. \r\n", variable_name));
                out.push_str(&format!("Try to init your variable with a value:\r\n"));
                out.push_str(&format!(" {} = (<value>)\r\n", variable_name));
                out.push_str(&format!("Or check if:\r\n"));
                out.push_str(&format!(" - The name is correct\r\n"));
                out.push_str(&format!(" - This is not normally a label\r\n"));
                out.push_str(&format!("  Use: `'{}` instead\r\n", variable_name));
                out.push_str(&format!(" - This is not normally a function\r\n"));
                out.push_str(&format!("  Use: `{}()` instead", variable_name));
            }
            Errors::LabelNotFound { label_name } => {
                out.push_str(&format!("Label `'{}` was not found. \r\n", label_name));
                out.push_str(&format!("Try to init your label at an index:\r\n"));
                out.push_str(&format!(" '{}:<case value>\r\n", label_name));
                out.push_str(&format!("Or check if:\r\n"));
                out.push_str(&format!(" - The name is correct\r\n"));
                out.push_str(&format!(" - The label is declared when this runs\r\n"));
                out.push_str(&format!(" - The label is owned by a function\r\n"));
                out.push_str(&format!("  Use: `'#{}` instead\r\n", label_name));
                out.push_str(&format!(" - The label is not global\r\n"));
                out.push_str(&format!("  Use: `'#{}` instead", label_name));
            }
            Errors::EmptyVariable { varname } => {
                out.push_str(&format!("Variable `{}` is empty \r\n", varname));
                out.push_str(&format!("Try to init your variable with a value:\r\n"));
                out.push_str(&format!(" {} = (<value>)", varname));
            }
            Errors::UndefinedVariable { varname } => {
                out.push_str(&format!("Variable `{}` is undefined \r\n", varname));
                out.push_str(&format!("Try to init your with a value:\r\n"));
                out.push_str(&format!(" {} = (<value>)", varname));
            }
            Errors::UnableToReadLitteral { litteral } => {
                out.push_str(&format!("Can't compile litteral {} \r\n", litteral));
                out.push_str(&format!("Is your synthax correct ?\r\n"));
                out.push_str(&format!("Examples of litterals:\r\n"));
                out.push_str(&format!(" 0 \r\n"));
                out.push_str(&format!(" ~-1 \r\n"));
                out.push_str(&format!(" self.0..10?9 \r\n"));
                out.push_str(&format!(" 'label:var1 \r\n"));
                out.push_str(&format!(" 'label:'label2-1"));
            }
            Errors::SelfExpressionMissingNumberBeforeQuestionMark { expression } => {
                out.push_str(&format!("Can't compute expression {} \r\n", expression));
                out.push_str(&format!("You need to add a number before the `?`\r\n"));
                out.push_str(&format!("Examples:\r\n"));
                out.push_str(&format!(" self.15?0\r\n"));
                out.push_str(&format!(" self.15?\r\n"));
                out.push_str(&format!(" self.15..32?1\r\n"));
                out.push_str(&format!(" self...32?3\r\n"));
                out.push_str(&format!(" self.15..?"));
            }
            Errors::SelfExpressionMissingNumber { expression } => {
                out.push_str(&format!("Can't compute expression {} \r\n", expression));
                out.push_str(&format!("You need to have a number after `self.`\r\n"));
                out.push_str(&format!("Examples:\r\n"));
                out.push_str(&format!(" self.15\r\n"));
                out.push_str(&format!(" self.15?0\r\n"));
                out.push_str(&format!(" self.15?\r\n"));
                out.push_str(&format!(" self.15..32?1\r\n"));
                out.push_str(&format!(" self...32?3\r\n"));
                out.push_str(&format!(" self.15..?"));
            }
            Errors::SelfExpressionYNotNumber { expression } => {
                out.push_str(&format!("Can't compute expression {} \r\n", expression));
                out.push_str(&format!("In a `self.x..y`, y must be a number\r\n"));
                out.push_str(&format!("Examples:\r\n"));
                out.push_str(&format!(" self.15\r\n"));
                out.push_str(&format!(" self.15?0\r\n"));
                out.push_str(&format!(" self.15?\r\n"));
                out.push_str(&format!(" self.15..32?1\r\n"));
                out.push_str(&format!(" self...32?3\r\n"));
                out.push_str(&format!(" self.15..?"));
            }
            Errors::SelfExpressionXNotNumber { expression } => {
                out.push_str(&format!("Can't compute expression {} \r\n", expression));
                out.push_str(&format!("In a `self.x..y`, x must be a number\r\n"));
                out.push_str(&format!("Examples:\r\n"));
                out.push_str(&format!(" self.15\r\n"));
                out.push_str(&format!(" self.15?0\r\n"));
                out.push_str(&format!(" self.15?\r\n"));
                out.push_str(&format!(" self.15..32?1\r\n"));
                out.push_str(&format!(" self...32?3\r\n"));
                out.push_str(&format!(" self.15..?"));
            }
            Errors::EqualNotPrecededByLitteral => {
                out.push_str(&format!("a `=` must be followed by a litteral\r\n"));
                out.push_str(&format!("Example:\r\n"));
                out.push_str(&format!(" var2 = (0 20 var1)\r\n"));
                out.push_str(&format!(
                    "Please add a name for the variable or remove the ="
                ));
            }
            Errors::LiteralAfterAssignement {
                literal,
                assignement,
            } => {
                out.push_str(&format!("Can't place a literal after an assignement"));
                out.push_str(&format!("Have you forgotten parenthesis ?"));
                out.push_str(&format!("Try to replace this:"));
                out.push_str(&format!(" {} = {}", assignement, literal));
                out.push_str(&format!("By this:"));
                out.push_str(&format!(" {} = ({})", assignement, literal));
                out.push_str(&format!("Or remove the `=`"));
            }
            Errors::BlockAfterAssignement { assignement } => {
                out.push_str(&format!("Can't place a block after an assignement"));
                out.push_str(&format!(" > {} = {{", &assignement));
                out.push_str(&format!("Try removing the `=`:"));
                out.push_str(&format!(" {} {{", &assignement));
                out.push_str(&format!("Block example:"));
                out.push_str(&format!("test {{"));
                out.push_str(&format!(" 0 5 6 self"));
                out.push_str(&format!("}}"));
            }
            Errors::BlockMustBePrecededByLiteral {} => {
                out.push_str(&format!("A block must have a name"));
                out.push_str(&format!("Please add a name to the block"));
                out.push_str(&format!("Block example:"));
                out.push_str(&format!("test {{"));
                out.push_str(&format!(" 0 5 6 self"));
                out.push_str(&format!("}}"));
            }
            Errors::ParenthesisNotInAssignementOrFunctionCall {} => {
                out.push_str(&format!("A code function call must be preceded by a function name or add a = to make an assignement."));
                out.push_str(&format!("Example:"));
                out.push_str(&format!(" test(0 1 26 var1)"));
                out.push_str(&format!("Example:"));
                out.push_str(&format!(" var1 = (0 1 26 var2)"));
                out.push_str(&format!("Please add a litteral to make a function call, add a litteral and a `=` to make a variable assignement or remove both parenthesis"));
            }
            Errors::AssignementFollowedByAnotherAssignement {
                assignement1,
                assignement2,
            } => {
                out.push_str(&format!("Can't place a assignement after an assignement"));
                out.push_str(&format!(" HERE > {} = {} =", &assignement1, &assignement2));
                out.push_str(&format!("Try to unwrap your statement"));
                out.push_str(&format!(" {} = ...", assignement1));
                out.push_str(&format!(" {} = ...", assignement2));
            }
        }
        out
    }
}

/*

                    out.push_str(&format!("Can't place a block after an assignement");
                    out.push_str(&format!(" HERE > {}", &litteral);
                    out.push_str(&format!("Have you forgot parenthesis ?");
                    out.push_str(&format!(" var1 = (10)");
                    out.push_str(&format!("Or added a = in your function definition");
                    out.push_str(&format!(" fn testfunc {{");
                    out.push_str(&format!("    10 20 10");
                    out.push_str(&format!(" }}");
                    was_assignement = false;
                }
                if was_litteral {
                    output.push(Stage3Token::FunctionCreation(litteral, compile(e)));
                    litteral = String::new();
                    was_litteral = false;
                } else {
                    out.push_str(&format!("A code block must be preceded by a litteral.");
                    out.push_str(&format!("Example:");
                    out.push_str(&format!(" test {{");
                    out.push_str(&format!(" 0 5 6 self");
                    out.push_str(&format!(" }}");
                    out.push_str(&format!("Please add a litteral to create a function or remove the block");
*/
