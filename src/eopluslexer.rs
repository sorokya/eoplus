// Generated from eoplus-antlr4/EOPlusLexer.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::atn::ATN;
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::char_stream::CharStream;
use antlr_rust::dfa::DFA;
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::lexer_atn_simulator::{ILexerATNSimulator, LexerATNSimulator};
use antlr_rust::parser_rule_context::{cast, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, EmptyContext, EmptyCustomRuleContext};
use antlr_rust::token::*;
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;

use antlr_rust::{lazy_static, Tid, TidAble, TidExt};

use std::cell::RefCell;
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
pub const channelNames: [&'static str; 0 + 2] = ["DEFAULT_TOKEN_CHANNEL", "HIDDEN"];

pub const modeNames: [&'static str; 1] = ["DEFAULT_MODE"];

pub const ruleNames: [&'static str; 48] = [
    "MAIN",
    "STATE",
    "QUESTNAME",
    "VERSION",
    "DESC",
    "ACTION",
    "EORULE",
    "GOTO",
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    "LPAREN",
    "RPAREN",
    "LBRACE",
    "RBRACE",
    "SEMI",
    "COMMA",
    "DOT",
    "IntegerLiteral",
    "StringLiteral",
    "StringCharacter",
    "EscapeSequence",
    "Identifier",
    "WS",
    "LINE_COMMENT",
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

pub type LexerContext<'input> =
    BaseRuleContext<'input, EmptyCustomRuleContext<'input, LocalTokenFactory<'input>>>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a>>::From;

pub struct EOPlusLexer<'input, Input: CharStream<From<'input>>> {
    base: BaseLexer<'input, EOPlusLexerActions, Input, LocalTokenFactory<'input>>,
}

antlr_rust::tid! { impl<'input,Input> TidAble<'input> for EOPlusLexer<'input,Input> where Input:CharStream<From<'input> > }

impl<'input, Input: CharStream<From<'input>>> Deref for EOPlusLexer<'input, Input> {
    type Target = BaseLexer<'input, EOPlusLexerActions, Input, LocalTokenFactory<'input>>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, Input: CharStream<From<'input>>> DerefMut for EOPlusLexer<'input, Input> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<'input, Input: CharStream<From<'input>>> EOPlusLexer<'input, Input> {
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "EOPlusLexer.g4"
    }

    pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
        antlr_rust::recognizer::check_version("0", "3");
        Self {
            base: BaseLexer::new_base_lexer(
                input,
                LexerATNSimulator::new_lexer_atnsimulator(
                    _ATN.clone(),
                    _decision_to_DFA.clone(),
                    _shared_context_cache.clone(),
                ),
                EOPlusLexerActions {},
                tf,
            ),
        }
    }
}

impl<'input, Input: CharStream<From<'input>>> EOPlusLexer<'input, Input>
where
    &'input LocalTokenFactory<'input>: Default,
{
    pub fn new(input: Input) -> Self {
        EOPlusLexer::new_with_token_factory(
            input,
            <&LocalTokenFactory<'input> as Default>::default(),
        )
    }
}

pub struct EOPlusLexerActions {}

impl EOPlusLexerActions {}

impl<'input, Input: CharStream<From<'input>>>
    Actions<'input, BaseLexer<'input, EOPlusLexerActions, Input, LocalTokenFactory<'input>>>
    for EOPlusLexerActions
{
}

impl<'input, Input: CharStream<From<'input>>> EOPlusLexer<'input, Input> {}

impl<'input, Input: CharStream<From<'input>>>
    LexerRecog<'input, BaseLexer<'input, EOPlusLexerActions, Input, LocalTokenFactory<'input>>>
    for EOPlusLexerActions
{
}
impl<'input> TokenAware<'input> for EOPlusLexerActions {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, Input: CharStream<From<'input>>> TokenSource<'input> for EOPlusLexer<'input, Input> {
    type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        self.base.get_input_stream()
    }

    fn get_source_name(&self) -> String {
        self.base.get_source_name()
    }

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
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
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x16\u{106}\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\
		\x05\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\
		\x09\x04\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\
		\x0e\x09\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\
		\x12\x04\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\
		\x17\x09\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\
		\x1b\x04\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\
		\x20\x09\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\
		\x24\x04\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\
		\x29\x09\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\
		\x2d\x04\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x03\
		\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\x03\x03\
		\x03\x03\x03\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\
		\x04\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\
		\x05\x03\x05\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x07\x03\x07\x03\
		\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x08\x03\x08\x03\x08\x03\x08\x03\
		\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0b\x03\
		\x0b\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\
		\x10\x03\x10\x03\x11\x03\x11\x03\x12\x03\x12\x03\x13\x03\x13\x03\x14\x03\
		\x14\x03\x15\x03\x15\x03\x16\x03\x16\x03\x17\x03\x17\x03\x18\x03\x18\x03\
		\x19\x03\x19\x03\x1a\x03\x1a\x03\x1b\x03\x1b\x03\x1c\x03\x1c\x03\x1d\x03\
		\x1d\x03\x1e\x03\x1e\x03\x1f\x03\x1f\x03\x20\x03\x20\x03\x21\x03\x21\x03\
		\x22\x03\x22\x03\x23\x03\x23\x03\x24\x03\x24\x03\x25\x03\x25\x03\x26\x03\
		\x26\x03\x27\x03\x27\x03\x28\x03\x28\x03\x29\x03\x29\x03\x2a\x03\x2a\x03\
		\x2b\x06\x2b\u{da}\x0a\x2b\x0d\x2b\x0e\x2b\u{db}\x03\x2c\x03\x2c\x07\x2c\
		\u{e0}\x0a\x2c\x0c\x2c\x0e\x2c\u{e3}\x0b\x2c\x03\x2c\x03\x2c\x03\x2d\x03\
		\x2d\x05\x2d\u{e9}\x0a\x2d\x03\x2e\x03\x2e\x03\x2e\x03\x2f\x03\x2f\x07\
		\x2f\u{f0}\x0a\x2f\x0c\x2f\x0e\x2f\u{f3}\x0b\x2f\x03\x30\x06\x30\u{f6}\
		\x0a\x30\x0d\x30\x0e\x30\u{f7}\x03\x30\x03\x30\x03\x31\x03\x31\x03\x31\
		\x03\x31\x07\x31\u{100}\x0a\x31\x0c\x31\x0e\x31\u{103}\x0b\x31\x03\x31\
		\x03\x31\x02\x02\x32\x03\x03\x05\x04\x07\x05\x09\x06\x0b\x07\x0d\x08\x0f\
		\x09\x11\x0a\x13\x02\x15\x02\x17\x02\x19\x02\x1b\x02\x1d\x02\x1f\x02\x21\
		\x02\x23\x02\x25\x02\x27\x02\x29\x02\x2b\x02\x2d\x02\x2f\x02\x31\x02\x33\
		\x02\x35\x02\x37\x02\x39\x02\x3b\x02\x3d\x02\x3f\x02\x41\x02\x43\x02\x45\
		\x02\x47\x0b\x49\x0c\x4b\x0d\x4d\x0e\x4f\x0f\x51\x10\x53\x11\x55\x12\x57\
		\x13\x59\x02\x5b\x02\x5d\x14\x5f\x15\x61\x16\x03\x02\x22\x04\x02\x43\x43\
		\x63\x63\x04\x02\x44\x44\x64\x64\x04\x02\x45\x45\x65\x65\x04\x02\x46\x46\
		\x66\x66\x04\x02\x47\x47\x67\x67\x04\x02\x48\x48\x68\x68\x04\x02\x49\x49\
		\x69\x69\x04\x02\x4a\x4a\x6a\x6a\x04\x02\x4b\x4b\x6b\x6b\x04\x02\x4c\x4c\
		\x6c\x6c\x04\x02\x4d\x4d\x6d\x6d\x04\x02\x4e\x4e\x6e\x6e\x04\x02\x4f\x4f\
		\x6f\x6f\x04\x02\x50\x50\x70\x70\x04\x02\x51\x51\x71\x71\x04\x02\x52\x52\
		\x72\x72\x04\x02\x53\x53\x73\x73\x04\x02\x54\x54\x74\x74\x04\x02\x55\x55\
		\x75\x75\x04\x02\x56\x56\x76\x76\x04\x02\x57\x57\x77\x77\x04\x02\x58\x58\
		\x78\x78\x04\x02\x59\x59\x79\x79\x04\x02\x5a\x5a\x7a\x7a\x04\x02\x5b\x5b\
		\x7b\x7b\x04\x02\x5c\x5c\x7c\x7c\x03\x02\x32\x3b\x06\x02\x0c\x0c\x0f\x0f\
		\x24\x24\x5e\x5e\x04\x02\x0c\x0c\x0f\x0f\x06\x02\x26\x26\x43\x5c\x61\x61\
		\x63\x7c\x06\x02\x32\x3b\x43\x5c\x61\x61\x63\x7c\x05\x02\x0b\x0c\x0e\x0f\
		\x22\x22\x02\u{ef}\x02\x03\x03\x02\x02\x02\x02\x05\x03\x02\x02\x02\x02\
		\x07\x03\x02\x02\x02\x02\x09\x03\x02\x02\x02\x02\x0b\x03\x02\x02\x02\x02\
		\x0d\x03\x02\x02\x02\x02\x0f\x03\x02\x02\x02\x02\x11\x03\x02\x02\x02\x02\
		\x47\x03\x02\x02\x02\x02\x49\x03\x02\x02\x02\x02\x4b\x03\x02\x02\x02\x02\
		\x4d\x03\x02\x02\x02\x02\x4f\x03\x02\x02\x02\x02\x51\x03\x02\x02\x02\x02\
		\x53\x03\x02\x02\x02\x02\x55\x03\x02\x02\x02\x02\x57\x03\x02\x02\x02\x02\
		\x5d\x03\x02\x02\x02\x02\x5f\x03\x02\x02\x02\x02\x61\x03\x02\x02\x02\x03\
		\x63\x03\x02\x02\x02\x05\x68\x03\x02\x02\x02\x07\x6e\x03\x02\x02\x02\x09\
		\x78\x03\x02\x02\x02\x0b\u{80}\x03\x02\x02\x02\x0d\u{85}\x03\x02\x02\x02\
		\x0f\u{8c}\x03\x02\x02\x02\x11\u{91}\x03\x02\x02\x02\x13\u{96}\x03\x02\
		\x02\x02\x15\u{98}\x03\x02\x02\x02\x17\u{9a}\x03\x02\x02\x02\x19\u{9c}\
		\x03\x02\x02\x02\x1b\u{9e}\x03\x02\x02\x02\x1d\u{a0}\x03\x02\x02\x02\x1f\
		\u{a2}\x03\x02\x02\x02\x21\u{a4}\x03\x02\x02\x02\x23\u{a6}\x03\x02\x02\
		\x02\x25\u{a8}\x03\x02\x02\x02\x27\u{aa}\x03\x02\x02\x02\x29\u{ac}\x03\
		\x02\x02\x02\x2b\u{ae}\x03\x02\x02\x02\x2d\u{b0}\x03\x02\x02\x02\x2f\u{b2}\
		\x03\x02\x02\x02\x31\u{b4}\x03\x02\x02\x02\x33\u{b6}\x03\x02\x02\x02\x35\
		\u{b8}\x03\x02\x02\x02\x37\u{ba}\x03\x02\x02\x02\x39\u{bc}\x03\x02\x02\
		\x02\x3b\u{be}\x03\x02\x02\x02\x3d\u{c0}\x03\x02\x02\x02\x3f\u{c2}\x03\
		\x02\x02\x02\x41\u{c4}\x03\x02\x02\x02\x43\u{c6}\x03\x02\x02\x02\x45\u{c8}\
		\x03\x02\x02\x02\x47\u{ca}\x03\x02\x02\x02\x49\u{cc}\x03\x02\x02\x02\x4b\
		\u{ce}\x03\x02\x02\x02\x4d\u{d0}\x03\x02\x02\x02\x4f\u{d2}\x03\x02\x02\
		\x02\x51\u{d4}\x03\x02\x02\x02\x53\u{d6}\x03\x02\x02\x02\x55\u{d9}\x03\
		\x02\x02\x02\x57\u{dd}\x03\x02\x02\x02\x59\u{e8}\x03\x02\x02\x02\x5b\u{ea}\
		\x03\x02\x02\x02\x5d\u{ed}\x03\x02\x02\x02\x5f\u{f5}\x03\x02\x02\x02\x61\
		\u{fb}\x03\x02\x02\x02\x63\x64\x05\x2b\x16\x02\x64\x65\x05\x13\x0a\x02\
		\x65\x66\x05\x23\x12\x02\x66\x67\x05\x2d\x17\x02\x67\x04\x03\x02\x02\x02\
		\x68\x69\x05\x37\x1c\x02\x69\x6a\x05\x39\x1d\x02\x6a\x6b\x05\x13\x0a\x02\
		\x6b\x6c\x05\x39\x1d\x02\x6c\x6d\x05\x1b\x0e\x02\x6d\x06\x03\x02\x02\x02\
		\x6e\x6f\x05\x33\x1a\x02\x6f\x70\x05\x3b\x1e\x02\x70\x71\x05\x1b\x0e\x02\
		\x71\x72\x05\x37\x1c\x02\x72\x73\x05\x39\x1d\x02\x73\x74\x05\x2d\x17\x02\
		\x74\x75\x05\x13\x0a\x02\x75\x76\x05\x2b\x16\x02\x76\x77\x05\x1b\x0e\x02\
		\x77\x08\x03\x02\x02\x02\x78\x79\x05\x3d\x1f\x02\x79\x7a\x05\x1b\x0e\x02\
		\x7a\x7b\x05\x35\x1b\x02\x7b\x7c\x05\x37\x1c\x02\x7c\x7d\x05\x23\x12\x02\
		\x7d\x7e\x05\x2f\x18\x02\x7e\x7f\x05\x2d\x17\x02\x7f\x0a\x03\x02\x02\x02\
		\u{80}\u{81}\x05\x19\x0d\x02\u{81}\u{82}\x05\x1b\x0e\x02\u{82}\u{83}\x05\
		\x37\x1c\x02\u{83}\u{84}\x05\x17\x0c\x02\u{84}\x0c\x03\x02\x02\x02\u{85}\
		\u{86}\x05\x13\x0a\x02\u{86}\u{87}\x05\x17\x0c\x02\u{87}\u{88}\x05\x39\
		\x1d\x02\u{88}\u{89}\x05\x23\x12\x02\u{89}\u{8a}\x05\x2f\x18\x02\u{8a}\
		\u{8b}\x05\x2d\x17\x02\u{8b}\x0e\x03\x02\x02\x02\u{8c}\u{8d}\x05\x35\x1b\
		\x02\u{8d}\u{8e}\x05\x3b\x1e\x02\u{8e}\u{8f}\x05\x29\x15\x02\u{8f}\u{90}\
		\x05\x1b\x0e\x02\u{90}\x10\x03\x02\x02\x02\u{91}\u{92}\x05\x1f\x10\x02\
		\u{92}\u{93}\x05\x2f\x18\x02\u{93}\u{94}\x05\x39\x1d\x02\u{94}\u{95}\x05\
		\x2f\x18\x02\u{95}\x12\x03\x02\x02\x02\u{96}\u{97}\x09\x02\x02\x02\u{97}\
		\x14\x03\x02\x02\x02\u{98}\u{99}\x09\x03\x02\x02\u{99}\x16\x03\x02\x02\
		\x02\u{9a}\u{9b}\x09\x04\x02\x02\u{9b}\x18\x03\x02\x02\x02\u{9c}\u{9d}\
		\x09\x05\x02\x02\u{9d}\x1a\x03\x02\x02\x02\u{9e}\u{9f}\x09\x06\x02\x02\
		\u{9f}\x1c\x03\x02\x02\x02\u{a0}\u{a1}\x09\x07\x02\x02\u{a1}\x1e\x03\x02\
		\x02\x02\u{a2}\u{a3}\x09\x08\x02\x02\u{a3}\x20\x03\x02\x02\x02\u{a4}\u{a5}\
		\x09\x09\x02\x02\u{a5}\x22\x03\x02\x02\x02\u{a6}\u{a7}\x09\x0a\x02\x02\
		\u{a7}\x24\x03\x02\x02\x02\u{a8}\u{a9}\x09\x0b\x02\x02\u{a9}\x26\x03\x02\
		\x02\x02\u{aa}\u{ab}\x09\x0c\x02\x02\u{ab}\x28\x03\x02\x02\x02\u{ac}\u{ad}\
		\x09\x0d\x02\x02\u{ad}\x2a\x03\x02\x02\x02\u{ae}\u{af}\x09\x0e\x02\x02\
		\u{af}\x2c\x03\x02\x02\x02\u{b0}\u{b1}\x09\x0f\x02\x02\u{b1}\x2e\x03\x02\
		\x02\x02\u{b2}\u{b3}\x09\x10\x02\x02\u{b3}\x30\x03\x02\x02\x02\u{b4}\u{b5}\
		\x09\x11\x02\x02\u{b5}\x32\x03\x02\x02\x02\u{b6}\u{b7}\x09\x12\x02\x02\
		\u{b7}\x34\x03\x02\x02\x02\u{b8}\u{b9}\x09\x13\x02\x02\u{b9}\x36\x03\x02\
		\x02\x02\u{ba}\u{bb}\x09\x14\x02\x02\u{bb}\x38\x03\x02\x02\x02\u{bc}\u{bd}\
		\x09\x15\x02\x02\u{bd}\x3a\x03\x02\x02\x02\u{be}\u{bf}\x09\x16\x02\x02\
		\u{bf}\x3c\x03\x02\x02\x02\u{c0}\u{c1}\x09\x17\x02\x02\u{c1}\x3e\x03\x02\
		\x02\x02\u{c2}\u{c3}\x09\x18\x02\x02\u{c3}\x40\x03\x02\x02\x02\u{c4}\u{c5}\
		\x09\x19\x02\x02\u{c5}\x42\x03\x02\x02\x02\u{c6}\u{c7}\x09\x1a\x02\x02\
		\u{c7}\x44\x03\x02\x02\x02\u{c8}\u{c9}\x09\x1b\x02\x02\u{c9}\x46\x03\x02\
		\x02\x02\u{ca}\u{cb}\x07\x2a\x02\x02\u{cb}\x48\x03\x02\x02\x02\u{cc}\u{cd}\
		\x07\x2b\x02\x02\u{cd}\x4a\x03\x02\x02\x02\u{ce}\u{cf}\x07\x7d\x02\x02\
		\u{cf}\x4c\x03\x02\x02\x02\u{d0}\u{d1}\x07\x7f\x02\x02\u{d1}\x4e\x03\x02\
		\x02\x02\u{d2}\u{d3}\x07\x3d\x02\x02\u{d3}\x50\x03\x02\x02\x02\u{d4}\u{d5}\
		\x07\x2e\x02\x02\u{d5}\x52\x03\x02\x02\x02\u{d6}\u{d7}\x07\x30\x02\x02\
		\u{d7}\x54\x03\x02\x02\x02\u{d8}\u{da}\x09\x1c\x02\x02\u{d9}\u{d8}\x03\
		\x02\x02\x02\u{da}\u{db}\x03\x02\x02\x02\u{db}\u{d9}\x03\x02\x02\x02\u{db}\
		\u{dc}\x03\x02\x02\x02\u{dc}\x56\x03\x02\x02\x02\u{dd}\u{e1}\x07\x24\x02\
		\x02\u{de}\u{e0}\x05\x59\x2d\x02\u{df}\u{de}\x03\x02\x02\x02\u{e0}\u{e3}\
		\x03\x02\x02\x02\u{e1}\u{df}\x03\x02\x02\x02\u{e1}\u{e2}\x03\x02\x02\x02\
		\u{e2}\u{e4}\x03\x02\x02\x02\u{e3}\u{e1}\x03\x02\x02\x02\u{e4}\u{e5}\x07\
		\x24\x02\x02\u{e5}\x58\x03\x02\x02\x02\u{e6}\u{e9}\x0a\x1d\x02\x02\u{e7}\
		\u{e9}\x05\x5b\x2e\x02\u{e8}\u{e6}\x03\x02\x02\x02\u{e8}\u{e7}\x03\x02\
		\x02\x02\u{e9}\x5a\x03\x02\x02\x02\u{ea}\u{eb}\x07\x5e\x02\x02\u{eb}\u{ec}\
		\x0a\x1e\x02\x02\u{ec}\x5c\x03\x02\x02\x02\u{ed}\u{f1}\x09\x1f\x02\x02\
		\u{ee}\u{f0}\x09\x20\x02\x02\u{ef}\u{ee}\x03\x02\x02\x02\u{f0}\u{f3}\x03\
		\x02\x02\x02\u{f1}\u{ef}\x03\x02\x02\x02\u{f1}\u{f2}\x03\x02\x02\x02\u{f2}\
		\x5e\x03\x02\x02\x02\u{f3}\u{f1}\x03\x02\x02\x02\u{f4}\u{f6}\x09\x21\x02\
		\x02\u{f5}\u{f4}\x03\x02\x02\x02\u{f6}\u{f7}\x03\x02\x02\x02\u{f7}\u{f5}\
		\x03\x02\x02\x02\u{f7}\u{f8}\x03\x02\x02\x02\u{f8}\u{f9}\x03\x02\x02\x02\
		\u{f9}\u{fa}\x08\x30\x02\x02\u{fa}\x60\x03\x02\x02\x02\u{fb}\u{fc}\x07\
		\x31\x02\x02\u{fc}\u{fd}\x07\x31\x02\x02\u{fd}\u{101}\x03\x02\x02\x02\u{fe}\
		\u{100}\x0a\x1e\x02\x02\u{ff}\u{fe}\x03\x02\x02\x02\u{100}\u{103}\x03\x02\
		\x02\x02\u{101}\u{ff}\x03\x02\x02\x02\u{101}\u{102}\x03\x02\x02\x02\u{102}\
		\u{104}\x03\x02\x02\x02\u{103}\u{101}\x03\x02\x02\x02\u{104}\u{105}\x08\
		\x31\x02\x02\u{105}\x62\x03\x02\x02\x02\x09\x02\u{db}\u{e1}\u{e8}\u{f1}\
		\u{f7}\u{101}\x03\x02\x03\x02";
