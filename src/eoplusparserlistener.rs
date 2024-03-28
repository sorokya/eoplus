#![allow(nonstandard_style)]
// Generated from eoplus-antlr4/EOPlusParser.g4 by ANTLR 4.8
use super::eoplusparser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait EOPlusParserListener<'input>: ParseTreeListener<'input, EOPlusParserContextType> {
    /**
     * Enter a parse tree produced by {@link EOPlusParser#file}.
     * @param ctx the parse tree
     */
    fn enter_file(&mut self, _ctx: &FileContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#file}.
     * @param ctx the parse tree
     */
    fn exit_file(&mut self, _ctx: &FileContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link EOPlusParser#block}.
     * @param ctx the parse tree
     */
    fn enter_block(&mut self, _ctx: &BlockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#block}.
     * @param ctx the parse tree
     */
    fn exit_block(&mut self, _ctx: &BlockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link EOPlusParser#mainBlock}.
     * @param ctx the parse tree
     */
    fn enter_mainBlock(&mut self, _ctx: &MainBlockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#mainBlock}.
     * @param ctx the parse tree
     */
    fn exit_mainBlock(&mut self, _ctx: &MainBlockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link EOPlusParser#mainAttribute}.
     * @param ctx the parse tree
     */
    fn enter_mainAttribute(&mut self, _ctx: &MainAttributeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#mainAttribute}.
     * @param ctx the parse tree
     */
    fn exit_mainAttribute(&mut self, _ctx: &MainAttributeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link EOPlusParser#nameAttribute}.
     * @param ctx the parse tree
     */
    fn enter_nameAttribute(&mut self, _ctx: &NameAttributeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#nameAttribute}.
     * @param ctx the parse tree
     */
    fn exit_nameAttribute(&mut self, _ctx: &NameAttributeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link EOPlusParser#versionAttribute}.
     * @param ctx the parse tree
     */
    fn enter_versionAttribute(&mut self, _ctx: &VersionAttributeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#versionAttribute}.
     * @param ctx the parse tree
     */
    fn exit_versionAttribute(&mut self, _ctx: &VersionAttributeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link EOPlusParser#version}.
     * @param ctx the parse tree
     */
    fn enter_version(&mut self, _ctx: &VersionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#version}.
     * @param ctx the parse tree
     */
    fn exit_version(&mut self, _ctx: &VersionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link EOPlusParser#stateBlock}.
     * @param ctx the parse tree
     */
    fn enter_stateBlock(&mut self, _ctx: &StateBlockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#stateBlock}.
     * @param ctx the parse tree
     */
    fn exit_stateBlock(&mut self, _ctx: &StateBlockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link EOPlusParser#statement}.
     * @param ctx the parse tree
     */
    fn enter_statement(&mut self, _ctx: &StatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#statement}.
     * @param ctx the parse tree
     */
    fn exit_statement(&mut self, _ctx: &StatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link EOPlusParser#desc}.
     * @param ctx the parse tree
     */
    fn enter_desc(&mut self, _ctx: &DescContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#desc}.
     * @param ctx the parse tree
     */
    fn exit_desc(&mut self, _ctx: &DescContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link EOPlusParser#action}.
     * @param ctx the parse tree
     */
    fn enter_action(&mut self, _ctx: &ActionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#action}.
     * @param ctx the parse tree
     */
    fn exit_action(&mut self, _ctx: &ActionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link EOPlusParser#eorule}.
     * @param ctx the parse tree
     */
    fn enter_eorule(&mut self, _ctx: &EoruleContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#eorule}.
     * @param ctx the parse tree
     */
    fn exit_eorule(&mut self, _ctx: &EoruleContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link EOPlusParser#argumentList}.
     * @param ctx the parse tree
     */
    fn enter_argumentList(&mut self, _ctx: &ArgumentListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#argumentList}.
     * @param ctx the parse tree
     */
    fn exit_argumentList(&mut self, _ctx: &ArgumentListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link EOPlusParser#arguments}.
     * @param ctx the parse tree
     */
    fn enter_arguments(&mut self, _ctx: &ArgumentsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#arguments}.
     * @param ctx the parse tree
     */
    fn exit_arguments(&mut self, _ctx: &ArgumentsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link EOPlusParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link EOPlusParser#literal}.
     * @param ctx the parse tree
     */
    fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link EOPlusParser#literal}.
     * @param ctx the parse tree
     */
    fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) {}
}

antlr_rust::coerce_from! { 'input : EOPlusParserListener<'input> }
