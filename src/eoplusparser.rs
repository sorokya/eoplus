// Generated from eoplus-antlr4/EOPlusParser.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use super::eoplusparserlistener::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::lazy_static;
use antlr_rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{cast, cast_mut, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::*;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const MAIN: isize = 1;
pub const STATE: isize = 2;
pub const QUESTNAME: isize = 3;
pub const VERSION: isize = 4;
pub const DESC: isize = 5;
pub const ACTION: isize = 6;
pub const EORULE: isize = 7;
pub const GOTO: isize = 8;
pub const LPAREN: isize = 9;
pub const RPAREN: isize = 10;
pub const LBRACE: isize = 11;
pub const RBRACE: isize = 12;
pub const SEMI: isize = 13;
pub const COMMA: isize = 14;
pub const DOT: isize = 15;
pub const IntegerLiteral: isize = 16;
pub const StringLiteral: isize = 17;
pub const Identifier: isize = 18;
pub const WS: isize = 19;
pub const LINE_COMMENT: isize = 20;
pub const RULE_file: usize = 0;
pub const RULE_block: usize = 1;
pub const RULE_mainBlock: usize = 2;
pub const RULE_mainAttribute: usize = 3;
pub const RULE_nameAttribute: usize = 4;
pub const RULE_versionAttribute: usize = 5;
pub const RULE_version: usize = 6;
pub const RULE_stateBlock: usize = 7;
pub const RULE_statement: usize = 8;
pub const RULE_desc: usize = 9;
pub const RULE_action: usize = 10;
pub const RULE_eorule: usize = 11;
pub const RULE_argumentList: usize = 12;
pub const RULE_arguments: usize = 13;
pub const RULE_expression: usize = 14;
pub const RULE_literal: usize = 15;
pub const ruleNames: [&'static str; 16] = [
    "file",
    "block",
    "mainBlock",
    "mainAttribute",
    "nameAttribute",
    "versionAttribute",
    "version",
    "stateBlock",
    "statement",
    "desc",
    "action",
    "eorule",
    "argumentList",
    "arguments",
    "expression",
    "literal",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 16] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("'('"),
    Some("')'"),
    Some("'{'"),
    Some("'}'"),
    Some("';'"),
    Some("','"),
    Some("'.'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 21] = [
    None,
    Some("MAIN"),
    Some("STATE"),
    Some("QUESTNAME"),
    Some("VERSION"),
    Some("DESC"),
    Some("ACTION"),
    Some("EORULE"),
    Some("GOTO"),
    Some("LPAREN"),
    Some("RPAREN"),
    Some("LBRACE"),
    Some("RBRACE"),
    Some("SEMI"),
    Some("COMMA"),
    Some("DOT"),
    Some("IntegerLiteral"),
    Some("StringLiteral"),
    Some("Identifier"),
    Some("WS"),
    Some("LINE_COMMENT"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

type BaseParserType<'input, I> = BaseParser<
    'input,
    EOPlusParserExt<'input>,
    I,
    EOPlusParserContextType,
    dyn EOPlusParserListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type EOPlusParserTreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, EOPlusParserContextType, dyn EOPlusParserListener<'input> + 'a>;

/// Parser for EOPlusParser grammar
pub struct EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str {
        _serializedATN
    }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "3");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                EOPlusParserExt {
                    _pd: Default::default(),
                },
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> EOPlusParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> EOPlusParser<'input, I, DefaultErrorStrategy<'input, EOPlusParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for EOPlusParser
pub trait EOPlusParserContext<'input>:
    for<'x> Listenable<dyn EOPlusParserListener<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = EOPlusParserContextType>
{
}

antlr_rust::coerce_from! { 'input : EOPlusParserContext<'input> }

impl<'input> EOPlusParserContext<'input> for TerminalNode<'input, EOPlusParserContextType> {}
impl<'input> EOPlusParserContext<'input> for ErrorNode<'input, EOPlusParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn EOPlusParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn EOPlusParserListener<'input> + 'input }

pub struct EOPlusParserContextType;
antlr_rust::tid! {EOPlusParserContextType}

impl<'input> ParserNodeType<'input> for EOPlusParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn EOPlusParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct EOPlusParserExt<'input> {
    _pd: PhantomData<&'input str>,
}

impl<'input> EOPlusParserExt<'input> {}
antlr_rust::tid! { EOPlusParserExt<'a> }

impl<'input> TokenAware<'input> for EOPlusParserExt<'input> {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for EOPlusParserExt<'input>
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for EOPlusParserExt<'input>
{
    fn get_grammar_file_name(&self) -> &str {
        "EOPlusParser.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
}
//------------------- file ----------------
pub type FileContextAll<'input> = FileContext<'input>;

pub type FileContext<'input> = BaseParserRuleContext<'input, FileContextExt<'input>>;

#[derive(Clone)]
pub struct FileContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for FileContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a> for FileContext<'input> {
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_file(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_file(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for FileContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_file
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_file }
}
antlr_rust::tid! {FileContextExt<'a>}

impl<'input> FileContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FileContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FileContextExt { ph: PhantomData },
        ))
    }
}

pub trait FileContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<FileContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EOF, 0)
    }
    fn block_all(&self) -> Vec<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn block(&self, i: usize) -> Option<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> FileContextAttrs<'input> for FileContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn file(&mut self) -> Result<Rc<FileContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = FileContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_file);
        let mut _localctx: Rc<FileContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(35);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == MAIN || _la == STATE {
                    {
                        {
                            /*InvokeRule block*/
                            recog.base.set_state(32);
                            recog.block()?;
                        }
                    }
                    recog.base.set_state(37);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(38);
                recog.base.match_token(EOF, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- block ----------------
pub type BlockContextAll<'input> = BlockContext<'input>;

pub type BlockContext<'input> = BaseParserRuleContext<'input, BlockContextExt<'input>>;

#[derive(Clone)]
pub struct BlockContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for BlockContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a> for BlockContext<'input> {
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_block(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_block(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for BlockContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_block
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_block }
}
antlr_rust::tid! {BlockContextExt<'a>}

impl<'input> BlockContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<BlockContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            BlockContextExt { ph: PhantomData },
        ))
    }
}

pub trait BlockContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<BlockContextExt<'input>>
{
    fn mainBlock(&self) -> Option<Rc<MainBlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn stateBlock(&self) -> Option<Rc<StateBlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> BlockContextAttrs<'input> for BlockContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn block(&mut self) -> Result<Rc<BlockContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = BlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_block);
        let mut _localctx: Rc<BlockContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(42);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                MAIN => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule mainBlock*/
                        recog.base.set_state(40);
                        recog.mainBlock()?;
                    }
                }

                STATE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule stateBlock*/
                        recog.base.set_state(41);
                        recog.stateBlock()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- mainBlock ----------------
pub type MainBlockContextAll<'input> = MainBlockContext<'input>;

pub type MainBlockContext<'input> = BaseParserRuleContext<'input, MainBlockContextExt<'input>>;

#[derive(Clone)]
pub struct MainBlockContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for MainBlockContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a> for MainBlockContext<'input> {
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_mainBlock(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_mainBlock(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for MainBlockContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_mainBlock
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_mainBlock }
}
antlr_rust::tid! {MainBlockContextExt<'a>}

impl<'input> MainBlockContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<MainBlockContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            MainBlockContextExt { ph: PhantomData },
        ))
    }
}

pub trait MainBlockContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<MainBlockContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token MAIN
    /// Returns `None` if there is no child corresponding to token MAIN
    fn MAIN(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MAIN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    fn mainAttribute_all(&self) -> Vec<Rc<MainAttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn mainAttribute(&self, i: usize) -> Option<Rc<MainAttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> MainBlockContextAttrs<'input> for MainBlockContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn mainBlock(&mut self) -> Result<Rc<MainBlockContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = MainBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_mainBlock);
        let mut _localctx: Rc<MainBlockContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(44);
                recog.base.match_token(MAIN, &mut recog.err_handler)?;

                recog.base.set_state(45);
                recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                recog.base.set_state(49);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == QUESTNAME || _la == VERSION {
                    {
                        {
                            /*InvokeRule mainAttribute*/
                            recog.base.set_state(46);
                            recog.mainAttribute()?;
                        }
                    }
                    recog.base.set_state(51);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(52);
                recog.base.match_token(RBRACE, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- mainAttribute ----------------
pub type MainAttributeContextAll<'input> = MainAttributeContext<'input>;

pub type MainAttributeContext<'input> =
    BaseParserRuleContext<'input, MainAttributeContextExt<'input>>;

#[derive(Clone)]
pub struct MainAttributeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for MainAttributeContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a>
    for MainAttributeContext<'input>
{
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_mainAttribute(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_mainAttribute(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for MainAttributeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_mainAttribute
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_mainAttribute }
}
antlr_rust::tid! {MainAttributeContextExt<'a>}

impl<'input> MainAttributeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<MainAttributeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            MainAttributeContextExt { ph: PhantomData },
        ))
    }
}

pub trait MainAttributeContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<MainAttributeContextExt<'input>>
{
    fn nameAttribute(&self) -> Option<Rc<NameAttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn versionAttribute(&self) -> Option<Rc<VersionAttributeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> MainAttributeContextAttrs<'input> for MainAttributeContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn mainAttribute(&mut self) -> Result<Rc<MainAttributeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            MainAttributeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 6, RULE_mainAttribute);
        let mut _localctx: Rc<MainAttributeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(56);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                QUESTNAME => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule nameAttribute*/
                        recog.base.set_state(54);
                        recog.nameAttribute()?;
                    }
                }

                VERSION => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule versionAttribute*/
                        recog.base.set_state(55);
                        recog.versionAttribute()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- nameAttribute ----------------
pub type NameAttributeContextAll<'input> = NameAttributeContext<'input>;

pub type NameAttributeContext<'input> =
    BaseParserRuleContext<'input, NameAttributeContextExt<'input>>;

#[derive(Clone)]
pub struct NameAttributeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for NameAttributeContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a>
    for NameAttributeContext<'input>
{
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_nameAttribute(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_nameAttribute(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for NameAttributeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_nameAttribute
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_nameAttribute }
}
antlr_rust::tid! {NameAttributeContextExt<'a>}

impl<'input> NameAttributeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<NameAttributeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            NameAttributeContextExt { ph: PhantomData },
        ))
    }
}

pub trait NameAttributeContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<NameAttributeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token QUESTNAME
    /// Returns `None` if there is no child corresponding to token QUESTNAME
    fn QUESTNAME(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(QUESTNAME, 0)
    }
    /// Retrieves first TerminalNode corresponding to token StringLiteral
    /// Returns `None` if there is no child corresponding to token StringLiteral
    fn StringLiteral(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(StringLiteral, 0)
    }
}

impl<'input> NameAttributeContextAttrs<'input> for NameAttributeContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn nameAttribute(&mut self) -> Result<Rc<NameAttributeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            NameAttributeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 8, RULE_nameAttribute);
        let mut _localctx: Rc<NameAttributeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(58);
                recog.base.match_token(QUESTNAME, &mut recog.err_handler)?;

                recog.base.set_state(59);
                recog
                    .base
                    .match_token(StringLiteral, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- versionAttribute ----------------
pub type VersionAttributeContextAll<'input> = VersionAttributeContext<'input>;

pub type VersionAttributeContext<'input> =
    BaseParserRuleContext<'input, VersionAttributeContextExt<'input>>;

#[derive(Clone)]
pub struct VersionAttributeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for VersionAttributeContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a>
    for VersionAttributeContext<'input>
{
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_versionAttribute(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_versionAttribute(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for VersionAttributeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_versionAttribute
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_versionAttribute }
}
antlr_rust::tid! {VersionAttributeContextExt<'a>}

impl<'input> VersionAttributeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<VersionAttributeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            VersionAttributeContextExt { ph: PhantomData },
        ))
    }
}

pub trait VersionAttributeContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<VersionAttributeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token VERSION
    /// Returns `None` if there is no child corresponding to token VERSION
    fn VERSION(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(VERSION, 0)
    }
    fn version(&self) -> Option<Rc<VersionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> VersionAttributeContextAttrs<'input> for VersionAttributeContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn versionAttribute(
        &mut self,
    ) -> Result<Rc<VersionAttributeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            VersionAttributeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 10, RULE_versionAttribute);
        let mut _localctx: Rc<VersionAttributeContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(61);
                recog.base.match_token(VERSION, &mut recog.err_handler)?;

                /*InvokeRule version*/
                recog.base.set_state(62);
                recog.version()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- version ----------------
pub type VersionContextAll<'input> = VersionContext<'input>;

pub type VersionContext<'input> = BaseParserRuleContext<'input, VersionContextExt<'input>>;

#[derive(Clone)]
pub struct VersionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for VersionContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a> for VersionContext<'input> {
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_version(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_version(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for VersionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_version
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_version }
}
antlr_rust::tid! {VersionContextExt<'a>}

impl<'input> VersionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<VersionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            VersionContextExt { ph: PhantomData },
        ))
    }
}

pub trait VersionContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<VersionContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token IntegerLiteral in current rule
    fn IntegerLiteral_all(&self) -> Vec<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token IntegerLiteral, starting from 0.
    /// Returns `None` if number of children corresponding to token IntegerLiteral is less or equal than `i`.
    fn IntegerLiteral(&self, i: usize) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IntegerLiteral, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
    fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, i)
    }
}

impl<'input> VersionContextAttrs<'input> for VersionContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn version(&mut self) -> Result<Rc<VersionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = VersionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_version);
        let mut _localctx: Rc<VersionContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(64);
                recog
                    .base
                    .match_token(IntegerLiteral, &mut recog.err_handler)?;

                recog.base.set_state(69);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == DOT {
                    {
                        {
                            recog.base.set_state(65);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;

                            recog.base.set_state(66);
                            recog
                                .base
                                .match_token(IntegerLiteral, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(71);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- stateBlock ----------------
pub type StateBlockContextAll<'input> = StateBlockContext<'input>;

pub type StateBlockContext<'input> = BaseParserRuleContext<'input, StateBlockContextExt<'input>>;

#[derive(Clone)]
pub struct StateBlockContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for StateBlockContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a> for StateBlockContext<'input> {
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_stateBlock(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_stateBlock(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for StateBlockContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_stateBlock
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_stateBlock }
}
antlr_rust::tid! {StateBlockContextExt<'a>}

impl<'input> StateBlockContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<StateBlockContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            StateBlockContextExt { ph: PhantomData },
        ))
    }
}

pub trait StateBlockContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<StateBlockContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token STATE
    /// Returns `None` if there is no child corresponding to token STATE
    fn STATE(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STATE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LBRACE
    /// Returns `None` if there is no child corresponding to token LBRACE
    fn LBRACE(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LBRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RBRACE, 0)
    }
    fn statement_all(&self) -> Vec<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SEMI in current rule
    fn SEMI_all(&self) -> Vec<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SEMI, starting from 0.
    /// Returns `None` if number of children corresponding to token SEMI is less or equal than `i`.
    fn SEMI(&self, i: usize) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMI, i)
    }
}

impl<'input> StateBlockContextAttrs<'input> for StateBlockContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn stateBlock(&mut self) -> Result<Rc<StateBlockContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = StateBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 14, RULE_stateBlock);
        let mut _localctx: Rc<StateBlockContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(72);
                recog.base.match_token(STATE, &mut recog.err_handler)?;

                recog.base.set_state(73);
                recog.base.match_token(Identifier, &mut recog.err_handler)?;

                recog.base.set_state(74);
                recog.base.match_token(LBRACE, &mut recog.err_handler)?;

                recog.base.set_state(79);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while ((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << DESC)
                            | (1usize << ACTION)
                            | (1usize << EORULE)
                            | (1usize << SEMI)))
                        != 0
                {
                    {
                        recog.base.set_state(77);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.base.input.la(1) {
                            DESC | ACTION | EORULE => {
                                {
                                    /*InvokeRule statement*/
                                    recog.base.set_state(75);
                                    recog.statement()?;
                                }
                            }

                            SEMI => {
                                recog.base.set_state(76);
                                recog.base.match_token(SEMI, &mut recog.err_handler)?;
                            }

                            _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                &mut recog.base,
                            )))?,
                        }
                    }
                    recog.base.set_state(81);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(82);
                recog.base.match_token(RBRACE, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;

pub type StatementContext<'input> = BaseParserRuleContext<'input, StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for StatementContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a> for StatementContext<'input> {
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_statement(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_statement(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid! {StatementContextExt<'a>}

impl<'input> StatementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<StatementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            StatementContextExt { ph: PhantomData },
        ))
    }
}

pub trait StatementContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<StatementContextExt<'input>>
{
    fn desc(&self) -> Option<Rc<DescContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn action(&self) -> Option<Rc<ActionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn eorule(&self) -> Option<Rc<EoruleContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn statement(&mut self) -> Result<Rc<StatementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(87);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                DESC => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule desc*/
                        recog.base.set_state(84);
                        recog.desc()?;
                    }
                }

                ACTION => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule action*/
                        recog.base.set_state(85);
                        recog.action()?;
                    }
                }

                EORULE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule eorule*/
                        recog.base.set_state(86);
                        recog.eorule()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- desc ----------------
pub type DescContextAll<'input> = DescContext<'input>;

pub type DescContext<'input> = BaseParserRuleContext<'input, DescContextExt<'input>>;

#[derive(Clone)]
pub struct DescContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for DescContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a> for DescContext<'input> {
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_desc(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_desc(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for DescContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_desc
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_desc }
}
antlr_rust::tid! {DescContextExt<'a>}

impl<'input> DescContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DescContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            DescContextExt { ph: PhantomData },
        ))
    }
}

pub trait DescContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<DescContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token DESC
    /// Returns `None` if there is no child corresponding to token DESC
    fn DESC(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DESC, 0)
    }
    /// Retrieves first TerminalNode corresponding to token StringLiteral
    /// Returns `None` if there is no child corresponding to token StringLiteral
    fn StringLiteral(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(StringLiteral, 0)
    }
}

impl<'input> DescContextAttrs<'input> for DescContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn desc(&mut self) -> Result<Rc<DescContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = DescContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_desc);
        let mut _localctx: Rc<DescContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(89);
                recog.base.match_token(DESC, &mut recog.err_handler)?;

                recog.base.set_state(90);
                recog
                    .base
                    .match_token(StringLiteral, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- action ----------------
pub type ActionContextAll<'input> = ActionContext<'input>;

pub type ActionContext<'input> = BaseParserRuleContext<'input, ActionContextExt<'input>>;

#[derive(Clone)]
pub struct ActionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for ActionContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a> for ActionContext<'input> {
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_action(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_action(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for ActionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_action
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_action }
}
antlr_rust::tid! {ActionContextExt<'a>}

impl<'input> ActionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ActionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ActionContextExt { ph: PhantomData },
        ))
    }
}

pub trait ActionContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<ActionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ACTION
    /// Returns `None` if there is no child corresponding to token ACTION
    fn ACTION(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ACTION, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
    fn argumentList(&self) -> Option<Rc<ArgumentListContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ActionContextAttrs<'input> for ActionContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn action(&mut self) -> Result<Rc<ActionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_action);
        let mut _localctx: Rc<ActionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(92);
                recog.base.match_token(ACTION, &mut recog.err_handler)?;

                recog.base.set_state(93);
                recog.base.match_token(Identifier, &mut recog.err_handler)?;

                /*InvokeRule argumentList*/
                recog.base.set_state(94);
                recog.argumentList()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- eorule ----------------
pub type EoruleContextAll<'input> = EoruleContext<'input>;

pub type EoruleContext<'input> = BaseParserRuleContext<'input, EoruleContextExt<'input>>;

#[derive(Clone)]
pub struct EoruleContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for EoruleContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a> for EoruleContext<'input> {
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_eorule(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_eorule(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for EoruleContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_eorule
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_eorule }
}
antlr_rust::tid! {EoruleContextExt<'a>}

impl<'input> EoruleContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<EoruleContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            EoruleContextExt { ph: PhantomData },
        ))
    }
}

pub trait EoruleContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<EoruleContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token EORULE
    /// Returns `None` if there is no child corresponding to token EORULE
    fn EORULE(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EORULE, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
    fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
    /// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
    fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, i)
    }
    fn argumentList(&self) -> Option<Rc<ArgumentListContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token GOTO
    /// Returns `None` if there is no child corresponding to token GOTO
    fn GOTO(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GOTO, 0)
    }
}

impl<'input> EoruleContextAttrs<'input> for EoruleContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn eorule(&mut self) -> Result<Rc<EoruleContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = EoruleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_eorule);
        let mut _localctx: Rc<EoruleContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(96);
                recog.base.match_token(EORULE, &mut recog.err_handler)?;

                recog.base.set_state(97);
                recog.base.match_token(Identifier, &mut recog.err_handler)?;

                /*InvokeRule argumentList*/
                recog.base.set_state(98);
                recog.argumentList()?;

                recog.base.set_state(99);
                recog.base.match_token(GOTO, &mut recog.err_handler)?;

                recog.base.set_state(100);
                recog.base.match_token(Identifier, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- argumentList ----------------
pub type ArgumentListContextAll<'input> = ArgumentListContext<'input>;

pub type ArgumentListContext<'input> =
    BaseParserRuleContext<'input, ArgumentListContextExt<'input>>;

#[derive(Clone)]
pub struct ArgumentListContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for ArgumentListContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a> for ArgumentListContext<'input> {
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_argumentList(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_argumentList(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for ArgumentListContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_argumentList
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_argumentList }
}
antlr_rust::tid! {ArgumentListContextExt<'a>}

impl<'input> ArgumentListContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ArgumentListContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ArgumentListContextExt { ph: PhantomData },
        ))
    }
}

pub trait ArgumentListContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<ArgumentListContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
    fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ArgumentListContextAttrs<'input> for ArgumentListContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn argumentList(&mut self) -> Result<Rc<ArgumentListContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ArgumentListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 24, RULE_argumentList);
        let mut _localctx: Rc<ArgumentListContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(102);
                recog.base.match_token(LPAREN, &mut recog.err_handler)?;

                recog.base.set_state(104);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == IntegerLiteral || _la == StringLiteral {
                    {
                        /*InvokeRule arguments*/
                        recog.base.set_state(103);
                        recog.arguments()?;
                    }
                }

                recog.base.set_state(106);
                recog.base.match_token(RPAREN, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- arguments ----------------
pub type ArgumentsContextAll<'input> = ArgumentsContext<'input>;

pub type ArgumentsContext<'input> = BaseParserRuleContext<'input, ArgumentsContextExt<'input>>;

#[derive(Clone)]
pub struct ArgumentsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for ArgumentsContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a> for ArgumentsContext<'input> {
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_arguments(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_arguments(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for ArgumentsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_arguments
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_arguments }
}
antlr_rust::tid! {ArgumentsContextExt<'a>}

impl<'input> ArgumentsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ArgumentsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ArgumentsContextExt { ph: PhantomData },
        ))
    }
}

pub trait ArgumentsContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<ArgumentsContextExt<'input>>
{
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> ArgumentsContextAttrs<'input> for ArgumentsContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn arguments(&mut self) -> Result<Rc<ArgumentsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ArgumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_arguments);
        let mut _localctx: Rc<ArgumentsContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule expression*/
                recog.base.set_state(108);
                recog.expression()?;

                recog.base.set_state(113);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(109);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule expression*/
                            recog.base.set_state(110);
                            recog.expression()?;
                        }
                    }
                    recog.base.set_state(115);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;

pub type ExpressionContext<'input> = BaseParserRuleContext<'input, ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for ExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a> for ExpressionContext<'input> {
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expression(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_expression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid! {ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait ExpressionContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>
{
    fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn expression(&mut self) -> Result<Rc<ExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 28, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule literal*/
                recog.base.set_state(116);
                recog.literal()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- literal ----------------
pub type LiteralContextAll<'input> = LiteralContext<'input>;

pub type LiteralContext<'input> = BaseParserRuleContext<'input, LiteralContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> EOPlusParserContext<'input> for LiteralContext<'input> {}

impl<'input, 'a> Listenable<dyn EOPlusParserListener<'input> + 'a> for LiteralContext<'input> {
    fn enter(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_literal(self);
    }
    fn exit(&self, listener: &mut (dyn EOPlusParserListener<'input> + 'a)) {
        listener.exit_literal(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for LiteralContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = EOPlusParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}
antlr_rust::tid! {LiteralContextExt<'a>}

impl<'input> LiteralContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn EOPlusParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<LiteralContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            LiteralContextExt { ph: PhantomData },
        ))
    }
}

pub trait LiteralContextAttrs<'input>:
    EOPlusParserContext<'input> + BorrowMut<LiteralContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token IntegerLiteral
    /// Returns `None` if there is no child corresponding to token IntegerLiteral
    fn IntegerLiteral(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IntegerLiteral, 0)
    }
    /// Retrieves first TerminalNode corresponding to token StringLiteral
    /// Returns `None` if there is no child corresponding to token StringLiteral
    fn StringLiteral(&self) -> Option<Rc<TerminalNode<'input, EOPlusParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(StringLiteral, 0)
    }
}

impl<'input> LiteralContextAttrs<'input> for LiteralContext<'input> {}

impl<'input, I, H> EOPlusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn literal(&mut self) -> Result<Rc<LiteralContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_literal);
        let mut _localctx: Rc<LiteralContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(118);
                _la = recog.base.input.la(1);
                if { !(_la == IntegerLiteral || _la == StringLiteral) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x16\x7b\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\x0e\
	\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x03\x02\x07\x02\x24\x0a\
	\x02\x0c\x02\x0e\x02\x27\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\x05\x03\
	\x2d\x0a\x03\x03\x04\x03\x04\x03\x04\x07\x04\x32\x0a\x04\x0c\x04\x0e\x04\
	\x35\x0b\x04\x03\x04\x03\x04\x03\x05\x03\x05\x05\x05\x3b\x0a\x05\x03\x06\
	\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\x08\x03\x08\x03\x08\x07\x08\
	\x46\x0a\x08\x0c\x08\x0e\x08\x49\x0b\x08\x03\x09\x03\x09\x03\x09\x03\x09\
	\x03\x09\x07\x09\x50\x0a\x09\x0c\x09\x0e\x09\x53\x0b\x09\x03\x09\x03\x09\
	\x03\x0a\x03\x0a\x03\x0a\x05\x0a\x5a\x0a\x0a\x03\x0b\x03\x0b\x03\x0b\x03\
	\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\
	\x0d\x03\x0e\x03\x0e\x05\x0e\x6b\x0a\x0e\x03\x0e\x03\x0e\x03\x0f\x03\x0f\
	\x03\x0f\x07\x0f\x72\x0a\x0f\x0c\x0f\x0e\x0f\x75\x0b\x0f\x03\x10\x03\x10\
	\x03\x11\x03\x11\x03\x11\x02\x02\x12\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\
	\x14\x16\x18\x1a\x1c\x1e\x20\x02\x03\x03\x02\x12\x13\x02\x75\x02\x25\x03\
	\x02\x02\x02\x04\x2c\x03\x02\x02\x02\x06\x2e\x03\x02\x02\x02\x08\x3a\x03\
	\x02\x02\x02\x0a\x3c\x03\x02\x02\x02\x0c\x3f\x03\x02\x02\x02\x0e\x42\x03\
	\x02\x02\x02\x10\x4a\x03\x02\x02\x02\x12\x59\x03\x02\x02\x02\x14\x5b\x03\
	\x02\x02\x02\x16\x5e\x03\x02\x02\x02\x18\x62\x03\x02\x02\x02\x1a\x68\x03\
	\x02\x02\x02\x1c\x6e\x03\x02\x02\x02\x1e\x76\x03\x02\x02\x02\x20\x78\x03\
	\x02\x02\x02\x22\x24\x05\x04\x03\x02\x23\x22\x03\x02\x02\x02\x24\x27\x03\
	\x02\x02\x02\x25\x23\x03\x02\x02\x02\x25\x26\x03\x02\x02\x02\x26\x28\x03\
	\x02\x02\x02\x27\x25\x03\x02\x02\x02\x28\x29\x07\x02\x02\x03\x29\x03\x03\
	\x02\x02\x02\x2a\x2d\x05\x06\x04\x02\x2b\x2d\x05\x10\x09\x02\x2c\x2a\x03\
	\x02\x02\x02\x2c\x2b\x03\x02\x02\x02\x2d\x05\x03\x02\x02\x02\x2e\x2f\x07\
	\x03\x02\x02\x2f\x33\x07\x0d\x02\x02\x30\x32\x05\x08\x05\x02\x31\x30\x03\
	\x02\x02\x02\x32\x35\x03\x02\x02\x02\x33\x31\x03\x02\x02\x02\x33\x34\x03\
	\x02\x02\x02\x34\x36\x03\x02\x02\x02\x35\x33\x03\x02\x02\x02\x36\x37\x07\
	\x0e\x02\x02\x37\x07\x03\x02\x02\x02\x38\x3b\x05\x0a\x06\x02\x39\x3b\x05\
	\x0c\x07\x02\x3a\x38\x03\x02\x02\x02\x3a\x39\x03\x02\x02\x02\x3b\x09\x03\
	\x02\x02\x02\x3c\x3d\x07\x05\x02\x02\x3d\x3e\x07\x13\x02\x02\x3e\x0b\x03\
	\x02\x02\x02\x3f\x40\x07\x06\x02\x02\x40\x41\x05\x0e\x08\x02\x41\x0d\x03\
	\x02\x02\x02\x42\x47\x07\x12\x02\x02\x43\x44\x07\x11\x02\x02\x44\x46\x07\
	\x12\x02\x02\x45\x43\x03\x02\x02\x02\x46\x49\x03\x02\x02\x02\x47\x45\x03\
	\x02\x02\x02\x47\x48\x03\x02\x02\x02\x48\x0f\x03\x02\x02\x02\x49\x47\x03\
	\x02\x02\x02\x4a\x4b\x07\x04\x02\x02\x4b\x4c\x07\x14\x02\x02\x4c\x51\x07\
	\x0d\x02\x02\x4d\x50\x05\x12\x0a\x02\x4e\x50\x07\x0f\x02\x02\x4f\x4d\x03\
	\x02\x02\x02\x4f\x4e\x03\x02\x02\x02\x50\x53\x03\x02\x02\x02\x51\x4f\x03\
	\x02\x02\x02\x51\x52\x03\x02\x02\x02\x52\x54\x03\x02\x02\x02\x53\x51\x03\
	\x02\x02\x02\x54\x55\x07\x0e\x02\x02\x55\x11\x03\x02\x02\x02\x56\x5a\x05\
	\x14\x0b\x02\x57\x5a\x05\x16\x0c\x02\x58\x5a\x05\x18\x0d\x02\x59\x56\x03\
	\x02\x02\x02\x59\x57\x03\x02\x02\x02\x59\x58\x03\x02\x02\x02\x5a\x13\x03\
	\x02\x02\x02\x5b\x5c\x07\x07\x02\x02\x5c\x5d\x07\x13\x02\x02\x5d\x15\x03\
	\x02\x02\x02\x5e\x5f\x07\x08\x02\x02\x5f\x60\x07\x14\x02\x02\x60\x61\x05\
	\x1a\x0e\x02\x61\x17\x03\x02\x02\x02\x62\x63\x07\x09\x02\x02\x63\x64\x07\
	\x14\x02\x02\x64\x65\x05\x1a\x0e\x02\x65\x66\x07\x0a\x02\x02\x66\x67\x07\
	\x14\x02\x02\x67\x19\x03\x02\x02\x02\x68\x6a\x07\x0b\x02\x02\x69\x6b\x05\
	\x1c\x0f\x02\x6a\x69\x03\x02\x02\x02\x6a\x6b\x03\x02\x02\x02\x6b\x6c\x03\
	\x02\x02\x02\x6c\x6d\x07\x0c\x02\x02\x6d\x1b\x03\x02\x02\x02\x6e\x73\x05\
	\x1e\x10\x02\x6f\x70\x07\x10\x02\x02\x70\x72\x05\x1e\x10\x02\x71\x6f\x03\
	\x02\x02\x02\x72\x75\x03\x02\x02\x02\x73\x71\x03\x02\x02\x02\x73\x74\x03\
	\x02\x02\x02\x74\x1d\x03\x02\x02\x02\x75\x73\x03\x02\x02\x02\x76\x77\x05\
	\x20\x11\x02\x77\x1f\x03\x02\x02\x02\x78\x79\x09\x02\x02\x02\x79\x21\x03\
	\x02\x02\x02\x0c\x25\x2c\x33\x3a\x47\x4f\x51\x59\x6a\x73";
