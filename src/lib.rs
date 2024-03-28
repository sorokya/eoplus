mod eopluslexer;
mod eoplusparser;
mod eoplusparserlistener;

use crate::{eopluslexer::EOPlusLexer, eoplusparser::EOPlusParser};

use antlr_rust::{
    common_token_stream::CommonTokenStream,
    errors::ANTLRError,
    parser_rule_context::ParserRuleContext,
    tree::{ParseTree, ParseTreeListener, Tree},
    InputStream,
};
use eoplusparser::{EOPlusParserContext, EOPlusParserContextType};
use eoplusparserlistener::EOPlusParserListener;

#[derive(Debug, Default)]
pub struct Quest {
    pub name: String,
    pub version: String,
    pub states: Vec<State>,
}

#[derive(Clone, Debug, Default)]
pub struct State {
    pub name: String,
    pub description: String,
    pub actions: Vec<Action>,
    pub rules: Vec<Rule>,
}

#[derive(Clone, Debug, Default)]
pub struct Action {
    pub name: String,
    pub args: Vec<Arg>,
}

#[derive(Clone, Debug, Default)]
pub struct Rule {
    pub name: String,
    pub args: Vec<Arg>,
    pub goto: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arg {
    Int(i32),
    Str(String),
}

#[derive(Default)]
struct Listener {
    pub quest: Quest,
    current_state: State,
    current_action: Option<Action>,
    current_rule: Option<Rule>,
}

impl Listener {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'input> ParseTreeListener<'input, EOPlusParserContextType> for Listener {
    fn enter_every_rule(&mut self, _ctx: &dyn EOPlusParserContext<'input>) {}
}

impl<'input> EOPlusParserListener<'input> for Listener {
    fn enter_file(&mut self, _ctx: &eoplusparser::FileContext<'input>) {}

    fn exit_file(&mut self, _ctx: &eoplusparser::FileContext<'input>) {}

    fn enter_block(&mut self, _ctx: &eoplusparser::BlockContext<'input>) {}

    fn exit_block(&mut self, _ctx: &eoplusparser::BlockContext<'input>) {}

    fn enter_mainBlock(&mut self, _ctx: &eoplusparser::MainBlockContext<'input>) {}

    fn exit_mainBlock(&mut self, _ctx: &eoplusparser::MainBlockContext<'input>) {}

    fn enter_mainAttribute(&mut self, _ctx: &eoplusparser::MainAttributeContext<'input>) {}

    fn exit_mainAttribute(&mut self, _ctx: &eoplusparser::MainAttributeContext<'input>) {}

    fn enter_nameAttribute(&mut self, _ctx: &eoplusparser::NameAttributeContext<'input>) {}

    fn exit_nameAttribute(&mut self, ctx: &eoplusparser::NameAttributeContext<'input>) {
        if let Some(token) = ctx.get_token(eopluslexer::StringLiteral, 0) {
            self.quest.name = token.get_text().replace("\"", "");
        }
    }

    fn enter_versionAttribute(&mut self, _ctx: &eoplusparser::VersionAttributeContext<'input>) {}

    fn exit_versionAttribute(&mut self, _ctx: &eoplusparser::VersionAttributeContext<'input>) {}

    fn enter_version(&mut self, _ctx: &eoplusparser::VersionContext<'input>) {}

    fn exit_version(&mut self, ctx: &eoplusparser::VersionContext<'input>) {
        for child in ctx.get_children() {
            self.quest.version.push_str(&child.get_text());
        }
    }

    fn enter_stateBlock(&mut self, _ctx: &eoplusparser::StateBlockContext<'input>) {
        self.current_state = State::default();
    }

    fn exit_stateBlock(&mut self, ctx: &eoplusparser::StateBlockContext<'input>) {
        if let Some(token) = ctx.get_token(eopluslexer::Identifier, 0) {
            self.current_state.name = token.get_text();
        }
        self.quest.states.push(self.current_state.clone());
    }

    fn enter_statement(&mut self, _ctx: &eoplusparser::StatementContext<'input>) {}

    fn exit_statement(&mut self, _ctx: &eoplusparser::StatementContext<'input>) {}

    fn enter_desc(&mut self, _ctx: &eoplusparser::DescContext<'input>) {}

    fn exit_desc(&mut self, ctx: &eoplusparser::DescContext<'input>) {
        if let Some(token) = ctx.get_token(eopluslexer::StringLiteral, 0) {
            self.current_state.description = token.get_text().replace("\"", "");
        }
    }

    fn enter_action(&mut self, _ctx: &eoplusparser::ActionContext<'input>) {
        self.current_action = Some(Action::default());
    }

    fn exit_action(&mut self, ctx: &eoplusparser::ActionContext<'input>) {
        let action = self.current_action.as_mut().unwrap();
        if let Some(token) = ctx.get_token(eopluslexer::Identifier, 0) {
            action.name = token.get_text();
        }

        self.current_state.actions.push(action.clone());
        self.current_action = None;
    }

    fn enter_eorule(&mut self, _ctx: &eoplusparser::EoruleContext<'input>) {
        self.current_rule = Some(Rule::default());
    }

    fn exit_eorule(&mut self, ctx: &eoplusparser::EoruleContext<'input>) {
        let rule = self.current_rule.as_mut().unwrap();
        if let Some(token) = ctx.get_token(eopluslexer::Identifier, 0) {
            rule.name = token.get_text();
        }

        if let Some(token) = ctx.get_token(eopluslexer::Identifier, 1) {
            rule.goto = token.get_text();
        }

        self.current_state.rules.push(rule.clone());
        self.current_rule = None;
    }

    fn enter_argumentList(&mut self, _ctx: &eoplusparser::ArgumentListContext<'input>) {}

    fn exit_argumentList(&mut self, _ctx: &eoplusparser::ArgumentListContext<'input>) {}

    fn enter_arguments(&mut self, _ctx: &eoplusparser::ArgumentsContext<'input>) {}

    fn exit_arguments(&mut self, _ctx: &eoplusparser::ArgumentsContext<'input>) {}

    fn enter_expression(&mut self, _ctx: &eoplusparser::ExpressionContext<'input>) {}

    fn exit_expression(&mut self, ctx: &eoplusparser::ExpressionContext<'input>) {
        let expression = ctx.get_text();
        let args = match self.current_rule.as_mut() {
            Some(rule) => &mut rule.args,
            None => &mut self.current_action.as_mut().unwrap().args,
        };

        match expression.parse::<i32>() {
            Ok(int) => args.push(Arg::Int(int)),
            Err(_) => args.push(Arg::Str(expression.replace("\"", ""))),
        }
    }

    fn enter_literal(&mut self, _ctx: &eoplusparser::LiteralContext<'input>) {}

    fn exit_literal(&mut self, _ctx: &eoplusparser::LiteralContext<'input>) {}
}

pub fn parse_quest(input: &str) -> Result<Quest, ANTLRError> {
    let lexer = EOPlusLexer::new(InputStream::new(input));
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = EOPlusParser::new(token_source);

    let listener_id = parser.add_parse_listener(Box::new(Listener::new()));

    parser.file()?;

    let listener = parser.remove_parse_listener(listener_id);
    Ok(listener.quest)
}
