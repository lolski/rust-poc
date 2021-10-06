// Generated from /home/vmax/.cache/bazel/_bazel_vmax/78a5ef47aa0f794ae8d175ad83930027/sandbox/linux-sandbox/19/execroot/__main__/bazel-out/k8-fastbuild/bin/TypeQLRust.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::typeqlrustlistener::*;
use super::typeqlrustvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const T__7:isize=8; 
		pub const T__8:isize=9; 
		pub const MATCH:isize=10; 
		pub const GET:isize=11; 
		pub const DEFINE:isize=12; 
		pub const UNDEFINE:isize=13; 
		pub const INSERT:isize=14; 
		pub const DELETE:isize=15; 
		pub const COMPUTE:isize=16; 
		pub const THING:isize=17; 
		pub const ENTITY:isize=18; 
		pub const ATTRIBUTE:isize=19; 
		pub const RELATION:isize=20; 
		pub const ROLE:isize=21; 
		pub const RULE:isize=22; 
		pub const OFFSET:isize=23; 
		pub const LIMIT:isize=24; 
		pub const SORT:isize=25; 
		pub const ORDER_:isize=26; 
		pub const ASC:isize=27; 
		pub const DESC:isize=28; 
		pub const TYPE:isize=29; 
		pub const ABSTRACT:isize=30; 
		pub const SUB_:isize=31; 
		pub const SUB:isize=32; 
		pub const SUBX:isize=33; 
		pub const OWNS:isize=34; 
		pub const IS_KEY:isize=35; 
		pub const REGEX:isize=36; 
		pub const AS:isize=37; 
		pub const PLAYS:isize=38; 
		pub const RELATES:isize=39; 
		pub const WHEN:isize=40; 
		pub const THEN:isize=41; 
		pub const IID:isize=42; 
		pub const ISA_:isize=43; 
		pub const ISA:isize=44; 
		pub const ISAX:isize=45; 
		pub const HAS:isize=46; 
		pub const VALUE:isize=47; 
		pub const IS:isize=48; 
		pub const OR:isize=49; 
		pub const NOT:isize=50; 
		pub const EQ:isize=51; 
		pub const NEQ:isize=52; 
		pub const GT:isize=53; 
		pub const GTE:isize=54; 
		pub const LT:isize=55; 
		pub const LTE:isize=56; 
		pub const LIKE:isize=57; 
		pub const CONTAINS:isize=58; 
		pub const GROUP:isize=59; 
		pub const COUNT:isize=60; 
		pub const MAX:isize=61; 
		pub const MIN:isize=62; 
		pub const MEAN:isize=63; 
		pub const MEDIAN:isize=64; 
		pub const STD:isize=65; 
		pub const SUM:isize=66; 
		pub const CLUSTER:isize=67; 
		pub const CENTRALITY:isize=68; 
		pub const PATH:isize=69; 
		pub const DEGREE:isize=70; 
		pub const K_CORE:isize=71; 
		pub const CONNECTED_COMPONENT:isize=72; 
		pub const FROM:isize=73; 
		pub const TO:isize=74; 
		pub const OF:isize=75; 
		pub const IN:isize=76; 
		pub const USING:isize=77; 
		pub const WHERE:isize=78; 
		pub const MIN_K:isize=79; 
		pub const K:isize=80; 
		pub const SIZE:isize=81; 
		pub const LONG:isize=82; 
		pub const DOUBLE:isize=83; 
		pub const STRING:isize=84; 
		pub const BOOLEAN:isize=85; 
		pub const DATETIME:isize=86; 
		pub const BOOLEAN_:isize=87; 
		pub const TRUE:isize=88; 
		pub const FALSE:isize=89; 
		pub const STRING_:isize=90; 
		pub const LONG_:isize=91; 
		pub const DOUBLE_:isize=92; 
		pub const DATE_:isize=93; 
		pub const DATETIME_:isize=94; 
		pub const VAR_:isize=95; 
		pub const VAR_ANONYMOUS_:isize=96; 
		pub const VAR_NAMED_:isize=97; 
		pub const IID_:isize=98; 
		pub const LABEL_:isize=99; 
		pub const LABEL_SCOPED_:isize=100; 
		pub const COMMENT:isize=101; 
		pub const WS:isize=102; 
		pub const UNRECOGNISED:isize=103;
	pub const RULE_eof_query:usize = 0; 
	pub const RULE_eof_queries:usize = 1; 
	pub const RULE_eof_pattern:usize = 2; 
	pub const RULE_eof_patterns:usize = 3; 
	pub const RULE_eof_definables:usize = 4; 
	pub const RULE_eof_variable:usize = 5; 
	pub const RULE_eof_label:usize = 6; 
	pub const RULE_eof_schema_rule:usize = 7; 
	pub const RULE_query:usize = 8; 
	pub const RULE_query_define:usize = 9; 
	pub const RULE_query_undefine:usize = 10; 
	pub const RULE_query_insert:usize = 11; 
	pub const RULE_query_delete_or_update:usize = 12; 
	pub const RULE_query_match:usize = 13; 
	pub const RULE_query_compute:usize = 14; 
	pub const RULE_query_match_aggregate:usize = 15; 
	pub const RULE_query_match_group:usize = 16; 
	pub const RULE_query_match_group_agg:usize = 17; 
	pub const RULE_modifiers:usize = 18; 
	pub const RULE_filter_:usize = 19; 
	pub const RULE_sort:usize = 20; 
	pub const RULE_offset:usize = 21; 
	pub const RULE_limit:usize = 22; 
	pub const RULE_match_aggregate:usize = 23; 
	pub const RULE_aggregate_method:usize = 24; 
	pub const RULE_match_group:usize = 25; 
	pub const RULE_definables:usize = 26; 
	pub const RULE_definable:usize = 27; 
	pub const RULE_patterns:usize = 28; 
	pub const RULE_pattern:usize = 29; 
	pub const RULE_pattern_conjunction:usize = 30; 
	pub const RULE_pattern_disjunction:usize = 31; 
	pub const RULE_pattern_negation:usize = 32; 
	pub const RULE_pattern_variable:usize = 33; 
	pub const RULE_variable_concept:usize = 34; 
	pub const RULE_variable_type:usize = 35; 
	pub const RULE_type_constraint:usize = 36; 
	pub const RULE_variable_things:usize = 37; 
	pub const RULE_variable_thing_any:usize = 38; 
	pub const RULE_variable_thing:usize = 39; 
	pub const RULE_variable_relation:usize = 40; 
	pub const RULE_variable_attribute:usize = 41; 
	pub const RULE_relation:usize = 42; 
	pub const RULE_role_player:usize = 43; 
	pub const RULE_player:usize = 44; 
	pub const RULE_attributes:usize = 45; 
	pub const RULE_attribute:usize = 46; 
	pub const RULE_predicate:usize = 47; 
	pub const RULE_predicate_equality:usize = 48; 
	pub const RULE_predicate_substring:usize = 49; 
	pub const RULE_predicate_value:usize = 50; 
	pub const RULE_schema_rule:usize = 51; 
	pub const RULE_compute_conditions:usize = 52; 
	pub const RULE_compute_method:usize = 53; 
	pub const RULE_conditions_count:usize = 54; 
	pub const RULE_conditions_value:usize = 55; 
	pub const RULE_conditions_central:usize = 56; 
	pub const RULE_conditions_cluster:usize = 57; 
	pub const RULE_conditions_path:usize = 58; 
	pub const RULE_input_count:usize = 59; 
	pub const RULE_input_value:usize = 60; 
	pub const RULE_input_central:usize = 61; 
	pub const RULE_input_cluster:usize = 62; 
	pub const RULE_input_path:usize = 63; 
	pub const RULE_compute_direction:usize = 64; 
	pub const RULE_compute_target:usize = 65; 
	pub const RULE_compute_scope:usize = 66; 
	pub const RULE_compute_config:usize = 67; 
	pub const RULE_compute_algorithm:usize = 68; 
	pub const RULE_compute_args:usize = 69; 
	pub const RULE_compute_args_array:usize = 70; 
	pub const RULE_compute_arg:usize = 71; 
	pub const RULE_type_any:usize = 72; 
	pub const RULE_type_scoped:usize = 73; 
	pub const RULE_type_:usize = 74; 
	pub const RULE_label_any:usize = 75; 
	pub const RULE_label_scoped:usize = 76; 
	pub const RULE_label:usize = 77; 
	pub const RULE_labels:usize = 78; 
	pub const RULE_label_array:usize = 79; 
	pub const RULE_schema_native:usize = 80; 
	pub const RULE_type_native:usize = 81; 
	pub const RULE_value_type:usize = 82; 
	pub const RULE_value:usize = 83; 
	pub const RULE_regex:usize = 84; 
	pub const RULE_unreserved:usize = 85;
	pub const ruleNames: [&'static str; 86] =  [
		"eof_query", "eof_queries", "eof_pattern", "eof_patterns", "eof_definables", 
		"eof_variable", "eof_label", "eof_schema_rule", "query", "query_define", 
		"query_undefine", "query_insert", "query_delete_or_update", "query_match", 
		"query_compute", "query_match_aggregate", "query_match_group", "query_match_group_agg", 
		"modifiers", "filter_", "sort", "offset", "limit", "match_aggregate", 
		"aggregate_method", "match_group", "definables", "definable", "patterns", 
		"pattern", "pattern_conjunction", "pattern_disjunction", "pattern_negation", 
		"pattern_variable", "variable_concept", "variable_type", "type_constraint", 
		"variable_things", "variable_thing_any", "variable_thing", "variable_relation", 
		"variable_attribute", "relation", "role_player", "player", "attributes", 
		"attribute", "predicate", "predicate_equality", "predicate_substring", 
		"predicate_value", "schema_rule", "compute_conditions", "compute_method", 
		"conditions_count", "conditions_value", "conditions_central", "conditions_cluster", 
		"conditions_path", "input_count", "input_value", "input_central", "input_cluster", 
		"input_path", "compute_direction", "compute_target", "compute_scope", 
		"compute_config", "compute_algorithm", "compute_args", "compute_args_array", 
		"compute_arg", "type_any", "type_scoped", "type_", "label_any", "label_scoped", 
		"label", "labels", "label_array", "schema_native", "type_native", "value_type", 
		"value", "regex", "unreserved"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;97] = [
		None, Some("';'"), Some("','"), Some("'{'"), Some("'}'"), Some("'('"), 
		Some("')'"), Some("':'"), Some("'['"), Some("']'"), Some("'match'"), Some("'get'"), 
		Some("'define'"), Some("'undefine'"), Some("'insert'"), Some("'delete'"), 
		Some("'compute'"), Some("'thing'"), Some("'entity'"), Some("'attribute'"), 
		Some("'relation'"), Some("'role'"), Some("'rule'"), Some("'offset'"), 
		Some("'limit'"), Some("'sort'"), None, Some("'asc'"), Some("'desc'"), 
		Some("'type'"), Some("'abstract'"), None, Some("'sub'"), Some("'sub!'"), 
		Some("'owns'"), Some("'@key'"), Some("'regex'"), Some("'as'"), Some("'plays'"), 
		Some("'relates'"), Some("'when'"), Some("'then'"), Some("'iid'"), None, 
		Some("'isa'"), Some("'isa!'"), Some("'has'"), Some("'value'"), Some("'is'"), 
		Some("'or'"), Some("'not'"), Some("'='"), Some("'!='"), Some("'>'"), Some("'>='"), 
		Some("'<'"), Some("'<='"), Some("'like'"), Some("'contains'"), Some("'group'"), 
		Some("'count'"), Some("'max'"), Some("'min'"), Some("'mean'"), Some("'median'"), 
		Some("'std'"), Some("'sum'"), Some("'cluster'"), Some("'centrality'"), 
		Some("'path'"), Some("'degree'"), Some("'k-core'"), Some("'connected-component'"), 
		Some("'from'"), Some("'to'"), Some("'of'"), Some("'in'"), Some("'using'"), 
		Some("'where'"), Some("'min-k'"), Some("'k'"), Some("'size'"), Some("'long'"), 
		Some("'double'"), Some("'string'"), Some("'boolean'"), Some("'datetime'"), 
		None, Some("'true'"), Some("'false'"), None, None, None, None, None, None, 
		Some("'$_'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;104]  = [
		None, None, None, None, None, None, None, None, None, None, Some("MATCH"), 
		Some("GET"), Some("DEFINE"), Some("UNDEFINE"), Some("INSERT"), Some("DELETE"), 
		Some("COMPUTE"), Some("THING"), Some("ENTITY"), Some("ATTRIBUTE"), Some("RELATION"), 
		Some("ROLE"), Some("RULE"), Some("OFFSET"), Some("LIMIT"), Some("SORT"), 
		Some("ORDER_"), Some("ASC"), Some("DESC"), Some("TYPE"), Some("ABSTRACT"), 
		Some("SUB_"), Some("SUB"), Some("SUBX"), Some("OWNS"), Some("IS_KEY"), 
		Some("REGEX"), Some("AS"), Some("PLAYS"), Some("RELATES"), Some("WHEN"), 
		Some("THEN"), Some("IID"), Some("ISA_"), Some("ISA"), Some("ISAX"), Some("HAS"), 
		Some("VALUE"), Some("IS"), Some("OR"), Some("NOT"), Some("EQ"), Some("NEQ"), 
		Some("GT"), Some("GTE"), Some("LT"), Some("LTE"), Some("LIKE"), Some("CONTAINS"), 
		Some("GROUP"), Some("COUNT"), Some("MAX"), Some("MIN"), Some("MEAN"), 
		Some("MEDIAN"), Some("STD"), Some("SUM"), Some("CLUSTER"), Some("CENTRALITY"), 
		Some("PATH"), Some("DEGREE"), Some("K_CORE"), Some("CONNECTED_COMPONENT"), 
		Some("FROM"), Some("TO"), Some("OF"), Some("IN"), Some("USING"), Some("WHERE"), 
		Some("MIN_K"), Some("K"), Some("SIZE"), Some("LONG"), Some("DOUBLE"), 
		Some("STRING"), Some("BOOLEAN"), Some("DATETIME"), Some("BOOLEAN_"), Some("TRUE"), 
		Some("FALSE"), Some("STRING_"), Some("LONG_"), Some("DOUBLE_"), Some("DATE_"), 
		Some("DATETIME_"), Some("VAR_"), Some("VAR_ANONYMOUS_"), Some("VAR_NAMED_"), 
		Some("IID_"), Some("LABEL_"), Some("LABEL_SCOPED_"), Some("COMMENT"), 
		Some("WS"), Some("UNRECOGNISED")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,TypeQLRustParserExt, I, TypeQLRustParserContextType , dyn TypeQLRustListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type TypeQLRustTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, TypeQLRustParserContextType , dyn TypeQLRustListener<'input> + 'a>;

/// Parser for TypeQLRust grammar
pub struct TypeQLRustParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","2");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				TypeQLRustParserExt{
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> TypeQLRustParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> TypeQLRustParser<'input, I, DefaultErrorStrategy<'input,TypeQLRustParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for TypeQLRustParser
pub trait TypeQLRustParserContext<'input>:
	for<'x> Listenable<dyn TypeQLRustListener<'input> + 'x > + 
	for<'x> Visitable<dyn TypeQLRustVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=TypeQLRustParserContextType>
{}

impl<'input, 'x, T> VisitableDyn<T> for dyn TypeQLRustParserContext<'input> + 'input
where
    T: TypeQLRustVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn TypeQLRustVisitor<'input> + 'x))
    }
}

impl<'input> TypeQLRustParserContext<'input> for TerminalNode<'input,TypeQLRustParserContextType> {}
impl<'input> TypeQLRustParserContext<'input> for ErrorNode<'input,TypeQLRustParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn TypeQLRustParserContext<'input> + 'input{}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn TypeQLRustListener<'input> + 'input{}

pub struct TypeQLRustParserContextType;
antlr_rust::type_id!{TypeQLRustParserContextType}

impl<'input> ParserNodeType<'input> for TypeQLRustParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn TypeQLRustParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct TypeQLRustParserExt{
}

impl TypeQLRustParserExt{
}


impl<'input> TokenAware<'input> for TypeQLRustParserExt{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for TypeQLRustParserExt{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for TypeQLRustParserExt{
	fn get_grammar_file_name(&self) -> & str{ "TypeQLRust.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- eof_query ----------------
pub type Eof_queryContextAll<'input> = Eof_queryContext<'input>;


pub type Eof_queryContext<'input> = BaseParserRuleContext<'input,Eof_queryContextExt<'input>>;

#[derive(Clone)]
pub struct Eof_queryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Eof_queryContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Eof_queryContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_eof_query(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_eof_query(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Eof_queryContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_eof_query(self);
	}
}

impl<'input> CustomRuleContext<'input> for Eof_queryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eof_query }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eof_query }
}
antlr_rust::type_id!{Eof_queryContextExt<'a>}

impl<'input> Eof_queryContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Eof_queryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Eof_queryContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Eof_queryContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Eof_queryContextExt<'input>>{

fn query(&self) -> Option<Rc<QueryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}

}

impl<'input> Eof_queryContextAttrs<'input> for Eof_queryContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eof_query(&mut self,)
	-> Result<Rc<Eof_queryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Eof_queryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_eof_query);
        let mut _localctx: Rc<Eof_queryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule query*/
			recog.base.set_state(172);
			recog.query()?;

			recog.base.set_state(173);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- eof_queries ----------------
pub type Eof_queriesContextAll<'input> = Eof_queriesContext<'input>;


pub type Eof_queriesContext<'input> = BaseParserRuleContext<'input,Eof_queriesContextExt<'input>>;

#[derive(Clone)]
pub struct Eof_queriesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Eof_queriesContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Eof_queriesContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_eof_queries(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_eof_queries(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Eof_queriesContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_eof_queries(self);
	}
}

impl<'input> CustomRuleContext<'input> for Eof_queriesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eof_queries }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eof_queries }
}
antlr_rust::type_id!{Eof_queriesContextExt<'a>}

impl<'input> Eof_queriesContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Eof_queriesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Eof_queriesContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Eof_queriesContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Eof_queriesContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn query_all(&self) ->  Vec<Rc<QueryContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn query(&self, i: usize) -> Option<Rc<QueryContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Eof_queriesContextAttrs<'input> for Eof_queriesContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eof_queries(&mut self,)
	-> Result<Rc<Eof_queriesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Eof_queriesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_eof_queries);
        let mut _localctx: Rc<Eof_queriesContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(176); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule query*/
				recog.base.set_state(175);
				recog.query()?;

				}
				}
				recog.base.set_state(178); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << MATCH) | (1usize << DEFINE) | (1usize << UNDEFINE) | (1usize << INSERT) | (1usize << COMPUTE))) != 0)) {break}
			}
			recog.base.set_state(180);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- eof_pattern ----------------
pub type Eof_patternContextAll<'input> = Eof_patternContext<'input>;


pub type Eof_patternContext<'input> = BaseParserRuleContext<'input,Eof_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Eof_patternContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Eof_patternContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Eof_patternContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_eof_pattern(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_eof_pattern(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Eof_patternContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_eof_pattern(self);
	}
}

impl<'input> CustomRuleContext<'input> for Eof_patternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eof_pattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eof_pattern }
}
antlr_rust::type_id!{Eof_patternContextExt<'a>}

impl<'input> Eof_patternContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Eof_patternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Eof_patternContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Eof_patternContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Eof_patternContextExt<'input>>{

fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}

}

impl<'input> Eof_patternContextAttrs<'input> for Eof_patternContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eof_pattern(&mut self,)
	-> Result<Rc<Eof_patternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Eof_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_eof_pattern);
        let mut _localctx: Rc<Eof_patternContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule pattern*/
			recog.base.set_state(182);
			recog.pattern()?;

			recog.base.set_state(183);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- eof_patterns ----------------
pub type Eof_patternsContextAll<'input> = Eof_patternsContext<'input>;


pub type Eof_patternsContext<'input> = BaseParserRuleContext<'input,Eof_patternsContextExt<'input>>;

#[derive(Clone)]
pub struct Eof_patternsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Eof_patternsContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Eof_patternsContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_eof_patterns(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_eof_patterns(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Eof_patternsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_eof_patterns(self);
	}
}

impl<'input> CustomRuleContext<'input> for Eof_patternsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eof_patterns }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eof_patterns }
}
antlr_rust::type_id!{Eof_patternsContextExt<'a>}

impl<'input> Eof_patternsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Eof_patternsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Eof_patternsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Eof_patternsContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Eof_patternsContextExt<'input>>{

fn patterns(&self) -> Option<Rc<PatternsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}

}

impl<'input> Eof_patternsContextAttrs<'input> for Eof_patternsContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eof_patterns(&mut self,)
	-> Result<Rc<Eof_patternsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Eof_patternsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_eof_patterns);
        let mut _localctx: Rc<Eof_patternsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule patterns*/
			recog.base.set_state(185);
			recog.patterns()?;

			recog.base.set_state(186);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- eof_definables ----------------
pub type Eof_definablesContextAll<'input> = Eof_definablesContext<'input>;


pub type Eof_definablesContext<'input> = BaseParserRuleContext<'input,Eof_definablesContextExt<'input>>;

#[derive(Clone)]
pub struct Eof_definablesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Eof_definablesContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Eof_definablesContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_eof_definables(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_eof_definables(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Eof_definablesContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_eof_definables(self);
	}
}

impl<'input> CustomRuleContext<'input> for Eof_definablesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eof_definables }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eof_definables }
}
antlr_rust::type_id!{Eof_definablesContextExt<'a>}

impl<'input> Eof_definablesContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Eof_definablesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Eof_definablesContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Eof_definablesContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Eof_definablesContextExt<'input>>{

fn definables(&self) -> Option<Rc<DefinablesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}

}

impl<'input> Eof_definablesContextAttrs<'input> for Eof_definablesContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eof_definables(&mut self,)
	-> Result<Rc<Eof_definablesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Eof_definablesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_eof_definables);
        let mut _localctx: Rc<Eof_definablesContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule definables*/
			recog.base.set_state(188);
			recog.definables()?;

			recog.base.set_state(189);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- eof_variable ----------------
pub type Eof_variableContextAll<'input> = Eof_variableContext<'input>;


pub type Eof_variableContext<'input> = BaseParserRuleContext<'input,Eof_variableContextExt<'input>>;

#[derive(Clone)]
pub struct Eof_variableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Eof_variableContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Eof_variableContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_eof_variable(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_eof_variable(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Eof_variableContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_eof_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for Eof_variableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eof_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eof_variable }
}
antlr_rust::type_id!{Eof_variableContextExt<'a>}

impl<'input> Eof_variableContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Eof_variableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Eof_variableContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Eof_variableContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Eof_variableContextExt<'input>>{

fn pattern_variable(&self) -> Option<Rc<Pattern_variableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}

}

impl<'input> Eof_variableContextAttrs<'input> for Eof_variableContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eof_variable(&mut self,)
	-> Result<Rc<Eof_variableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Eof_variableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_eof_variable);
        let mut _localctx: Rc<Eof_variableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule pattern_variable*/
			recog.base.set_state(191);
			recog.pattern_variable()?;

			recog.base.set_state(192);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- eof_label ----------------
pub type Eof_labelContextAll<'input> = Eof_labelContext<'input>;


pub type Eof_labelContext<'input> = BaseParserRuleContext<'input,Eof_labelContextExt<'input>>;

#[derive(Clone)]
pub struct Eof_labelContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Eof_labelContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Eof_labelContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_eof_label(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_eof_label(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Eof_labelContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_eof_label(self);
	}
}

impl<'input> CustomRuleContext<'input> for Eof_labelContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eof_label }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eof_label }
}
antlr_rust::type_id!{Eof_labelContextExt<'a>}

impl<'input> Eof_labelContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Eof_labelContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Eof_labelContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Eof_labelContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Eof_labelContextExt<'input>>{

fn label(&self) -> Option<Rc<LabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}

}

impl<'input> Eof_labelContextAttrs<'input> for Eof_labelContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eof_label(&mut self,)
	-> Result<Rc<Eof_labelContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Eof_labelContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_eof_label);
        let mut _localctx: Rc<Eof_labelContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule label*/
			recog.base.set_state(194);
			recog.label()?;

			recog.base.set_state(195);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- eof_schema_rule ----------------
pub type Eof_schema_ruleContextAll<'input> = Eof_schema_ruleContext<'input>;


pub type Eof_schema_ruleContext<'input> = BaseParserRuleContext<'input,Eof_schema_ruleContextExt<'input>>;

#[derive(Clone)]
pub struct Eof_schema_ruleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Eof_schema_ruleContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Eof_schema_ruleContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_eof_schema_rule(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_eof_schema_rule(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Eof_schema_ruleContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_eof_schema_rule(self);
	}
}

impl<'input> CustomRuleContext<'input> for Eof_schema_ruleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eof_schema_rule }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eof_schema_rule }
}
antlr_rust::type_id!{Eof_schema_ruleContextExt<'a>}

impl<'input> Eof_schema_ruleContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Eof_schema_ruleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Eof_schema_ruleContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Eof_schema_ruleContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Eof_schema_ruleContextExt<'input>>{

fn schema_rule(&self) -> Option<Rc<Schema_ruleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}

}

impl<'input> Eof_schema_ruleContextAttrs<'input> for Eof_schema_ruleContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eof_schema_rule(&mut self,)
	-> Result<Rc<Eof_schema_ruleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Eof_schema_ruleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_eof_schema_rule);
        let mut _localctx: Rc<Eof_schema_ruleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule schema_rule*/
			recog.base.set_state(197);
			recog.schema_rule()?;

			recog.base.set_state(198);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- query ----------------
pub type QueryContextAll<'input> = QueryContext<'input>;


pub type QueryContext<'input> = BaseParserRuleContext<'input,QueryContextExt<'input>>;

#[derive(Clone)]
pub struct QueryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for QueryContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for QueryContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_query(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_query(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for QueryContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_query(self);
	}
}

impl<'input> CustomRuleContext<'input> for QueryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_query }
	//fn type_rule_index() -> usize where Self: Sized { RULE_query }
}
antlr_rust::type_id!{QueryContextExt<'a>}

impl<'input> QueryContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QueryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QueryContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait QueryContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<QueryContextExt<'input>>{

fn query_define(&self) -> Option<Rc<Query_defineContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn query_undefine(&self) -> Option<Rc<Query_undefineContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn query_insert(&self) -> Option<Rc<Query_insertContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn query_delete_or_update(&self) -> Option<Rc<Query_delete_or_updateContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn query_match(&self) -> Option<Rc<Query_matchContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn query_match_aggregate(&self) -> Option<Rc<Query_match_aggregateContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn query_match_group(&self) -> Option<Rc<Query_match_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn query_match_group_agg(&self) -> Option<Rc<Query_match_group_aggContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn query_compute(&self) -> Option<Rc<Query_computeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> QueryContextAttrs<'input> for QueryContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn query(&mut self,)
	-> Result<Rc<QueryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QueryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_query);
        let mut _localctx: Rc<QueryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(209);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule query_define*/
					recog.base.set_state(200);
					recog.query_define()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule query_undefine*/
					recog.base.set_state(201);
					recog.query_undefine()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule query_insert*/
					recog.base.set_state(202);
					recog.query_insert()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule query_delete_or_update*/
					recog.base.set_state(203);
					recog.query_delete_or_update()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule query_match*/
					recog.base.set_state(204);
					recog.query_match()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule query_match_aggregate*/
					recog.base.set_state(205);
					recog.query_match_aggregate()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule query_match_group*/
					recog.base.set_state(206);
					recog.query_match_group()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule query_match_group_agg*/
					recog.base.set_state(207);
					recog.query_match_group_agg()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule query_compute*/
					recog.base.set_state(208);
					recog.query_compute()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- query_define ----------------
pub type Query_defineContextAll<'input> = Query_defineContext<'input>;


pub type Query_defineContext<'input> = BaseParserRuleContext<'input,Query_defineContextExt<'input>>;

#[derive(Clone)]
pub struct Query_defineContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Query_defineContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Query_defineContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_query_define(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_query_define(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Query_defineContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_query_define(self);
	}
}

impl<'input> CustomRuleContext<'input> for Query_defineContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_query_define }
	//fn type_rule_index() -> usize where Self: Sized { RULE_query_define }
}
antlr_rust::type_id!{Query_defineContextExt<'a>}

impl<'input> Query_defineContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Query_defineContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Query_defineContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Query_defineContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Query_defineContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DEFINE
/// Returns `None` if there is no child corresponding to token DEFINE
fn DEFINE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(DEFINE, 0)
}
fn definables(&self) -> Option<Rc<DefinablesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Query_defineContextAttrs<'input> for Query_defineContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn query_define(&mut self,)
	-> Result<Rc<Query_defineContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Query_defineContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_query_define);
        let mut _localctx: Rc<Query_defineContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(211);
			recog.base.match_token(DEFINE,&mut recog.err_handler)?;

			/*InvokeRule definables*/
			recog.base.set_state(212);
			recog.definables()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- query_undefine ----------------
pub type Query_undefineContextAll<'input> = Query_undefineContext<'input>;


pub type Query_undefineContext<'input> = BaseParserRuleContext<'input,Query_undefineContextExt<'input>>;

#[derive(Clone)]
pub struct Query_undefineContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Query_undefineContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Query_undefineContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_query_undefine(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_query_undefine(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Query_undefineContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_query_undefine(self);
	}
}

impl<'input> CustomRuleContext<'input> for Query_undefineContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_query_undefine }
	//fn type_rule_index() -> usize where Self: Sized { RULE_query_undefine }
}
antlr_rust::type_id!{Query_undefineContextExt<'a>}

impl<'input> Query_undefineContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Query_undefineContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Query_undefineContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Query_undefineContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Query_undefineContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UNDEFINE
/// Returns `None` if there is no child corresponding to token UNDEFINE
fn UNDEFINE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(UNDEFINE, 0)
}
fn definables(&self) -> Option<Rc<DefinablesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Query_undefineContextAttrs<'input> for Query_undefineContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn query_undefine(&mut self,)
	-> Result<Rc<Query_undefineContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Query_undefineContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_query_undefine);
        let mut _localctx: Rc<Query_undefineContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(214);
			recog.base.match_token(UNDEFINE,&mut recog.err_handler)?;

			/*InvokeRule definables*/
			recog.base.set_state(215);
			recog.definables()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- query_insert ----------------
pub type Query_insertContextAll<'input> = Query_insertContext<'input>;


pub type Query_insertContext<'input> = BaseParserRuleContext<'input,Query_insertContextExt<'input>>;

#[derive(Clone)]
pub struct Query_insertContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Query_insertContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Query_insertContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_query_insert(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_query_insert(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Query_insertContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_query_insert(self);
	}
}

impl<'input> CustomRuleContext<'input> for Query_insertContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_query_insert }
	//fn type_rule_index() -> usize where Self: Sized { RULE_query_insert }
}
antlr_rust::type_id!{Query_insertContextExt<'a>}

impl<'input> Query_insertContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Query_insertContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Query_insertContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Query_insertContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Query_insertContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MATCH
/// Returns `None` if there is no child corresponding to token MATCH
fn MATCH(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MATCH, 0)
}
fn patterns(&self) -> Option<Rc<PatternsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token INSERT
/// Returns `None` if there is no child corresponding to token INSERT
fn INSERT(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(INSERT, 0)
}
fn variable_things(&self) -> Option<Rc<Variable_thingsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Query_insertContextAttrs<'input> for Query_insertContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn query_insert(&mut self,)
	-> Result<Rc<Query_insertContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Query_insertContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_query_insert);
        let mut _localctx: Rc<Query_insertContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(224);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 MATCH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(217);
					recog.base.match_token(MATCH,&mut recog.err_handler)?;

					/*InvokeRule patterns*/
					recog.base.set_state(218);
					recog.patterns()?;

					recog.base.set_state(219);
					recog.base.match_token(INSERT,&mut recog.err_handler)?;

					/*InvokeRule variable_things*/
					recog.base.set_state(220);
					recog.variable_things()?;

					}
				}

			 INSERT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(222);
					recog.base.match_token(INSERT,&mut recog.err_handler)?;

					/*InvokeRule variable_things*/
					recog.base.set_state(223);
					recog.variable_things()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- query_delete_or_update ----------------
pub type Query_delete_or_updateContextAll<'input> = Query_delete_or_updateContext<'input>;


pub type Query_delete_or_updateContext<'input> = BaseParserRuleContext<'input,Query_delete_or_updateContextExt<'input>>;

#[derive(Clone)]
pub struct Query_delete_or_updateContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Query_delete_or_updateContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Query_delete_or_updateContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_query_delete_or_update(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_query_delete_or_update(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Query_delete_or_updateContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_query_delete_or_update(self);
	}
}

impl<'input> CustomRuleContext<'input> for Query_delete_or_updateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_query_delete_or_update }
	//fn type_rule_index() -> usize where Self: Sized { RULE_query_delete_or_update }
}
antlr_rust::type_id!{Query_delete_or_updateContextExt<'a>}

impl<'input> Query_delete_or_updateContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Query_delete_or_updateContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Query_delete_or_updateContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Query_delete_or_updateContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Query_delete_or_updateContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MATCH
/// Returns `None` if there is no child corresponding to token MATCH
fn MATCH(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MATCH, 0)
}
fn patterns(&self) -> Option<Rc<PatternsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DELETE
/// Returns `None` if there is no child corresponding to token DELETE
fn DELETE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(DELETE, 0)
}
fn variable_things_all(&self) ->  Vec<Rc<Variable_thingsContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variable_things(&self, i: usize) -> Option<Rc<Variable_thingsContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token INSERT
/// Returns `None` if there is no child corresponding to token INSERT
fn INSERT(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(INSERT, 0)
}

}

impl<'input> Query_delete_or_updateContextAttrs<'input> for Query_delete_or_updateContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn query_delete_or_update(&mut self,)
	-> Result<Rc<Query_delete_or_updateContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Query_delete_or_updateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_query_delete_or_update);
        let mut _localctx: Rc<Query_delete_or_updateContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(226);
			recog.base.match_token(MATCH,&mut recog.err_handler)?;

			/*InvokeRule patterns*/
			recog.base.set_state(227);
			recog.patterns()?;

			recog.base.set_state(228);
			recog.base.match_token(DELETE,&mut recog.err_handler)?;

			/*InvokeRule variable_things*/
			recog.base.set_state(229);
			recog.variable_things()?;

			recog.base.set_state(232);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(3,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(230);
					recog.base.match_token(INSERT,&mut recog.err_handler)?;

					/*InvokeRule variable_things*/
					recog.base.set_state(231);
					recog.variable_things()?;

					}
				}

				_ => {}
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- query_match ----------------
pub type Query_matchContextAll<'input> = Query_matchContext<'input>;


pub type Query_matchContext<'input> = BaseParserRuleContext<'input,Query_matchContextExt<'input>>;

#[derive(Clone)]
pub struct Query_matchContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Query_matchContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Query_matchContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_query_match(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_query_match(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Query_matchContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_query_match(self);
	}
}

impl<'input> CustomRuleContext<'input> for Query_matchContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_query_match }
	//fn type_rule_index() -> usize where Self: Sized { RULE_query_match }
}
antlr_rust::type_id!{Query_matchContextExt<'a>}

impl<'input> Query_matchContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Query_matchContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Query_matchContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Query_matchContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Query_matchContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MATCH
/// Returns `None` if there is no child corresponding to token MATCH
fn MATCH(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MATCH, 0)
}
fn patterns(&self) -> Option<Rc<PatternsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn modifiers(&self) -> Option<Rc<ModifiersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Query_matchContextAttrs<'input> for Query_matchContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn query_match(&mut self,)
	-> Result<Rc<Query_matchContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Query_matchContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_query_match);
        let mut _localctx: Rc<Query_matchContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(234);
			recog.base.match_token(MATCH,&mut recog.err_handler)?;

			/*InvokeRule patterns*/
			recog.base.set_state(235);
			recog.patterns()?;

			{
			/*InvokeRule modifiers*/
			recog.base.set_state(236);
			recog.modifiers()?;

			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- query_compute ----------------
pub type Query_computeContextAll<'input> = Query_computeContext<'input>;


pub type Query_computeContext<'input> = BaseParserRuleContext<'input,Query_computeContextExt<'input>>;

#[derive(Clone)]
pub struct Query_computeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Query_computeContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Query_computeContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_query_compute(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_query_compute(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Query_computeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_query_compute(self);
	}
}

impl<'input> CustomRuleContext<'input> for Query_computeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_query_compute }
	//fn type_rule_index() -> usize where Self: Sized { RULE_query_compute }
}
antlr_rust::type_id!{Query_computeContextExt<'a>}

impl<'input> Query_computeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Query_computeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Query_computeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Query_computeContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Query_computeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COMPUTE
/// Returns `None` if there is no child corresponding to token COMPUTE
fn COMPUTE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(COMPUTE, 0)
}
fn compute_conditions(&self) -> Option<Rc<Compute_conditionsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Query_computeContextAttrs<'input> for Query_computeContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn query_compute(&mut self,)
	-> Result<Rc<Query_computeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Query_computeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_query_compute);
        let mut _localctx: Rc<Query_computeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(238);
			recog.base.match_token(COMPUTE,&mut recog.err_handler)?;

			/*InvokeRule compute_conditions*/
			recog.base.set_state(239);
			recog.compute_conditions()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- query_match_aggregate ----------------
pub type Query_match_aggregateContextAll<'input> = Query_match_aggregateContext<'input>;


pub type Query_match_aggregateContext<'input> = BaseParserRuleContext<'input,Query_match_aggregateContextExt<'input>>;

#[derive(Clone)]
pub struct Query_match_aggregateContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Query_match_aggregateContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Query_match_aggregateContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_query_match_aggregate(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_query_match_aggregate(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Query_match_aggregateContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_query_match_aggregate(self);
	}
}

impl<'input> CustomRuleContext<'input> for Query_match_aggregateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_query_match_aggregate }
	//fn type_rule_index() -> usize where Self: Sized { RULE_query_match_aggregate }
}
antlr_rust::type_id!{Query_match_aggregateContextExt<'a>}

impl<'input> Query_match_aggregateContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Query_match_aggregateContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Query_match_aggregateContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Query_match_aggregateContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Query_match_aggregateContextExt<'input>>{

fn query_match(&self) -> Option<Rc<Query_matchContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn match_aggregate(&self) -> Option<Rc<Match_aggregateContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Query_match_aggregateContextAttrs<'input> for Query_match_aggregateContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn query_match_aggregate(&mut self,)
	-> Result<Rc<Query_match_aggregateContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Query_match_aggregateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_query_match_aggregate);
        let mut _localctx: Rc<Query_match_aggregateContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule query_match*/
			recog.base.set_state(241);
			recog.query_match()?;

			/*InvokeRule match_aggregate*/
			recog.base.set_state(242);
			recog.match_aggregate()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- query_match_group ----------------
pub type Query_match_groupContextAll<'input> = Query_match_groupContext<'input>;


pub type Query_match_groupContext<'input> = BaseParserRuleContext<'input,Query_match_groupContextExt<'input>>;

#[derive(Clone)]
pub struct Query_match_groupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Query_match_groupContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Query_match_groupContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_query_match_group(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_query_match_group(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Query_match_groupContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_query_match_group(self);
	}
}

impl<'input> CustomRuleContext<'input> for Query_match_groupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_query_match_group }
	//fn type_rule_index() -> usize where Self: Sized { RULE_query_match_group }
}
antlr_rust::type_id!{Query_match_groupContextExt<'a>}

impl<'input> Query_match_groupContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Query_match_groupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Query_match_groupContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Query_match_groupContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Query_match_groupContextExt<'input>>{

fn query_match(&self) -> Option<Rc<Query_matchContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn match_group(&self) -> Option<Rc<Match_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Query_match_groupContextAttrs<'input> for Query_match_groupContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn query_match_group(&mut self,)
	-> Result<Rc<Query_match_groupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Query_match_groupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_query_match_group);
        let mut _localctx: Rc<Query_match_groupContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule query_match*/
			recog.base.set_state(244);
			recog.query_match()?;

			/*InvokeRule match_group*/
			recog.base.set_state(245);
			recog.match_group()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- query_match_group_agg ----------------
pub type Query_match_group_aggContextAll<'input> = Query_match_group_aggContext<'input>;


pub type Query_match_group_aggContext<'input> = BaseParserRuleContext<'input,Query_match_group_aggContextExt<'input>>;

#[derive(Clone)]
pub struct Query_match_group_aggContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Query_match_group_aggContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Query_match_group_aggContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_query_match_group_agg(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_query_match_group_agg(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Query_match_group_aggContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_query_match_group_agg(self);
	}
}

impl<'input> CustomRuleContext<'input> for Query_match_group_aggContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_query_match_group_agg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_query_match_group_agg }
}
antlr_rust::type_id!{Query_match_group_aggContextExt<'a>}

impl<'input> Query_match_group_aggContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Query_match_group_aggContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Query_match_group_aggContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Query_match_group_aggContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Query_match_group_aggContextExt<'input>>{

fn query_match(&self) -> Option<Rc<Query_matchContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn match_group(&self) -> Option<Rc<Match_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn match_aggregate(&self) -> Option<Rc<Match_aggregateContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Query_match_group_aggContextAttrs<'input> for Query_match_group_aggContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn query_match_group_agg(&mut self,)
	-> Result<Rc<Query_match_group_aggContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Query_match_group_aggContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_query_match_group_agg);
        let mut _localctx: Rc<Query_match_group_aggContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule query_match*/
			recog.base.set_state(247);
			recog.query_match()?;

			/*InvokeRule match_group*/
			recog.base.set_state(248);
			recog.match_group()?;

			/*InvokeRule match_aggregate*/
			recog.base.set_state(249);
			recog.match_aggregate()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- modifiers ----------------
pub type ModifiersContextAll<'input> = ModifiersContext<'input>;


pub type ModifiersContext<'input> = BaseParserRuleContext<'input,ModifiersContextExt<'input>>;

#[derive(Clone)]
pub struct ModifiersContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for ModifiersContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for ModifiersContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_modifiers(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_modifiers(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for ModifiersContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_modifiers(self);
	}
}

impl<'input> CustomRuleContext<'input> for ModifiersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_modifiers }
	//fn type_rule_index() -> usize where Self: Sized { RULE_modifiers }
}
antlr_rust::type_id!{ModifiersContextExt<'a>}

impl<'input> ModifiersContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModifiersContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModifiersContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModifiersContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<ModifiersContextExt<'input>>{

fn filter_(&self) -> Option<Rc<Filter_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn sort(&self) -> Option<Rc<SortContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn offset(&self) -> Option<Rc<OffsetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn limit(&self) -> Option<Rc<LimitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ModifiersContextAttrs<'input> for ModifiersContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn modifiers(&mut self,)
	-> Result<Rc<ModifiersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModifiersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_modifiers);
        let mut _localctx: Rc<ModifiersContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(254);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==GET {
				{
				/*InvokeRule filter_*/
				recog.base.set_state(251);
				recog.filter_()?;

				recog.base.set_state(252);
				recog.base.match_token(T__0,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(259);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==SORT {
				{
				/*InvokeRule sort*/
				recog.base.set_state(256);
				recog.sort()?;

				recog.base.set_state(257);
				recog.base.match_token(T__0,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(264);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==OFFSET {
				{
				/*InvokeRule offset*/
				recog.base.set_state(261);
				recog.offset()?;

				recog.base.set_state(262);
				recog.base.match_token(T__0,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(269);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LIMIT {
				{
				/*InvokeRule limit*/
				recog.base.set_state(266);
				recog.limit()?;

				recog.base.set_state(267);
				recog.base.match_token(T__0,&mut recog.err_handler)?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- filter_ ----------------
pub type Filter_ContextAll<'input> = Filter_Context<'input>;


pub type Filter_Context<'input> = BaseParserRuleContext<'input,Filter_ContextExt<'input>>;

#[derive(Clone)]
pub struct Filter_ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Filter_Context<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Filter_Context<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_filter_(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_filter_(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Filter_Context<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_filter_(self);
	}
}

impl<'input> CustomRuleContext<'input> for Filter_ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_filter_ }
	//fn type_rule_index() -> usize where Self: Sized { RULE_filter_ }
}
antlr_rust::type_id!{Filter_ContextExt<'a>}

impl<'input> Filter_ContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Filter_ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Filter_ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Filter_ContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Filter_ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token GET
/// Returns `None` if there is no child corresponding to token GET
fn GET(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(GET, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token VAR_ in current rule
fn VAR__all(&self) -> Vec<Rc<TerminalNode<'input,TypeQLRustParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token VAR_, starting from 0.
/// Returns `None` if number of children corresponding to token VAR_ is less or equal than `i`.
fn VAR_(&self, i: usize) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VAR_, i)
}

}

impl<'input> Filter_ContextAttrs<'input> for Filter_Context<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn filter_(&mut self,)
	-> Result<Rc<Filter_ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Filter_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_filter_);
        let mut _localctx: Rc<Filter_ContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(271);
			recog.base.match_token(GET,&mut recog.err_handler)?;

			recog.base.set_state(272);
			recog.base.match_token(VAR_,&mut recog.err_handler)?;

			recog.base.set_state(277);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__1 {
				{
				{
				recog.base.set_state(273);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				recog.base.set_state(274);
				recog.base.match_token(VAR_,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(279);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- sort ----------------
pub type SortContextAll<'input> = SortContext<'input>;


pub type SortContext<'input> = BaseParserRuleContext<'input,SortContextExt<'input>>;

#[derive(Clone)]
pub struct SortContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for SortContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for SortContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_sort(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_sort(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for SortContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_sort(self);
	}
}

impl<'input> CustomRuleContext<'input> for SortContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sort }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sort }
}
antlr_rust::type_id!{SortContextExt<'a>}

impl<'input> SortContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SortContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SortContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SortContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<SortContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SORT
/// Returns `None` if there is no child corresponding to token SORT
fn SORT(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(SORT, 0)
}
/// Retrieves first TerminalNode corresponding to token VAR_
/// Returns `None` if there is no child corresponding to token VAR_
fn VAR_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VAR_, 0)
}
/// Retrieves first TerminalNode corresponding to token ORDER_
/// Returns `None` if there is no child corresponding to token ORDER_
fn ORDER_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(ORDER_, 0)
}

}

impl<'input> SortContextAttrs<'input> for SortContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sort(&mut self,)
	-> Result<Rc<SortContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SortContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_sort);
        let mut _localctx: Rc<SortContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(280);
			recog.base.match_token(SORT,&mut recog.err_handler)?;

			recog.base.set_state(281);
			recog.base.match_token(VAR_,&mut recog.err_handler)?;

			recog.base.set_state(283);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ORDER_ {
				{
				recog.base.set_state(282);
				recog.base.match_token(ORDER_,&mut recog.err_handler)?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- offset ----------------
pub type OffsetContextAll<'input> = OffsetContext<'input>;


pub type OffsetContext<'input> = BaseParserRuleContext<'input,OffsetContextExt<'input>>;

#[derive(Clone)]
pub struct OffsetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for OffsetContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for OffsetContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_offset(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_offset(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for OffsetContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_offset(self);
	}
}

impl<'input> CustomRuleContext<'input> for OffsetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_offset }
	//fn type_rule_index() -> usize where Self: Sized { RULE_offset }
}
antlr_rust::type_id!{OffsetContextExt<'a>}

impl<'input> OffsetContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OffsetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OffsetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OffsetContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<OffsetContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OFFSET
/// Returns `None` if there is no child corresponding to token OFFSET
fn OFFSET(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(OFFSET, 0)
}
/// Retrieves first TerminalNode corresponding to token LONG_
/// Returns `None` if there is no child corresponding to token LONG_
fn LONG_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(LONG_, 0)
}

}

impl<'input> OffsetContextAttrs<'input> for OffsetContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn offset(&mut self,)
	-> Result<Rc<OffsetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OffsetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_offset);
        let mut _localctx: Rc<OffsetContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(285);
			recog.base.match_token(OFFSET,&mut recog.err_handler)?;

			recog.base.set_state(286);
			recog.base.match_token(LONG_,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- limit ----------------
pub type LimitContextAll<'input> = LimitContext<'input>;


pub type LimitContext<'input> = BaseParserRuleContext<'input,LimitContextExt<'input>>;

#[derive(Clone)]
pub struct LimitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for LimitContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for LimitContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_limit(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_limit(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for LimitContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_limit(self);
	}
}

impl<'input> CustomRuleContext<'input> for LimitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_limit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_limit }
}
antlr_rust::type_id!{LimitContextExt<'a>}

impl<'input> LimitContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LimitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LimitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LimitContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<LimitContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LIMIT
/// Returns `None` if there is no child corresponding to token LIMIT
fn LIMIT(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(LIMIT, 0)
}
/// Retrieves first TerminalNode corresponding to token LONG_
/// Returns `None` if there is no child corresponding to token LONG_
fn LONG_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(LONG_, 0)
}

}

impl<'input> LimitContextAttrs<'input> for LimitContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn limit(&mut self,)
	-> Result<Rc<LimitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LimitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_limit);
        let mut _localctx: Rc<LimitContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(288);
			recog.base.match_token(LIMIT,&mut recog.err_handler)?;

			recog.base.set_state(289);
			recog.base.match_token(LONG_,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- match_aggregate ----------------
pub type Match_aggregateContextAll<'input> = Match_aggregateContext<'input>;


pub type Match_aggregateContext<'input> = BaseParserRuleContext<'input,Match_aggregateContextExt<'input>>;

#[derive(Clone)]
pub struct Match_aggregateContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Match_aggregateContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Match_aggregateContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_match_aggregate(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_match_aggregate(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Match_aggregateContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_match_aggregate(self);
	}
}

impl<'input> CustomRuleContext<'input> for Match_aggregateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_match_aggregate }
	//fn type_rule_index() -> usize where Self: Sized { RULE_match_aggregate }
}
antlr_rust::type_id!{Match_aggregateContextExt<'a>}

impl<'input> Match_aggregateContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Match_aggregateContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Match_aggregateContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Match_aggregateContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Match_aggregateContextExt<'input>>{

fn aggregate_method(&self) -> Option<Rc<Aggregate_methodContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token VAR_
/// Returns `None` if there is no child corresponding to token VAR_
fn VAR_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VAR_, 0)
}

}

impl<'input> Match_aggregateContextAttrs<'input> for Match_aggregateContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn match_aggregate(&mut self,)
	-> Result<Rc<Match_aggregateContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Match_aggregateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_match_aggregate);
        let mut _localctx: Rc<Match_aggregateContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule aggregate_method*/
			recog.base.set_state(291);
			recog.aggregate_method()?;

			recog.base.set_state(293);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==VAR_ {
				{
				recog.base.set_state(292);
				recog.base.match_token(VAR_,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(295);
			recog.base.match_token(T__0,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- aggregate_method ----------------
pub type Aggregate_methodContextAll<'input> = Aggregate_methodContext<'input>;


pub type Aggregate_methodContext<'input> = BaseParserRuleContext<'input,Aggregate_methodContextExt<'input>>;

#[derive(Clone)]
pub struct Aggregate_methodContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Aggregate_methodContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Aggregate_methodContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_aggregate_method(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_aggregate_method(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Aggregate_methodContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_aggregate_method(self);
	}
}

impl<'input> CustomRuleContext<'input> for Aggregate_methodContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_aggregate_method }
	//fn type_rule_index() -> usize where Self: Sized { RULE_aggregate_method }
}
antlr_rust::type_id!{Aggregate_methodContextExt<'a>}

impl<'input> Aggregate_methodContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Aggregate_methodContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Aggregate_methodContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Aggregate_methodContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Aggregate_methodContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COUNT
/// Returns `None` if there is no child corresponding to token COUNT
fn COUNT(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(COUNT, 0)
}
/// Retrieves first TerminalNode corresponding to token MAX
/// Returns `None` if there is no child corresponding to token MAX
fn MAX(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MAX, 0)
}
/// Retrieves first TerminalNode corresponding to token MEAN
/// Returns `None` if there is no child corresponding to token MEAN
fn MEAN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MEAN, 0)
}
/// Retrieves first TerminalNode corresponding to token MEDIAN
/// Returns `None` if there is no child corresponding to token MEDIAN
fn MEDIAN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MEDIAN, 0)
}
/// Retrieves first TerminalNode corresponding to token MIN
/// Returns `None` if there is no child corresponding to token MIN
fn MIN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MIN, 0)
}
/// Retrieves first TerminalNode corresponding to token STD
/// Returns `None` if there is no child corresponding to token STD
fn STD(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(STD, 0)
}
/// Retrieves first TerminalNode corresponding to token SUM
/// Returns `None` if there is no child corresponding to token SUM
fn SUM(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(SUM, 0)
}

}

impl<'input> Aggregate_methodContextAttrs<'input> for Aggregate_methodContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn aggregate_method(&mut self,)
	-> Result<Rc<Aggregate_methodContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Aggregate_methodContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_aggregate_method);
        let mut _localctx: Rc<Aggregate_methodContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(297);
			_la = recog.base.input.la(1);
			if { !(((((_la - 60)) & !0x3f) == 0 && ((1usize << (_la - 60)) & ((1usize << (COUNT - 60)) | (1usize << (MAX - 60)) | (1usize << (MIN - 60)) | (1usize << (MEAN - 60)) | (1usize << (MEDIAN - 60)) | (1usize << (STD - 60)) | (1usize << (SUM - 60)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- match_group ----------------
pub type Match_groupContextAll<'input> = Match_groupContext<'input>;


pub type Match_groupContext<'input> = BaseParserRuleContext<'input,Match_groupContextExt<'input>>;

#[derive(Clone)]
pub struct Match_groupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Match_groupContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Match_groupContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_match_group(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_match_group(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Match_groupContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_match_group(self);
	}
}

impl<'input> CustomRuleContext<'input> for Match_groupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_match_group }
	//fn type_rule_index() -> usize where Self: Sized { RULE_match_group }
}
antlr_rust::type_id!{Match_groupContextExt<'a>}

impl<'input> Match_groupContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Match_groupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Match_groupContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Match_groupContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Match_groupContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token GROUP
/// Returns `None` if there is no child corresponding to token GROUP
fn GROUP(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(GROUP, 0)
}
/// Retrieves first TerminalNode corresponding to token VAR_
/// Returns `None` if there is no child corresponding to token VAR_
fn VAR_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VAR_, 0)
}

}

impl<'input> Match_groupContextAttrs<'input> for Match_groupContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn match_group(&mut self,)
	-> Result<Rc<Match_groupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Match_groupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_match_group);
        let mut _localctx: Rc<Match_groupContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(299);
			recog.base.match_token(GROUP,&mut recog.err_handler)?;

			recog.base.set_state(300);
			recog.base.match_token(VAR_,&mut recog.err_handler)?;

			recog.base.set_state(301);
			recog.base.match_token(T__0,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- definables ----------------
pub type DefinablesContextAll<'input> = DefinablesContext<'input>;


pub type DefinablesContext<'input> = BaseParserRuleContext<'input,DefinablesContextExt<'input>>;

#[derive(Clone)]
pub struct DefinablesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for DefinablesContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for DefinablesContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_definables(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_definables(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for DefinablesContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_definables(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefinablesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_definables }
	//fn type_rule_index() -> usize where Self: Sized { RULE_definables }
}
antlr_rust::type_id!{DefinablesContextExt<'a>}

impl<'input> DefinablesContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DefinablesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DefinablesContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DefinablesContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<DefinablesContextExt<'input>>{

fn definable_all(&self) ->  Vec<Rc<DefinableContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn definable(&self, i: usize) -> Option<Rc<DefinableContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DefinablesContextAttrs<'input> for DefinablesContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn definables(&mut self,)
	-> Result<Rc<DefinablesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DefinablesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_definables);
        let mut _localctx: Rc<DefinablesContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(306); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule definable*/
				recog.base.set_state(303);
				recog.definable()?;

				recog.base.set_state(304);
				recog.base.match_token(T__0,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(308); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << GET) | (1usize << THING) | (1usize << ENTITY) | (1usize << ATTRIBUTE) | (1usize << RELATION) | (1usize << ROLE) | (1usize << RULE) | (1usize << OFFSET) | (1usize << LIMIT) | (1usize << SORT) | (1usize << VALUE) | (1usize << CONTAINS) | (1usize << GROUP) | (1usize << COUNT) | (1usize << MAX) | (1usize << MIN) | (1usize << MEAN))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (MEDIAN - 64)) | (1usize << (STD - 64)) | (1usize << (SUM - 64)) | (1usize << (CLUSTER - 64)) | (1usize << (PATH - 64)) | (1usize << (DEGREE - 64)) | (1usize << (K_CORE - 64)) | (1usize << (CONNECTED_COMPONENT - 64)) | (1usize << (FROM - 64)) | (1usize << (TO - 64)) | (1usize << (OF - 64)) | (1usize << (IN - 64)) | (1usize << (WHERE - 64)) | (1usize << (MIN_K - 64)) | (1usize << (K - 64)) | (1usize << (SIZE - 64)) | (1usize << (VAR_ - 64)) | (1usize << (LABEL_ - 64)) | (1usize << (LABEL_SCOPED_ - 64)))) != 0)) {break}
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- definable ----------------
pub type DefinableContextAll<'input> = DefinableContext<'input>;


pub type DefinableContext<'input> = BaseParserRuleContext<'input,DefinableContextExt<'input>>;

#[derive(Clone)]
pub struct DefinableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for DefinableContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for DefinableContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_definable(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_definable(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for DefinableContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_definable(self);
	}
}

impl<'input> CustomRuleContext<'input> for DefinableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_definable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_definable }
}
antlr_rust::type_id!{DefinableContextExt<'a>}

impl<'input> DefinableContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DefinableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DefinableContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DefinableContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<DefinableContextExt<'input>>{

fn variable_type(&self) -> Option<Rc<Variable_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn schema_rule(&self) -> Option<Rc<Schema_ruleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DefinableContextAttrs<'input> for DefinableContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn definable(&mut self,)
	-> Result<Rc<DefinableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DefinableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_definable);
        let mut _localctx: Rc<DefinableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(312);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(12,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule variable_type*/
					recog.base.set_state(310);
					recog.variable_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule schema_rule*/
					recog.base.set_state(311);
					recog.schema_rule()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- patterns ----------------
pub type PatternsContextAll<'input> = PatternsContext<'input>;


pub type PatternsContext<'input> = BaseParserRuleContext<'input,PatternsContextExt<'input>>;

#[derive(Clone)]
pub struct PatternsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for PatternsContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for PatternsContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_patterns(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_patterns(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for PatternsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_patterns(self);
	}
}

impl<'input> CustomRuleContext<'input> for PatternsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patterns }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patterns }
}
antlr_rust::type_id!{PatternsContextExt<'a>}

impl<'input> PatternsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternsContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<PatternsContextExt<'input>>{

fn pattern_all(&self) ->  Vec<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn pattern(&self, i: usize) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PatternsContextAttrs<'input> for PatternsContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn patterns(&mut self,)
	-> Result<Rc<PatternsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_patterns);
        let mut _localctx: Rc<PatternsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(317); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					/*InvokeRule pattern*/
					recog.base.set_state(314);
					recog.pattern()?;

					recog.base.set_state(315);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(319); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(13,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- pattern ----------------
pub type PatternContextAll<'input> = PatternContext<'input>;


pub type PatternContext<'input> = BaseParserRuleContext<'input,PatternContextExt<'input>>;

#[derive(Clone)]
pub struct PatternContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for PatternContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for PatternContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_pattern(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_pattern(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for PatternContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_pattern(self);
	}
}

impl<'input> CustomRuleContext<'input> for PatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pattern }
}
antlr_rust::type_id!{PatternContextExt<'a>}

impl<'input> PatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<PatternContextExt<'input>>{

fn pattern_variable(&self) -> Option<Rc<Pattern_variableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pattern_conjunction(&self) -> Option<Rc<Pattern_conjunctionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pattern_disjunction(&self) -> Option<Rc<Pattern_disjunctionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pattern_negation(&self) -> Option<Rc<Pattern_negationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PatternContextAttrs<'input> for PatternContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pattern(&mut self,)
	-> Result<Rc<PatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_pattern);
        let mut _localctx: Rc<PatternContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(325);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(14,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule pattern_variable*/
					recog.base.set_state(321);
					recog.pattern_variable()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule pattern_conjunction*/
					recog.base.set_state(322);
					recog.pattern_conjunction()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule pattern_disjunction*/
					recog.base.set_state(323);
					recog.pattern_disjunction()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule pattern_negation*/
					recog.base.set_state(324);
					recog.pattern_negation()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- pattern_conjunction ----------------
pub type Pattern_conjunctionContextAll<'input> = Pattern_conjunctionContext<'input>;


pub type Pattern_conjunctionContext<'input> = BaseParserRuleContext<'input,Pattern_conjunctionContextExt<'input>>;

#[derive(Clone)]
pub struct Pattern_conjunctionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Pattern_conjunctionContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Pattern_conjunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_pattern_conjunction(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_pattern_conjunction(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Pattern_conjunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_pattern_conjunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for Pattern_conjunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pattern_conjunction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pattern_conjunction }
}
antlr_rust::type_id!{Pattern_conjunctionContextExt<'a>}

impl<'input> Pattern_conjunctionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Pattern_conjunctionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Pattern_conjunctionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Pattern_conjunctionContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Pattern_conjunctionContextExt<'input>>{

fn patterns(&self) -> Option<Rc<PatternsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Pattern_conjunctionContextAttrs<'input> for Pattern_conjunctionContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pattern_conjunction(&mut self,)
	-> Result<Rc<Pattern_conjunctionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Pattern_conjunctionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_pattern_conjunction);
        let mut _localctx: Rc<Pattern_conjunctionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(327);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			/*InvokeRule patterns*/
			recog.base.set_state(328);
			recog.patterns()?;

			recog.base.set_state(329);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- pattern_disjunction ----------------
pub type Pattern_disjunctionContextAll<'input> = Pattern_disjunctionContext<'input>;


pub type Pattern_disjunctionContext<'input> = BaseParserRuleContext<'input,Pattern_disjunctionContextExt<'input>>;

#[derive(Clone)]
pub struct Pattern_disjunctionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Pattern_disjunctionContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Pattern_disjunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_pattern_disjunction(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_pattern_disjunction(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Pattern_disjunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_pattern_disjunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for Pattern_disjunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pattern_disjunction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pattern_disjunction }
}
antlr_rust::type_id!{Pattern_disjunctionContextExt<'a>}

impl<'input> Pattern_disjunctionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Pattern_disjunctionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Pattern_disjunctionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Pattern_disjunctionContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Pattern_disjunctionContextExt<'input>>{

fn patterns_all(&self) ->  Vec<Rc<PatternsContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn patterns(&self, i: usize) -> Option<Rc<PatternsContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token OR in current rule
fn OR_all(&self) -> Vec<Rc<TerminalNode<'input,TypeQLRustParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token OR, starting from 0.
/// Returns `None` if number of children corresponding to token OR is less or equal than `i`.
fn OR(&self, i: usize) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(OR, i)
}

}

impl<'input> Pattern_disjunctionContextAttrs<'input> for Pattern_disjunctionContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pattern_disjunction(&mut self,)
	-> Result<Rc<Pattern_disjunctionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Pattern_disjunctionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_pattern_disjunction);
        let mut _localctx: Rc<Pattern_disjunctionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(331);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			/*InvokeRule patterns*/
			recog.base.set_state(332);
			recog.patterns()?;

			recog.base.set_state(333);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			recog.base.set_state(339); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				recog.base.set_state(334);
				recog.base.match_token(OR,&mut recog.err_handler)?;

				recog.base.set_state(335);
				recog.base.match_token(T__2,&mut recog.err_handler)?;

				/*InvokeRule patterns*/
				recog.base.set_state(336);
				recog.patterns()?;

				recog.base.set_state(337);
				recog.base.match_token(T__3,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(341); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==OR) {break}
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- pattern_negation ----------------
pub type Pattern_negationContextAll<'input> = Pattern_negationContext<'input>;


pub type Pattern_negationContext<'input> = BaseParserRuleContext<'input,Pattern_negationContextExt<'input>>;

#[derive(Clone)]
pub struct Pattern_negationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Pattern_negationContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Pattern_negationContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_pattern_negation(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_pattern_negation(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Pattern_negationContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_pattern_negation(self);
	}
}

impl<'input> CustomRuleContext<'input> for Pattern_negationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pattern_negation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pattern_negation }
}
antlr_rust::type_id!{Pattern_negationContextExt<'a>}

impl<'input> Pattern_negationContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Pattern_negationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Pattern_negationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Pattern_negationContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Pattern_negationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NOT
/// Returns `None` if there is no child corresponding to token NOT
fn NOT(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(NOT, 0)
}
fn patterns(&self) -> Option<Rc<PatternsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Pattern_negationContextAttrs<'input> for Pattern_negationContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pattern_negation(&mut self,)
	-> Result<Rc<Pattern_negationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Pattern_negationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_pattern_negation);
        let mut _localctx: Rc<Pattern_negationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(343);
			recog.base.match_token(NOT,&mut recog.err_handler)?;

			recog.base.set_state(344);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			/*InvokeRule patterns*/
			recog.base.set_state(345);
			recog.patterns()?;

			recog.base.set_state(346);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- pattern_variable ----------------
pub type Pattern_variableContextAll<'input> = Pattern_variableContext<'input>;


pub type Pattern_variableContext<'input> = BaseParserRuleContext<'input,Pattern_variableContextExt<'input>>;

#[derive(Clone)]
pub struct Pattern_variableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Pattern_variableContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Pattern_variableContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_pattern_variable(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_pattern_variable(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Pattern_variableContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_pattern_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for Pattern_variableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pattern_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pattern_variable }
}
antlr_rust::type_id!{Pattern_variableContextExt<'a>}

impl<'input> Pattern_variableContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Pattern_variableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Pattern_variableContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Pattern_variableContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Pattern_variableContextExt<'input>>{

fn variable_concept(&self) -> Option<Rc<Variable_conceptContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable_type(&self) -> Option<Rc<Variable_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable_thing_any(&self) -> Option<Rc<Variable_thing_anyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Pattern_variableContextAttrs<'input> for Pattern_variableContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pattern_variable(&mut self,)
	-> Result<Rc<Pattern_variableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Pattern_variableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_pattern_variable);
        let mut _localctx: Rc<Pattern_variableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(351);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(16,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule variable_concept*/
					recog.base.set_state(348);
					recog.variable_concept()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule variable_type*/
					recog.base.set_state(349);
					recog.variable_type()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule variable_thing_any*/
					recog.base.set_state(350);
					recog.variable_thing_any()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- variable_concept ----------------
pub type Variable_conceptContextAll<'input> = Variable_conceptContext<'input>;


pub type Variable_conceptContext<'input> = BaseParserRuleContext<'input,Variable_conceptContextExt<'input>>;

#[derive(Clone)]
pub struct Variable_conceptContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Variable_conceptContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Variable_conceptContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_variable_concept(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_variable_concept(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Variable_conceptContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_variable_concept(self);
	}
}

impl<'input> CustomRuleContext<'input> for Variable_conceptContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable_concept }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable_concept }
}
antlr_rust::type_id!{Variable_conceptContextExt<'a>}

impl<'input> Variable_conceptContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Variable_conceptContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Variable_conceptContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Variable_conceptContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Variable_conceptContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token VAR_ in current rule
fn VAR__all(&self) -> Vec<Rc<TerminalNode<'input,TypeQLRustParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token VAR_, starting from 0.
/// Returns `None` if number of children corresponding to token VAR_ is less or equal than `i`.
fn VAR_(&self, i: usize) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VAR_, i)
}
/// Retrieves first TerminalNode corresponding to token IS
/// Returns `None` if there is no child corresponding to token IS
fn IS(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(IS, 0)
}

}

impl<'input> Variable_conceptContextAttrs<'input> for Variable_conceptContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variable_concept(&mut self,)
	-> Result<Rc<Variable_conceptContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Variable_conceptContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_variable_concept);
        let mut _localctx: Rc<Variable_conceptContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(353);
			recog.base.match_token(VAR_,&mut recog.err_handler)?;

			recog.base.set_state(354);
			recog.base.match_token(IS,&mut recog.err_handler)?;

			recog.base.set_state(355);
			recog.base.match_token(VAR_,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- variable_type ----------------
pub type Variable_typeContextAll<'input> = Variable_typeContext<'input>;


pub type Variable_typeContext<'input> = BaseParserRuleContext<'input,Variable_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Variable_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Variable_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Variable_typeContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_variable_type(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_variable_type(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Variable_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_variable_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Variable_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable_type }
}
antlr_rust::type_id!{Variable_typeContextExt<'a>}

impl<'input> Variable_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Variable_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Variable_typeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Variable_typeContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Variable_typeContextExt<'input>>{

fn type_any(&self) -> Option<Rc<Type_anyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn type_constraint_all(&self) ->  Vec<Rc<Type_constraintContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn type_constraint(&self, i: usize) -> Option<Rc<Type_constraintContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Variable_typeContextAttrs<'input> for Variable_typeContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variable_type(&mut self,)
	-> Result<Rc<Variable_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Variable_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_variable_type);
        let mut _localctx: Rc<Variable_typeContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule type_any*/
			recog.base.set_state(357);
			recog.type_any()?;

			/*InvokeRule type_constraint*/
			recog.base.set_state(358);
			recog.type_constraint()?;

			recog.base.set_state(363);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__1 {
				{
				{
				recog.base.set_state(359);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				/*InvokeRule type_constraint*/
				recog.base.set_state(360);
				recog.type_constraint()?;

				}
				}
				recog.base.set_state(365);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- type_constraint ----------------
pub type Type_constraintContextAll<'input> = Type_constraintContext<'input>;


pub type Type_constraintContext<'input> = BaseParserRuleContext<'input,Type_constraintContextExt<'input>>;

#[derive(Clone)]
pub struct Type_constraintContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Type_constraintContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Type_constraintContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_type_constraint(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_type_constraint(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Type_constraintContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_type_constraint(self);
	}
}

impl<'input> CustomRuleContext<'input> for Type_constraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type_constraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type_constraint }
}
antlr_rust::type_id!{Type_constraintContextExt<'a>}

impl<'input> Type_constraintContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Type_constraintContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type_constraintContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Type_constraintContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Type_constraintContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ABSTRACT
/// Returns `None` if there is no child corresponding to token ABSTRACT
fn ABSTRACT(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(ABSTRACT, 0)
}
/// Retrieves first TerminalNode corresponding to token SUB_
/// Returns `None` if there is no child corresponding to token SUB_
fn SUB_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(SUB_, 0)
}
fn type_any(&self) -> Option<Rc<Type_anyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OWNS
/// Returns `None` if there is no child corresponding to token OWNS
fn OWNS(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(OWNS, 0)
}
fn type__all(&self) ->  Vec<Rc<Type_ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn type_(&self, i: usize) -> Option<Rc<Type_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token AS
/// Returns `None` if there is no child corresponding to token AS
fn AS(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(AS, 0)
}
/// Retrieves first TerminalNode corresponding to token IS_KEY
/// Returns `None` if there is no child corresponding to token IS_KEY
fn IS_KEY(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(IS_KEY, 0)
}
/// Retrieves first TerminalNode corresponding to token RELATES
/// Returns `None` if there is no child corresponding to token RELATES
fn RELATES(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(RELATES, 0)
}
/// Retrieves first TerminalNode corresponding to token PLAYS
/// Returns `None` if there is no child corresponding to token PLAYS
fn PLAYS(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(PLAYS, 0)
}
fn type_scoped(&self) -> Option<Rc<Type_scopedContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token VALUE
/// Returns `None` if there is no child corresponding to token VALUE
fn VALUE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VALUE, 0)
}
fn value_type(&self) -> Option<Rc<Value_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token REGEX
/// Returns `None` if there is no child corresponding to token REGEX
fn REGEX(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(REGEX, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_
/// Returns `None` if there is no child corresponding to token STRING_
fn STRING_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(STRING_, 0)
}
/// Retrieves first TerminalNode corresponding to token WHEN
/// Returns `None` if there is no child corresponding to token WHEN
fn WHEN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(WHEN, 0)
}
fn patterns(&self) -> Option<Rc<PatternsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token THEN
/// Returns `None` if there is no child corresponding to token THEN
fn THEN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(THEN, 0)
}
fn variable_things(&self) -> Option<Rc<Variable_thingsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token TYPE
/// Returns `None` if there is no child corresponding to token TYPE
fn TYPE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(TYPE, 0)
}
fn label_any(&self) -> Option<Rc<Label_anyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Type_constraintContextAttrs<'input> for Type_constraintContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn type_constraint(&mut self,)
	-> Result<Rc<Type_constraintContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type_constraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_type_constraint);
        let mut _localctx: Rc<Type_constraintContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(406);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ABSTRACT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(366);
					recog.base.match_token(ABSTRACT,&mut recog.err_handler)?;

					}
				}

			 SUB_ 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(367);
					recog.base.match_token(SUB_,&mut recog.err_handler)?;

					/*InvokeRule type_any*/
					recog.base.set_state(368);
					recog.type_any()?;

					}
				}

			 OWNS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(369);
					recog.base.match_token(OWNS,&mut recog.err_handler)?;

					/*InvokeRule type_*/
					recog.base.set_state(370);
					recog.type_()?;

					recog.base.set_state(373);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==AS {
						{
						recog.base.set_state(371);
						recog.base.match_token(AS,&mut recog.err_handler)?;

						/*InvokeRule type_*/
						recog.base.set_state(372);
						recog.type_()?;

						}
					}

					recog.base.set_state(376);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==IS_KEY {
						{
						recog.base.set_state(375);
						recog.base.match_token(IS_KEY,&mut recog.err_handler)?;

						}
					}

					}
				}

			 RELATES 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(378);
					recog.base.match_token(RELATES,&mut recog.err_handler)?;

					/*InvokeRule type_*/
					recog.base.set_state(379);
					recog.type_()?;

					recog.base.set_state(382);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==AS {
						{
						recog.base.set_state(380);
						recog.base.match_token(AS,&mut recog.err_handler)?;

						/*InvokeRule type_*/
						recog.base.set_state(381);
						recog.type_()?;

						}
					}

					}
				}

			 PLAYS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(384);
					recog.base.match_token(PLAYS,&mut recog.err_handler)?;

					/*InvokeRule type_scoped*/
					recog.base.set_state(385);
					recog.type_scoped()?;

					recog.base.set_state(388);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==AS {
						{
						recog.base.set_state(386);
						recog.base.match_token(AS,&mut recog.err_handler)?;

						/*InvokeRule type_*/
						recog.base.set_state(387);
						recog.type_()?;

						}
					}

					}
				}

			 VALUE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(390);
					recog.base.match_token(VALUE,&mut recog.err_handler)?;

					/*InvokeRule value_type*/
					recog.base.set_state(391);
					recog.value_type()?;

					}
				}

			 REGEX 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(392);
					recog.base.match_token(REGEX,&mut recog.err_handler)?;

					recog.base.set_state(393);
					recog.base.match_token(STRING_,&mut recog.err_handler)?;

					}
				}

			 WHEN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					recog.base.set_state(394);
					recog.base.match_token(WHEN,&mut recog.err_handler)?;

					recog.base.set_state(395);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					/*InvokeRule patterns*/
					recog.base.set_state(396);
					recog.patterns()?;

					recog.base.set_state(397);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					}
				}

			 THEN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					recog.base.set_state(399);
					recog.base.match_token(THEN,&mut recog.err_handler)?;

					recog.base.set_state(400);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					/*InvokeRule variable_things*/
					recog.base.set_state(401);
					recog.variable_things()?;

					recog.base.set_state(402);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					}
				}

			 TYPE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					recog.base.set_state(404);
					recog.base.match_token(TYPE,&mut recog.err_handler)?;

					/*InvokeRule label_any*/
					recog.base.set_state(405);
					recog.label_any()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- variable_things ----------------
pub type Variable_thingsContextAll<'input> = Variable_thingsContext<'input>;


pub type Variable_thingsContext<'input> = BaseParserRuleContext<'input,Variable_thingsContextExt<'input>>;

#[derive(Clone)]
pub struct Variable_thingsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Variable_thingsContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Variable_thingsContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_variable_things(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_variable_things(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Variable_thingsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_variable_things(self);
	}
}

impl<'input> CustomRuleContext<'input> for Variable_thingsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable_things }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable_things }
}
antlr_rust::type_id!{Variable_thingsContextExt<'a>}

impl<'input> Variable_thingsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Variable_thingsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Variable_thingsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Variable_thingsContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Variable_thingsContextExt<'input>>{

fn variable_thing_any_all(&self) ->  Vec<Rc<Variable_thing_anyContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variable_thing_any(&self, i: usize) -> Option<Rc<Variable_thing_anyContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Variable_thingsContextAttrs<'input> for Variable_thingsContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variable_things(&mut self,)
	-> Result<Rc<Variable_thingsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Variable_thingsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_variable_things);
        let mut _localctx: Rc<Variable_thingsContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(411); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule variable_thing_any*/
				recog.base.set_state(408);
				recog.variable_thing_any()?;

				recog.base.set_state(409);
				recog.base.match_token(T__0,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(413); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__4) | (1usize << EQ) | (1usize << NEQ) | (1usize << GT) | (1usize << GTE) | (1usize << LT) | (1usize << LTE) | (1usize << LIKE) | (1usize << CONTAINS))) != 0) || ((((_la - 87)) & !0x3f) == 0 && ((1usize << (_la - 87)) & ((1usize << (BOOLEAN_ - 87)) | (1usize << (STRING_ - 87)) | (1usize << (LONG_ - 87)) | (1usize << (DOUBLE_ - 87)) | (1usize << (DATE_ - 87)) | (1usize << (DATETIME_ - 87)) | (1usize << (VAR_ - 87)))) != 0)) {break}
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- variable_thing_any ----------------
pub type Variable_thing_anyContextAll<'input> = Variable_thing_anyContext<'input>;


pub type Variable_thing_anyContext<'input> = BaseParserRuleContext<'input,Variable_thing_anyContextExt<'input>>;

#[derive(Clone)]
pub struct Variable_thing_anyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Variable_thing_anyContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Variable_thing_anyContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_variable_thing_any(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_variable_thing_any(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Variable_thing_anyContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_variable_thing_any(self);
	}
}

impl<'input> CustomRuleContext<'input> for Variable_thing_anyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable_thing_any }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable_thing_any }
}
antlr_rust::type_id!{Variable_thing_anyContextExt<'a>}

impl<'input> Variable_thing_anyContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Variable_thing_anyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Variable_thing_anyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Variable_thing_anyContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Variable_thing_anyContextExt<'input>>{

fn variable_thing(&self) -> Option<Rc<Variable_thingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable_relation(&self) -> Option<Rc<Variable_relationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable_attribute(&self) -> Option<Rc<Variable_attributeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Variable_thing_anyContextAttrs<'input> for Variable_thing_anyContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variable_thing_any(&mut self,)
	-> Result<Rc<Variable_thing_anyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Variable_thing_anyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_variable_thing_any);
        let mut _localctx: Rc<Variable_thing_anyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(418);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(24,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule variable_thing*/
					recog.base.set_state(415);
					recog.variable_thing()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule variable_relation*/
					recog.base.set_state(416);
					recog.variable_relation()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule variable_attribute*/
					recog.base.set_state(417);
					recog.variable_attribute()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- variable_thing ----------------
pub type Variable_thingContextAll<'input> = Variable_thingContext<'input>;


pub type Variable_thingContext<'input> = BaseParserRuleContext<'input,Variable_thingContextExt<'input>>;

#[derive(Clone)]
pub struct Variable_thingContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Variable_thingContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Variable_thingContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_variable_thing(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_variable_thing(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Variable_thingContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_variable_thing(self);
	}
}

impl<'input> CustomRuleContext<'input> for Variable_thingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable_thing }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable_thing }
}
antlr_rust::type_id!{Variable_thingContextExt<'a>}

impl<'input> Variable_thingContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Variable_thingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Variable_thingContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Variable_thingContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Variable_thingContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VAR_
/// Returns `None` if there is no child corresponding to token VAR_
fn VAR_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VAR_, 0)
}
/// Retrieves first TerminalNode corresponding to token ISA_
/// Returns `None` if there is no child corresponding to token ISA_
fn ISA_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(ISA_, 0)
}
fn type_(&self) -> Option<Rc<Type_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn attributes(&self) -> Option<Rc<AttributesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IID
/// Returns `None` if there is no child corresponding to token IID
fn IID(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(IID, 0)
}
/// Retrieves first TerminalNode corresponding to token IID_
/// Returns `None` if there is no child corresponding to token IID_
fn IID_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(IID_, 0)
}

}

impl<'input> Variable_thingContextAttrs<'input> for Variable_thingContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variable_thing(&mut self,)
	-> Result<Rc<Variable_thingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Variable_thingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_variable_thing);
        let mut _localctx: Rc<Variable_thingContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(436);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(27,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(420);
					recog.base.match_token(VAR_,&mut recog.err_handler)?;

					recog.base.set_state(421);
					recog.base.match_token(ISA_,&mut recog.err_handler)?;

					/*InvokeRule type_*/
					recog.base.set_state(422);
					recog.type_()?;

					recog.base.set_state(425);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__1 {
						{
						recog.base.set_state(423);
						recog.base.match_token(T__1,&mut recog.err_handler)?;

						/*InvokeRule attributes*/
						recog.base.set_state(424);
						recog.attributes()?;

						}
					}

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(427);
					recog.base.match_token(VAR_,&mut recog.err_handler)?;

					recog.base.set_state(428);
					recog.base.match_token(IID,&mut recog.err_handler)?;

					recog.base.set_state(429);
					recog.base.match_token(IID_,&mut recog.err_handler)?;

					recog.base.set_state(432);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__1 {
						{
						recog.base.set_state(430);
						recog.base.match_token(T__1,&mut recog.err_handler)?;

						/*InvokeRule attributes*/
						recog.base.set_state(431);
						recog.attributes()?;

						}
					}

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(434);
					recog.base.match_token(VAR_,&mut recog.err_handler)?;

					/*InvokeRule attributes*/
					recog.base.set_state(435);
					recog.attributes()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- variable_relation ----------------
pub type Variable_relationContextAll<'input> = Variable_relationContext<'input>;


pub type Variable_relationContext<'input> = BaseParserRuleContext<'input,Variable_relationContextExt<'input>>;

#[derive(Clone)]
pub struct Variable_relationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Variable_relationContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Variable_relationContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_variable_relation(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_variable_relation(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Variable_relationContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_variable_relation(self);
	}
}

impl<'input> CustomRuleContext<'input> for Variable_relationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable_relation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable_relation }
}
antlr_rust::type_id!{Variable_relationContextExt<'a>}

impl<'input> Variable_relationContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Variable_relationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Variable_relationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Variable_relationContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Variable_relationContextExt<'input>>{

fn relation(&self) -> Option<Rc<RelationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ISA_
/// Returns `None` if there is no child corresponding to token ISA_
fn ISA_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(ISA_, 0)
}
fn type_(&self) -> Option<Rc<Type_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token VAR_
/// Returns `None` if there is no child corresponding to token VAR_
fn VAR_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VAR_, 0)
}
fn attributes(&self) -> Option<Rc<AttributesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Variable_relationContextAttrs<'input> for Variable_relationContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variable_relation(&mut self,)
	-> Result<Rc<Variable_relationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Variable_relationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_variable_relation);
        let mut _localctx: Rc<Variable_relationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(455);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(32,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(439);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==VAR_ {
						{
						recog.base.set_state(438);
						recog.base.match_token(VAR_,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule relation*/
					recog.base.set_state(441);
					recog.relation()?;

					recog.base.set_state(442);
					recog.base.match_token(ISA_,&mut recog.err_handler)?;

					/*InvokeRule type_*/
					recog.base.set_state(443);
					recog.type_()?;

					recog.base.set_state(446);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__1 {
						{
						recog.base.set_state(444);
						recog.base.match_token(T__1,&mut recog.err_handler)?;

						/*InvokeRule attributes*/
						recog.base.set_state(445);
						recog.attributes()?;

						}
					}

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(449);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==VAR_ {
						{
						recog.base.set_state(448);
						recog.base.match_token(VAR_,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule relation*/
					recog.base.set_state(451);
					recog.relation()?;

					recog.base.set_state(453);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==HAS {
						{
						/*InvokeRule attributes*/
						recog.base.set_state(452);
						recog.attributes()?;

						}
					}

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- variable_attribute ----------------
pub type Variable_attributeContextAll<'input> = Variable_attributeContext<'input>;


pub type Variable_attributeContext<'input> = BaseParserRuleContext<'input,Variable_attributeContextExt<'input>>;

#[derive(Clone)]
pub struct Variable_attributeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Variable_attributeContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Variable_attributeContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_variable_attribute(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_variable_attribute(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Variable_attributeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_variable_attribute(self);
	}
}

impl<'input> CustomRuleContext<'input> for Variable_attributeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable_attribute }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable_attribute }
}
antlr_rust::type_id!{Variable_attributeContextExt<'a>}

impl<'input> Variable_attributeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Variable_attributeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Variable_attributeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Variable_attributeContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Variable_attributeContextExt<'input>>{

fn predicate(&self) -> Option<Rc<PredicateContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ISA_
/// Returns `None` if there is no child corresponding to token ISA_
fn ISA_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(ISA_, 0)
}
fn type_(&self) -> Option<Rc<Type_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token VAR_
/// Returns `None` if there is no child corresponding to token VAR_
fn VAR_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VAR_, 0)
}
fn attributes(&self) -> Option<Rc<AttributesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Variable_attributeContextAttrs<'input> for Variable_attributeContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variable_attribute(&mut self,)
	-> Result<Rc<Variable_attributeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Variable_attributeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_variable_attribute);
        let mut _localctx: Rc<Variable_attributeContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(474);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(37,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(458);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==VAR_ {
						{
						recog.base.set_state(457);
						recog.base.match_token(VAR_,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule predicate*/
					recog.base.set_state(460);
					recog.predicate()?;

					recog.base.set_state(461);
					recog.base.match_token(ISA_,&mut recog.err_handler)?;

					/*InvokeRule type_*/
					recog.base.set_state(462);
					recog.type_()?;

					recog.base.set_state(465);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__1 {
						{
						recog.base.set_state(463);
						recog.base.match_token(T__1,&mut recog.err_handler)?;

						/*InvokeRule attributes*/
						recog.base.set_state(464);
						recog.attributes()?;

						}
					}

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(468);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==VAR_ {
						{
						recog.base.set_state(467);
						recog.base.match_token(VAR_,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule predicate*/
					recog.base.set_state(470);
					recog.predicate()?;

					recog.base.set_state(472);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==HAS {
						{
						/*InvokeRule attributes*/
						recog.base.set_state(471);
						recog.attributes()?;

						}
					}

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- relation ----------------
pub type RelationContextAll<'input> = RelationContext<'input>;


pub type RelationContext<'input> = BaseParserRuleContext<'input,RelationContextExt<'input>>;

#[derive(Clone)]
pub struct RelationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for RelationContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for RelationContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relation(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_relation(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for RelationContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_relation(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation }
}
antlr_rust::type_id!{RelationContextExt<'a>}

impl<'input> RelationContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RelationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RelationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RelationContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<RelationContextExt<'input>>{

fn role_player_all(&self) ->  Vec<Rc<Role_playerContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn role_player(&self, i: usize) -> Option<Rc<Role_playerContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RelationContextAttrs<'input> for RelationContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn relation(&mut self,)
	-> Result<Rc<RelationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RelationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_relation);
        let mut _localctx: Rc<RelationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(476);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

			/*InvokeRule role_player*/
			recog.base.set_state(477);
			recog.role_player()?;

			recog.base.set_state(482);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__1 {
				{
				{
				recog.base.set_state(478);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				/*InvokeRule role_player*/
				recog.base.set_state(479);
				recog.role_player()?;

				}
				}
				recog.base.set_state(484);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(485);
			recog.base.match_token(T__5,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- role_player ----------------
pub type Role_playerContextAll<'input> = Role_playerContext<'input>;


pub type Role_playerContext<'input> = BaseParserRuleContext<'input,Role_playerContextExt<'input>>;

#[derive(Clone)]
pub struct Role_playerContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Role_playerContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Role_playerContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_role_player(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_role_player(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Role_playerContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_role_player(self);
	}
}

impl<'input> CustomRuleContext<'input> for Role_playerContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_role_player }
	//fn type_rule_index() -> usize where Self: Sized { RULE_role_player }
}
antlr_rust::type_id!{Role_playerContextExt<'a>}

impl<'input> Role_playerContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Role_playerContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Role_playerContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Role_playerContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Role_playerContextExt<'input>>{

fn type_(&self) -> Option<Rc<Type_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn player(&self) -> Option<Rc<PlayerContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Role_playerContextAttrs<'input> for Role_playerContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn role_player(&mut self,)
	-> Result<Rc<Role_playerContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Role_playerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_role_player);
        let mut _localctx: Rc<Role_playerContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(492);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(39,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule type_*/
					recog.base.set_state(487);
					recog.type_()?;

					recog.base.set_state(488);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					/*InvokeRule player*/
					recog.base.set_state(489);
					recog.player()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule player*/
					recog.base.set_state(491);
					recog.player()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- player ----------------
pub type PlayerContextAll<'input> = PlayerContext<'input>;


pub type PlayerContext<'input> = BaseParserRuleContext<'input,PlayerContextExt<'input>>;

#[derive(Clone)]
pub struct PlayerContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for PlayerContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for PlayerContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_player(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_player(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for PlayerContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_player(self);
	}
}

impl<'input> CustomRuleContext<'input> for PlayerContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_player }
	//fn type_rule_index() -> usize where Self: Sized { RULE_player }
}
antlr_rust::type_id!{PlayerContextExt<'a>}

impl<'input> PlayerContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PlayerContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PlayerContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PlayerContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<PlayerContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VAR_
/// Returns `None` if there is no child corresponding to token VAR_
fn VAR_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VAR_, 0)
}

}

impl<'input> PlayerContextAttrs<'input> for PlayerContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn player(&mut self,)
	-> Result<Rc<PlayerContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PlayerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_player);
        let mut _localctx: Rc<PlayerContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(494);
			recog.base.match_token(VAR_,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- attributes ----------------
pub type AttributesContextAll<'input> = AttributesContext<'input>;


pub type AttributesContext<'input> = BaseParserRuleContext<'input,AttributesContextExt<'input>>;

#[derive(Clone)]
pub struct AttributesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for AttributesContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for AttributesContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_attributes(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_attributes(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for AttributesContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_attributes(self);
	}
}

impl<'input> CustomRuleContext<'input> for AttributesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_attributes }
	//fn type_rule_index() -> usize where Self: Sized { RULE_attributes }
}
antlr_rust::type_id!{AttributesContextExt<'a>}

impl<'input> AttributesContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AttributesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AttributesContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AttributesContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<AttributesContextExt<'input>>{

fn attribute_all(&self) ->  Vec<Rc<AttributeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn attribute(&self, i: usize) -> Option<Rc<AttributeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> AttributesContextAttrs<'input> for AttributesContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn attributes(&mut self,)
	-> Result<Rc<AttributesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AttributesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_attributes);
        let mut _localctx: Rc<AttributesContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule attribute*/
			recog.base.set_state(496);
			recog.attribute()?;

			recog.base.set_state(501);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__1 {
				{
				{
				recog.base.set_state(497);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				/*InvokeRule attribute*/
				recog.base.set_state(498);
				recog.attribute()?;

				}
				}
				recog.base.set_state(503);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- attribute ----------------
pub type AttributeContextAll<'input> = AttributeContext<'input>;


pub type AttributeContext<'input> = BaseParserRuleContext<'input,AttributeContextExt<'input>>;

#[derive(Clone)]
pub struct AttributeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for AttributeContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for AttributeContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_attribute(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_attribute(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for AttributeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_attribute(self);
	}
}

impl<'input> CustomRuleContext<'input> for AttributeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_attribute }
	//fn type_rule_index() -> usize where Self: Sized { RULE_attribute }
}
antlr_rust::type_id!{AttributeContextExt<'a>}

impl<'input> AttributeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AttributeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AttributeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AttributeContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<AttributeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token HAS
/// Returns `None` if there is no child corresponding to token HAS
fn HAS(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(HAS, 0)
}
fn label(&self) -> Option<Rc<LabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token VAR_
/// Returns `None` if there is no child corresponding to token VAR_
fn VAR_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VAR_, 0)
}
fn predicate(&self) -> Option<Rc<PredicateContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AttributeContextAttrs<'input> for AttributeContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn attribute(&mut self,)
	-> Result<Rc<AttributeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AttributeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_attribute);
        let mut _localctx: Rc<AttributeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(512);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(42,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(504);
					recog.base.match_token(HAS,&mut recog.err_handler)?;

					/*InvokeRule label*/
					recog.base.set_state(505);
					recog.label()?;

					recog.base.set_state(508);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 VAR_ 
						=> {
							{
							recog.base.set_state(506);
							recog.base.match_token(VAR_,&mut recog.err_handler)?;

							}
						}

					 EQ | NEQ | GT | GTE | LT | LTE | LIKE | CONTAINS | BOOLEAN_ | STRING_ |
					 LONG_ | DOUBLE_ | DATE_ | DATETIME_ 
						=> {
							{
							/*InvokeRule predicate*/
							recog.base.set_state(507);
							recog.predicate()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(510);
					recog.base.match_token(HAS,&mut recog.err_handler)?;

					recog.base.set_state(511);
					recog.base.match_token(VAR_,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- predicate ----------------
pub type PredicateContextAll<'input> = PredicateContext<'input>;


pub type PredicateContext<'input> = BaseParserRuleContext<'input,PredicateContextExt<'input>>;

#[derive(Clone)]
pub struct PredicateContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for PredicateContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for PredicateContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_predicate(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_predicate(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for PredicateContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_predicate(self);
	}
}

impl<'input> CustomRuleContext<'input> for PredicateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_predicate }
	//fn type_rule_index() -> usize where Self: Sized { RULE_predicate }
}
antlr_rust::type_id!{PredicateContextExt<'a>}

impl<'input> PredicateContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PredicateContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PredicateContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PredicateContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<PredicateContextExt<'input>>{

fn value(&self) -> Option<Rc<ValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn predicate_equality(&self) -> Option<Rc<Predicate_equalityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn predicate_value(&self) -> Option<Rc<Predicate_valueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn predicate_substring(&self) -> Option<Rc<Predicate_substringContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token STRING_
/// Returns `None` if there is no child corresponding to token STRING_
fn STRING_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(STRING_, 0)
}

}

impl<'input> PredicateContextAttrs<'input> for PredicateContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn predicate(&mut self,)
	-> Result<Rc<PredicateContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PredicateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_predicate);
        let mut _localctx: Rc<PredicateContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(521);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BOOLEAN_ | STRING_ | LONG_ | DOUBLE_ | DATE_ | DATETIME_ 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule value*/
					recog.base.set_state(514);
					recog.value()?;

					}
				}

			 EQ | NEQ | GT | GTE | LT | LTE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule predicate_equality*/
					recog.base.set_state(515);
					recog.predicate_equality()?;

					/*InvokeRule predicate_value*/
					recog.base.set_state(516);
					recog.predicate_value()?;

					}
				}

			 LIKE | CONTAINS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule predicate_substring*/
					recog.base.set_state(518);
					recog.predicate_substring()?;

					recog.base.set_state(519);
					recog.base.match_token(STRING_,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- predicate_equality ----------------
pub type Predicate_equalityContextAll<'input> = Predicate_equalityContext<'input>;


pub type Predicate_equalityContext<'input> = BaseParserRuleContext<'input,Predicate_equalityContextExt<'input>>;

#[derive(Clone)]
pub struct Predicate_equalityContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Predicate_equalityContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Predicate_equalityContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_predicate_equality(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_predicate_equality(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Predicate_equalityContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_predicate_equality(self);
	}
}

impl<'input> CustomRuleContext<'input> for Predicate_equalityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_predicate_equality }
	//fn type_rule_index() -> usize where Self: Sized { RULE_predicate_equality }
}
antlr_rust::type_id!{Predicate_equalityContextExt<'a>}

impl<'input> Predicate_equalityContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Predicate_equalityContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Predicate_equalityContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Predicate_equalityContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Predicate_equalityContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
/// Retrieves first TerminalNode corresponding to token NEQ
/// Returns `None` if there is no child corresponding to token NEQ
fn NEQ(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(NEQ, 0)
}
/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(GT, 0)
}
/// Retrieves first TerminalNode corresponding to token GTE
/// Returns `None` if there is no child corresponding to token GTE
fn GTE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(GTE, 0)
}
/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(LT, 0)
}
/// Retrieves first TerminalNode corresponding to token LTE
/// Returns `None` if there is no child corresponding to token LTE
fn LTE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(LTE, 0)
}

}

impl<'input> Predicate_equalityContextAttrs<'input> for Predicate_equalityContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn predicate_equality(&mut self,)
	-> Result<Rc<Predicate_equalityContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Predicate_equalityContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_predicate_equality);
        let mut _localctx: Rc<Predicate_equalityContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(523);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << EQ) | (1usize << NEQ) | (1usize << GT) | (1usize << GTE) | (1usize << LT) | (1usize << LTE))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- predicate_substring ----------------
pub type Predicate_substringContextAll<'input> = Predicate_substringContext<'input>;


pub type Predicate_substringContext<'input> = BaseParserRuleContext<'input,Predicate_substringContextExt<'input>>;

#[derive(Clone)]
pub struct Predicate_substringContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Predicate_substringContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Predicate_substringContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_predicate_substring(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_predicate_substring(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Predicate_substringContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_predicate_substring(self);
	}
}

impl<'input> CustomRuleContext<'input> for Predicate_substringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_predicate_substring }
	//fn type_rule_index() -> usize where Self: Sized { RULE_predicate_substring }
}
antlr_rust::type_id!{Predicate_substringContextExt<'a>}

impl<'input> Predicate_substringContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Predicate_substringContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Predicate_substringContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Predicate_substringContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Predicate_substringContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CONTAINS
/// Returns `None` if there is no child corresponding to token CONTAINS
fn CONTAINS(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(CONTAINS, 0)
}
/// Retrieves first TerminalNode corresponding to token LIKE
/// Returns `None` if there is no child corresponding to token LIKE
fn LIKE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(LIKE, 0)
}

}

impl<'input> Predicate_substringContextAttrs<'input> for Predicate_substringContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn predicate_substring(&mut self,)
	-> Result<Rc<Predicate_substringContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Predicate_substringContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_predicate_substring);
        let mut _localctx: Rc<Predicate_substringContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(525);
			_la = recog.base.input.la(1);
			if { !(_la==LIKE || _la==CONTAINS) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- predicate_value ----------------
pub type Predicate_valueContextAll<'input> = Predicate_valueContext<'input>;


pub type Predicate_valueContext<'input> = BaseParserRuleContext<'input,Predicate_valueContextExt<'input>>;

#[derive(Clone)]
pub struct Predicate_valueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Predicate_valueContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Predicate_valueContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_predicate_value(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_predicate_value(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Predicate_valueContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_predicate_value(self);
	}
}

impl<'input> CustomRuleContext<'input> for Predicate_valueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_predicate_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_predicate_value }
}
antlr_rust::type_id!{Predicate_valueContextExt<'a>}

impl<'input> Predicate_valueContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Predicate_valueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Predicate_valueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Predicate_valueContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Predicate_valueContextExt<'input>>{

fn value(&self) -> Option<Rc<ValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token VAR_
/// Returns `None` if there is no child corresponding to token VAR_
fn VAR_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VAR_, 0)
}

}

impl<'input> Predicate_valueContextAttrs<'input> for Predicate_valueContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn predicate_value(&mut self,)
	-> Result<Rc<Predicate_valueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Predicate_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_predicate_value);
        let mut _localctx: Rc<Predicate_valueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(529);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BOOLEAN_ | STRING_ | LONG_ | DOUBLE_ | DATE_ | DATETIME_ 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule value*/
					recog.base.set_state(527);
					recog.value()?;

					}
				}

			 VAR_ 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(528);
					recog.base.match_token(VAR_,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- schema_rule ----------------
pub type Schema_ruleContextAll<'input> = Schema_ruleContext<'input>;


pub type Schema_ruleContext<'input> = BaseParserRuleContext<'input,Schema_ruleContextExt<'input>>;

#[derive(Clone)]
pub struct Schema_ruleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Schema_ruleContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Schema_ruleContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_schema_rule(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_schema_rule(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Schema_ruleContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_schema_rule(self);
	}
}

impl<'input> CustomRuleContext<'input> for Schema_ruleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_schema_rule }
	//fn type_rule_index() -> usize where Self: Sized { RULE_schema_rule }
}
antlr_rust::type_id!{Schema_ruleContextExt<'a>}

impl<'input> Schema_ruleContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Schema_ruleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Schema_ruleContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Schema_ruleContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Schema_ruleContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RULE
/// Returns `None` if there is no child corresponding to token RULE
fn RULE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(RULE, 0)
}
fn label(&self) -> Option<Rc<LabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token WHEN
/// Returns `None` if there is no child corresponding to token WHEN
fn WHEN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(WHEN, 0)
}
fn patterns(&self) -> Option<Rc<PatternsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token THEN
/// Returns `None` if there is no child corresponding to token THEN
fn THEN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(THEN, 0)
}
fn variable_thing_any(&self) -> Option<Rc<Variable_thing_anyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Schema_ruleContextAttrs<'input> for Schema_ruleContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn schema_rule(&mut self,)
	-> Result<Rc<Schema_ruleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Schema_ruleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_schema_rule);
        let mut _localctx: Rc<Schema_ruleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(546);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(45,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(531);
					recog.base.match_token(RULE,&mut recog.err_handler)?;

					/*InvokeRule label*/
					recog.base.set_state(532);
					recog.label()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(533);
					recog.base.match_token(RULE,&mut recog.err_handler)?;

					/*InvokeRule label*/
					recog.base.set_state(534);
					recog.label()?;

					recog.base.set_state(535);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					recog.base.set_state(536);
					recog.base.match_token(WHEN,&mut recog.err_handler)?;

					recog.base.set_state(537);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					/*InvokeRule patterns*/
					recog.base.set_state(538);
					recog.patterns()?;

					recog.base.set_state(539);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					recog.base.set_state(540);
					recog.base.match_token(THEN,&mut recog.err_handler)?;

					recog.base.set_state(541);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					/*InvokeRule variable_thing_any*/
					recog.base.set_state(542);
					recog.variable_thing_any()?;

					recog.base.set_state(543);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					recog.base.set_state(544);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- compute_conditions ----------------
pub type Compute_conditionsContextAll<'input> = Compute_conditionsContext<'input>;


pub type Compute_conditionsContext<'input> = BaseParserRuleContext<'input,Compute_conditionsContextExt<'input>>;

#[derive(Clone)]
pub struct Compute_conditionsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Compute_conditionsContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Compute_conditionsContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_compute_conditions(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_compute_conditions(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Compute_conditionsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_compute_conditions(self);
	}
}

impl<'input> CustomRuleContext<'input> for Compute_conditionsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compute_conditions }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compute_conditions }
}
antlr_rust::type_id!{Compute_conditionsContextExt<'a>}

impl<'input> Compute_conditionsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Compute_conditionsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Compute_conditionsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Compute_conditionsContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Compute_conditionsContextExt<'input>>{

fn conditions_count(&self) -> Option<Rc<Conditions_countContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn conditions_value(&self) -> Option<Rc<Conditions_valueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn conditions_central(&self) -> Option<Rc<Conditions_centralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn conditions_cluster(&self) -> Option<Rc<Conditions_clusterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn conditions_path(&self) -> Option<Rc<Conditions_pathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Compute_conditionsContextAttrs<'input> for Compute_conditionsContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compute_conditions(&mut self,)
	-> Result<Rc<Compute_conditionsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Compute_conditionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_compute_conditions);
        let mut _localctx: Rc<Compute_conditionsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(563);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 COUNT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule conditions_count*/
					recog.base.set_state(548);
					recog.conditions_count()?;

					recog.base.set_state(549);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					}
				}

			 MAX | MIN | MEAN | MEDIAN | STD | SUM 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule conditions_value*/
					recog.base.set_state(551);
					recog.conditions_value()?;

					recog.base.set_state(552);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					}
				}

			 CENTRALITY 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule conditions_central*/
					recog.base.set_state(554);
					recog.conditions_central()?;

					recog.base.set_state(555);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					}
				}

			 CLUSTER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule conditions_cluster*/
					recog.base.set_state(557);
					recog.conditions_cluster()?;

					recog.base.set_state(558);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					}
				}

			 PATH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule conditions_path*/
					recog.base.set_state(560);
					recog.conditions_path()?;

					recog.base.set_state(561);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- compute_method ----------------
pub type Compute_methodContextAll<'input> = Compute_methodContext<'input>;


pub type Compute_methodContext<'input> = BaseParserRuleContext<'input,Compute_methodContextExt<'input>>;

#[derive(Clone)]
pub struct Compute_methodContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Compute_methodContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Compute_methodContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_compute_method(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_compute_method(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Compute_methodContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_compute_method(self);
	}
}

impl<'input> CustomRuleContext<'input> for Compute_methodContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compute_method }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compute_method }
}
antlr_rust::type_id!{Compute_methodContextExt<'a>}

impl<'input> Compute_methodContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Compute_methodContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Compute_methodContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Compute_methodContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Compute_methodContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MIN
/// Returns `None` if there is no child corresponding to token MIN
fn MIN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MIN, 0)
}
/// Retrieves first TerminalNode corresponding to token MAX
/// Returns `None` if there is no child corresponding to token MAX
fn MAX(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MAX, 0)
}
/// Retrieves first TerminalNode corresponding to token MEDIAN
/// Returns `None` if there is no child corresponding to token MEDIAN
fn MEDIAN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MEDIAN, 0)
}
/// Retrieves first TerminalNode corresponding to token MEAN
/// Returns `None` if there is no child corresponding to token MEAN
fn MEAN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MEAN, 0)
}
/// Retrieves first TerminalNode corresponding to token STD
/// Returns `None` if there is no child corresponding to token STD
fn STD(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(STD, 0)
}
/// Retrieves first TerminalNode corresponding to token SUM
/// Returns `None` if there is no child corresponding to token SUM
fn SUM(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(SUM, 0)
}

}

impl<'input> Compute_methodContextAttrs<'input> for Compute_methodContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compute_method(&mut self,)
	-> Result<Rc<Compute_methodContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Compute_methodContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_compute_method);
        let mut _localctx: Rc<Compute_methodContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(565);
			_la = recog.base.input.la(1);
			if { !(((((_la - 61)) & !0x3f) == 0 && ((1usize << (_la - 61)) & ((1usize << (MAX - 61)) | (1usize << (MIN - 61)) | (1usize << (MEAN - 61)) | (1usize << (MEDIAN - 61)) | (1usize << (STD - 61)) | (1usize << (SUM - 61)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- conditions_count ----------------
pub type Conditions_countContextAll<'input> = Conditions_countContext<'input>;


pub type Conditions_countContext<'input> = BaseParserRuleContext<'input,Conditions_countContextExt<'input>>;

#[derive(Clone)]
pub struct Conditions_countContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Conditions_countContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Conditions_countContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_conditions_count(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_conditions_count(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Conditions_countContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_conditions_count(self);
	}
}

impl<'input> CustomRuleContext<'input> for Conditions_countContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_conditions_count }
	//fn type_rule_index() -> usize where Self: Sized { RULE_conditions_count }
}
antlr_rust::type_id!{Conditions_countContextExt<'a>}

impl<'input> Conditions_countContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Conditions_countContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Conditions_countContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Conditions_countContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Conditions_countContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COUNT
/// Returns `None` if there is no child corresponding to token COUNT
fn COUNT(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(COUNT, 0)
}
fn input_count(&self) -> Option<Rc<Input_countContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Conditions_countContextAttrs<'input> for Conditions_countContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn conditions_count(&mut self,)
	-> Result<Rc<Conditions_countContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Conditions_countContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_conditions_count);
        let mut _localctx: Rc<Conditions_countContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(567);
			recog.base.match_token(COUNT,&mut recog.err_handler)?;

			recog.base.set_state(569);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IN {
				{
				/*InvokeRule input_count*/
				recog.base.set_state(568);
				recog.input_count()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- conditions_value ----------------
pub type Conditions_valueContextAll<'input> = Conditions_valueContext<'input>;


pub type Conditions_valueContext<'input> = BaseParserRuleContext<'input,Conditions_valueContextExt<'input>>;

#[derive(Clone)]
pub struct Conditions_valueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Conditions_valueContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Conditions_valueContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_conditions_value(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_conditions_value(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Conditions_valueContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_conditions_value(self);
	}
}

impl<'input> CustomRuleContext<'input> for Conditions_valueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_conditions_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_conditions_value }
}
antlr_rust::type_id!{Conditions_valueContextExt<'a>}

impl<'input> Conditions_valueContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Conditions_valueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Conditions_valueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Conditions_valueContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Conditions_valueContextExt<'input>>{

fn compute_method(&self) -> Option<Rc<Compute_methodContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn input_value_all(&self) ->  Vec<Rc<Input_valueContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn input_value(&self, i: usize) -> Option<Rc<Input_valueContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Conditions_valueContextAttrs<'input> for Conditions_valueContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn conditions_value(&mut self,)
	-> Result<Rc<Conditions_valueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Conditions_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_conditions_value);
        let mut _localctx: Rc<Conditions_valueContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule compute_method*/
			recog.base.set_state(571);
			recog.compute_method()?;

			/*InvokeRule input_value*/
			recog.base.set_state(572);
			recog.input_value()?;

			recog.base.set_state(577);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__1 {
				{
				{
				recog.base.set_state(573);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				/*InvokeRule input_value*/
				recog.base.set_state(574);
				recog.input_value()?;

				}
				}
				recog.base.set_state(579);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- conditions_central ----------------
pub type Conditions_centralContextAll<'input> = Conditions_centralContext<'input>;


pub type Conditions_centralContext<'input> = BaseParserRuleContext<'input,Conditions_centralContextExt<'input>>;

#[derive(Clone)]
pub struct Conditions_centralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Conditions_centralContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Conditions_centralContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_conditions_central(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_conditions_central(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Conditions_centralContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_conditions_central(self);
	}
}

impl<'input> CustomRuleContext<'input> for Conditions_centralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_conditions_central }
	//fn type_rule_index() -> usize where Self: Sized { RULE_conditions_central }
}
antlr_rust::type_id!{Conditions_centralContextExt<'a>}

impl<'input> Conditions_centralContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Conditions_centralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Conditions_centralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Conditions_centralContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Conditions_centralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CENTRALITY
/// Returns `None` if there is no child corresponding to token CENTRALITY
fn CENTRALITY(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(CENTRALITY, 0)
}
fn input_central_all(&self) ->  Vec<Rc<Input_centralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn input_central(&self, i: usize) -> Option<Rc<Input_centralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Conditions_centralContextAttrs<'input> for Conditions_centralContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn conditions_central(&mut self,)
	-> Result<Rc<Conditions_centralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Conditions_centralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_conditions_central);
        let mut _localctx: Rc<Conditions_centralContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(580);
			recog.base.match_token(CENTRALITY,&mut recog.err_handler)?;

			/*InvokeRule input_central*/
			recog.base.set_state(581);
			recog.input_central()?;

			recog.base.set_state(586);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__1 {
				{
				{
				recog.base.set_state(582);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				/*InvokeRule input_central*/
				recog.base.set_state(583);
				recog.input_central()?;

				}
				}
				recog.base.set_state(588);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- conditions_cluster ----------------
pub type Conditions_clusterContextAll<'input> = Conditions_clusterContext<'input>;


pub type Conditions_clusterContext<'input> = BaseParserRuleContext<'input,Conditions_clusterContextExt<'input>>;

#[derive(Clone)]
pub struct Conditions_clusterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Conditions_clusterContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Conditions_clusterContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_conditions_cluster(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_conditions_cluster(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Conditions_clusterContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_conditions_cluster(self);
	}
}

impl<'input> CustomRuleContext<'input> for Conditions_clusterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_conditions_cluster }
	//fn type_rule_index() -> usize where Self: Sized { RULE_conditions_cluster }
}
antlr_rust::type_id!{Conditions_clusterContextExt<'a>}

impl<'input> Conditions_clusterContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Conditions_clusterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Conditions_clusterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Conditions_clusterContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Conditions_clusterContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CLUSTER
/// Returns `None` if there is no child corresponding to token CLUSTER
fn CLUSTER(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(CLUSTER, 0)
}
fn input_cluster_all(&self) ->  Vec<Rc<Input_clusterContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn input_cluster(&self, i: usize) -> Option<Rc<Input_clusterContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Conditions_clusterContextAttrs<'input> for Conditions_clusterContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn conditions_cluster(&mut self,)
	-> Result<Rc<Conditions_clusterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Conditions_clusterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_conditions_cluster);
        let mut _localctx: Rc<Conditions_clusterContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(589);
			recog.base.match_token(CLUSTER,&mut recog.err_handler)?;

			/*InvokeRule input_cluster*/
			recog.base.set_state(590);
			recog.input_cluster()?;

			recog.base.set_state(595);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__1 {
				{
				{
				recog.base.set_state(591);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				/*InvokeRule input_cluster*/
				recog.base.set_state(592);
				recog.input_cluster()?;

				}
				}
				recog.base.set_state(597);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- conditions_path ----------------
pub type Conditions_pathContextAll<'input> = Conditions_pathContext<'input>;


pub type Conditions_pathContext<'input> = BaseParserRuleContext<'input,Conditions_pathContextExt<'input>>;

#[derive(Clone)]
pub struct Conditions_pathContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Conditions_pathContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Conditions_pathContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_conditions_path(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_conditions_path(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Conditions_pathContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_conditions_path(self);
	}
}

impl<'input> CustomRuleContext<'input> for Conditions_pathContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_conditions_path }
	//fn type_rule_index() -> usize where Self: Sized { RULE_conditions_path }
}
antlr_rust::type_id!{Conditions_pathContextExt<'a>}

impl<'input> Conditions_pathContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Conditions_pathContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Conditions_pathContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Conditions_pathContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Conditions_pathContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PATH
/// Returns `None` if there is no child corresponding to token PATH
fn PATH(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(PATH, 0)
}
fn input_path_all(&self) ->  Vec<Rc<Input_pathContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn input_path(&self, i: usize) -> Option<Rc<Input_pathContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Conditions_pathContextAttrs<'input> for Conditions_pathContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn conditions_path(&mut self,)
	-> Result<Rc<Conditions_pathContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Conditions_pathContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_conditions_path);
        let mut _localctx: Rc<Conditions_pathContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(598);
			recog.base.match_token(PATH,&mut recog.err_handler)?;

			/*InvokeRule input_path*/
			recog.base.set_state(599);
			recog.input_path()?;

			recog.base.set_state(604);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__1 {
				{
				{
				recog.base.set_state(600);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				/*InvokeRule input_path*/
				recog.base.set_state(601);
				recog.input_path()?;

				}
				}
				recog.base.set_state(606);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- input_count ----------------
pub type Input_countContextAll<'input> = Input_countContext<'input>;


pub type Input_countContext<'input> = BaseParserRuleContext<'input,Input_countContextExt<'input>>;

#[derive(Clone)]
pub struct Input_countContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Input_countContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Input_countContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_input_count(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_input_count(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Input_countContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_input_count(self);
	}
}

impl<'input> CustomRuleContext<'input> for Input_countContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_input_count }
	//fn type_rule_index() -> usize where Self: Sized { RULE_input_count }
}
antlr_rust::type_id!{Input_countContextExt<'a>}

impl<'input> Input_countContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Input_countContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Input_countContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Input_countContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Input_countContextExt<'input>>{

fn compute_scope(&self) -> Option<Rc<Compute_scopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Input_countContextAttrs<'input> for Input_countContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn input_count(&mut self,)
	-> Result<Rc<Input_countContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Input_countContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_input_count);
        let mut _localctx: Rc<Input_countContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule compute_scope*/
			recog.base.set_state(607);
			recog.compute_scope()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- input_value ----------------
pub type Input_valueContextAll<'input> = Input_valueContext<'input>;


pub type Input_valueContext<'input> = BaseParserRuleContext<'input,Input_valueContextExt<'input>>;

#[derive(Clone)]
pub struct Input_valueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Input_valueContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Input_valueContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_input_value(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_input_value(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Input_valueContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_input_value(self);
	}
}

impl<'input> CustomRuleContext<'input> for Input_valueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_input_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_input_value }
}
antlr_rust::type_id!{Input_valueContextExt<'a>}

impl<'input> Input_valueContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Input_valueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Input_valueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Input_valueContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Input_valueContextExt<'input>>{

fn compute_scope(&self) -> Option<Rc<Compute_scopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn compute_target(&self) -> Option<Rc<Compute_targetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Input_valueContextAttrs<'input> for Input_valueContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn input_value(&mut self,)
	-> Result<Rc<Input_valueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Input_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_input_value);
        let mut _localctx: Rc<Input_valueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(611);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule compute_scope*/
					recog.base.set_state(609);
					recog.compute_scope()?;

					}
				}

			 OF 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule compute_target*/
					recog.base.set_state(610);
					recog.compute_target()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- input_central ----------------
pub type Input_centralContextAll<'input> = Input_centralContext<'input>;


pub type Input_centralContext<'input> = BaseParserRuleContext<'input,Input_centralContextExt<'input>>;

#[derive(Clone)]
pub struct Input_centralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Input_centralContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Input_centralContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_input_central(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_input_central(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Input_centralContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_input_central(self);
	}
}

impl<'input> CustomRuleContext<'input> for Input_centralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_input_central }
	//fn type_rule_index() -> usize where Self: Sized { RULE_input_central }
}
antlr_rust::type_id!{Input_centralContextExt<'a>}

impl<'input> Input_centralContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Input_centralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Input_centralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Input_centralContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Input_centralContextExt<'input>>{

fn compute_scope(&self) -> Option<Rc<Compute_scopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn compute_target(&self) -> Option<Rc<Compute_targetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn compute_config(&self) -> Option<Rc<Compute_configContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Input_centralContextAttrs<'input> for Input_centralContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn input_central(&mut self,)
	-> Result<Rc<Input_centralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Input_centralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_input_central);
        let mut _localctx: Rc<Input_centralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(616);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule compute_scope*/
					recog.base.set_state(613);
					recog.compute_scope()?;

					}
				}

			 OF 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule compute_target*/
					recog.base.set_state(614);
					recog.compute_target()?;

					}
				}

			 USING | WHERE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule compute_config*/
					recog.base.set_state(615);
					recog.compute_config()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- input_cluster ----------------
pub type Input_clusterContextAll<'input> = Input_clusterContext<'input>;


pub type Input_clusterContext<'input> = BaseParserRuleContext<'input,Input_clusterContextExt<'input>>;

#[derive(Clone)]
pub struct Input_clusterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Input_clusterContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Input_clusterContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_input_cluster(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_input_cluster(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Input_clusterContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_input_cluster(self);
	}
}

impl<'input> CustomRuleContext<'input> for Input_clusterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_input_cluster }
	//fn type_rule_index() -> usize where Self: Sized { RULE_input_cluster }
}
antlr_rust::type_id!{Input_clusterContextExt<'a>}

impl<'input> Input_clusterContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Input_clusterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Input_clusterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Input_clusterContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Input_clusterContextExt<'input>>{

fn compute_scope(&self) -> Option<Rc<Compute_scopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn compute_config(&self) -> Option<Rc<Compute_configContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Input_clusterContextAttrs<'input> for Input_clusterContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn input_cluster(&mut self,)
	-> Result<Rc<Input_clusterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Input_clusterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_input_cluster);
        let mut _localctx: Rc<Input_clusterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(620);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule compute_scope*/
					recog.base.set_state(618);
					recog.compute_scope()?;

					}
				}

			 USING | WHERE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule compute_config*/
					recog.base.set_state(619);
					recog.compute_config()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- input_path ----------------
pub type Input_pathContextAll<'input> = Input_pathContext<'input>;


pub type Input_pathContext<'input> = BaseParserRuleContext<'input,Input_pathContextExt<'input>>;

#[derive(Clone)]
pub struct Input_pathContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Input_pathContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Input_pathContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_input_path(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_input_path(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Input_pathContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_input_path(self);
	}
}

impl<'input> CustomRuleContext<'input> for Input_pathContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_input_path }
	//fn type_rule_index() -> usize where Self: Sized { RULE_input_path }
}
antlr_rust::type_id!{Input_pathContextExt<'a>}

impl<'input> Input_pathContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Input_pathContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Input_pathContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Input_pathContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Input_pathContextExt<'input>>{

fn compute_scope(&self) -> Option<Rc<Compute_scopeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn compute_direction(&self) -> Option<Rc<Compute_directionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Input_pathContextAttrs<'input> for Input_pathContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn input_path(&mut self,)
	-> Result<Rc<Input_pathContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Input_pathContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_input_path);
        let mut _localctx: Rc<Input_pathContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(624);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule compute_scope*/
					recog.base.set_state(622);
					recog.compute_scope()?;

					}
				}

			 FROM | TO 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule compute_direction*/
					recog.base.set_state(623);
					recog.compute_direction()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- compute_direction ----------------
pub type Compute_directionContextAll<'input> = Compute_directionContext<'input>;


pub type Compute_directionContext<'input> = BaseParserRuleContext<'input,Compute_directionContextExt<'input>>;

#[derive(Clone)]
pub struct Compute_directionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Compute_directionContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Compute_directionContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_compute_direction(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_compute_direction(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Compute_directionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_compute_direction(self);
	}
}

impl<'input> CustomRuleContext<'input> for Compute_directionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compute_direction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compute_direction }
}
antlr_rust::type_id!{Compute_directionContextExt<'a>}

impl<'input> Compute_directionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Compute_directionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Compute_directionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Compute_directionContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Compute_directionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FROM
/// Returns `None` if there is no child corresponding to token FROM
fn FROM(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(FROM, 0)
}
/// Retrieves first TerminalNode corresponding to token IID_
/// Returns `None` if there is no child corresponding to token IID_
fn IID_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(IID_, 0)
}
/// Retrieves first TerminalNode corresponding to token TO
/// Returns `None` if there is no child corresponding to token TO
fn TO(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(TO, 0)
}

}

impl<'input> Compute_directionContextAttrs<'input> for Compute_directionContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compute_direction(&mut self,)
	-> Result<Rc<Compute_directionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Compute_directionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_compute_direction);
        let mut _localctx: Rc<Compute_directionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(630);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 FROM 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(626);
					recog.base.match_token(FROM,&mut recog.err_handler)?;

					recog.base.set_state(627);
					recog.base.match_token(IID_,&mut recog.err_handler)?;

					}
				}

			 TO 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(628);
					recog.base.match_token(TO,&mut recog.err_handler)?;

					recog.base.set_state(629);
					recog.base.match_token(IID_,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- compute_target ----------------
pub type Compute_targetContextAll<'input> = Compute_targetContext<'input>;


pub type Compute_targetContext<'input> = BaseParserRuleContext<'input,Compute_targetContextExt<'input>>;

#[derive(Clone)]
pub struct Compute_targetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Compute_targetContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Compute_targetContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_compute_target(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_compute_target(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Compute_targetContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_compute_target(self);
	}
}

impl<'input> CustomRuleContext<'input> for Compute_targetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compute_target }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compute_target }
}
antlr_rust::type_id!{Compute_targetContextExt<'a>}

impl<'input> Compute_targetContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Compute_targetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Compute_targetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Compute_targetContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Compute_targetContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OF
/// Returns `None` if there is no child corresponding to token OF
fn OF(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(OF, 0)
}
fn labels(&self) -> Option<Rc<LabelsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Compute_targetContextAttrs<'input> for Compute_targetContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compute_target(&mut self,)
	-> Result<Rc<Compute_targetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Compute_targetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_compute_target);
        let mut _localctx: Rc<Compute_targetContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(632);
			recog.base.match_token(OF,&mut recog.err_handler)?;

			/*InvokeRule labels*/
			recog.base.set_state(633);
			recog.labels()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- compute_scope ----------------
pub type Compute_scopeContextAll<'input> = Compute_scopeContext<'input>;


pub type Compute_scopeContext<'input> = BaseParserRuleContext<'input,Compute_scopeContextExt<'input>>;

#[derive(Clone)]
pub struct Compute_scopeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Compute_scopeContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Compute_scopeContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_compute_scope(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_compute_scope(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Compute_scopeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_compute_scope(self);
	}
}

impl<'input> CustomRuleContext<'input> for Compute_scopeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compute_scope }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compute_scope }
}
antlr_rust::type_id!{Compute_scopeContextExt<'a>}

impl<'input> Compute_scopeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Compute_scopeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Compute_scopeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Compute_scopeContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Compute_scopeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}
fn labels(&self) -> Option<Rc<LabelsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Compute_scopeContextAttrs<'input> for Compute_scopeContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compute_scope(&mut self,)
	-> Result<Rc<Compute_scopeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Compute_scopeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_compute_scope);
        let mut _localctx: Rc<Compute_scopeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(635);
			recog.base.match_token(IN,&mut recog.err_handler)?;

			/*InvokeRule labels*/
			recog.base.set_state(636);
			recog.labels()?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- compute_config ----------------
pub type Compute_configContextAll<'input> = Compute_configContext<'input>;


pub type Compute_configContext<'input> = BaseParserRuleContext<'input,Compute_configContextExt<'input>>;

#[derive(Clone)]
pub struct Compute_configContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Compute_configContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Compute_configContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_compute_config(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_compute_config(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Compute_configContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_compute_config(self);
	}
}

impl<'input> CustomRuleContext<'input> for Compute_configContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compute_config }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compute_config }
}
antlr_rust::type_id!{Compute_configContextExt<'a>}

impl<'input> Compute_configContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Compute_configContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Compute_configContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Compute_configContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Compute_configContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token USING
/// Returns `None` if there is no child corresponding to token USING
fn USING(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(USING, 0)
}
fn compute_algorithm(&self) -> Option<Rc<Compute_algorithmContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token WHERE
/// Returns `None` if there is no child corresponding to token WHERE
fn WHERE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(WHERE, 0)
}
fn compute_args(&self) -> Option<Rc<Compute_argsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Compute_configContextAttrs<'input> for Compute_configContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compute_config(&mut self,)
	-> Result<Rc<Compute_configContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Compute_configContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_compute_config);
        let mut _localctx: Rc<Compute_configContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(642);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 USING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(638);
					recog.base.match_token(USING,&mut recog.err_handler)?;

					/*InvokeRule compute_algorithm*/
					recog.base.set_state(639);
					recog.compute_algorithm()?;

					}
				}

			 WHERE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(640);
					recog.base.match_token(WHERE,&mut recog.err_handler)?;

					/*InvokeRule compute_args*/
					recog.base.set_state(641);
					recog.compute_args()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- compute_algorithm ----------------
pub type Compute_algorithmContextAll<'input> = Compute_algorithmContext<'input>;


pub type Compute_algorithmContext<'input> = BaseParserRuleContext<'input,Compute_algorithmContextExt<'input>>;

#[derive(Clone)]
pub struct Compute_algorithmContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Compute_algorithmContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Compute_algorithmContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_compute_algorithm(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_compute_algorithm(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Compute_algorithmContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_compute_algorithm(self);
	}
}

impl<'input> CustomRuleContext<'input> for Compute_algorithmContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compute_algorithm }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compute_algorithm }
}
antlr_rust::type_id!{Compute_algorithmContextExt<'a>}

impl<'input> Compute_algorithmContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Compute_algorithmContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Compute_algorithmContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Compute_algorithmContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Compute_algorithmContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DEGREE
/// Returns `None` if there is no child corresponding to token DEGREE
fn DEGREE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(DEGREE, 0)
}
/// Retrieves first TerminalNode corresponding to token K_CORE
/// Returns `None` if there is no child corresponding to token K_CORE
fn K_CORE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(K_CORE, 0)
}
/// Retrieves first TerminalNode corresponding to token CONNECTED_COMPONENT
/// Returns `None` if there is no child corresponding to token CONNECTED_COMPONENT
fn CONNECTED_COMPONENT(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(CONNECTED_COMPONENT, 0)
}

}

impl<'input> Compute_algorithmContextAttrs<'input> for Compute_algorithmContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compute_algorithm(&mut self,)
	-> Result<Rc<Compute_algorithmContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Compute_algorithmContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_compute_algorithm);
        let mut _localctx: Rc<Compute_algorithmContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(644);
			_la = recog.base.input.la(1);
			if { !(((((_la - 70)) & !0x3f) == 0 && ((1usize << (_la - 70)) & ((1usize << (DEGREE - 70)) | (1usize << (K_CORE - 70)) | (1usize << (CONNECTED_COMPONENT - 70)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- compute_args ----------------
pub type Compute_argsContextAll<'input> = Compute_argsContext<'input>;


pub type Compute_argsContext<'input> = BaseParserRuleContext<'input,Compute_argsContextExt<'input>>;

#[derive(Clone)]
pub struct Compute_argsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Compute_argsContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Compute_argsContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_compute_args(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_compute_args(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Compute_argsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_compute_args(self);
	}
}

impl<'input> CustomRuleContext<'input> for Compute_argsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compute_args }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compute_args }
}
antlr_rust::type_id!{Compute_argsContextExt<'a>}

impl<'input> Compute_argsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Compute_argsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Compute_argsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Compute_argsContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Compute_argsContextExt<'input>>{

fn compute_arg(&self) -> Option<Rc<Compute_argContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn compute_args_array(&self) -> Option<Rc<Compute_args_arrayContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Compute_argsContextAttrs<'input> for Compute_argsContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compute_args(&mut self,)
	-> Result<Rc<Compute_argsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Compute_argsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_compute_args);
        let mut _localctx: Rc<Compute_argsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(648);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 CONTAINS | MIN_K | K | SIZE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule compute_arg*/
					recog.base.set_state(646);
					recog.compute_arg()?;

					}
				}

			 T__7 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule compute_args_array*/
					recog.base.set_state(647);
					recog.compute_args_array()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- compute_args_array ----------------
pub type Compute_args_arrayContextAll<'input> = Compute_args_arrayContext<'input>;


pub type Compute_args_arrayContext<'input> = BaseParserRuleContext<'input,Compute_args_arrayContextExt<'input>>;

#[derive(Clone)]
pub struct Compute_args_arrayContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Compute_args_arrayContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Compute_args_arrayContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_compute_args_array(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_compute_args_array(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Compute_args_arrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_compute_args_array(self);
	}
}

impl<'input> CustomRuleContext<'input> for Compute_args_arrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compute_args_array }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compute_args_array }
}
antlr_rust::type_id!{Compute_args_arrayContextExt<'a>}

impl<'input> Compute_args_arrayContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Compute_args_arrayContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Compute_args_arrayContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Compute_args_arrayContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Compute_args_arrayContextExt<'input>>{

fn compute_arg_all(&self) ->  Vec<Rc<Compute_argContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn compute_arg(&self, i: usize) -> Option<Rc<Compute_argContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Compute_args_arrayContextAttrs<'input> for Compute_args_arrayContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compute_args_array(&mut self,)
	-> Result<Rc<Compute_args_arrayContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Compute_args_arrayContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_compute_args_array);
        let mut _localctx: Rc<Compute_args_arrayContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(650);
			recog.base.match_token(T__7,&mut recog.err_handler)?;

			/*InvokeRule compute_arg*/
			recog.base.set_state(651);
			recog.compute_arg()?;

			recog.base.set_state(656);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__1 {
				{
				{
				recog.base.set_state(652);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				/*InvokeRule compute_arg*/
				recog.base.set_state(653);
				recog.compute_arg()?;

				}
				}
				recog.base.set_state(658);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(659);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- compute_arg ----------------
pub type Compute_argContextAll<'input> = Compute_argContext<'input>;


pub type Compute_argContext<'input> = BaseParserRuleContext<'input,Compute_argContextExt<'input>>;

#[derive(Clone)]
pub struct Compute_argContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Compute_argContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Compute_argContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_compute_arg(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_compute_arg(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Compute_argContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_compute_arg(self);
	}
}

impl<'input> CustomRuleContext<'input> for Compute_argContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compute_arg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compute_arg }
}
antlr_rust::type_id!{Compute_argContextExt<'a>}

impl<'input> Compute_argContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Compute_argContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Compute_argContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Compute_argContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Compute_argContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MIN_K
/// Returns `None` if there is no child corresponding to token MIN_K
fn MIN_K(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MIN_K, 0)
}
/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
/// Retrieves first TerminalNode corresponding to token LONG_
/// Returns `None` if there is no child corresponding to token LONG_
fn LONG_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(LONG_, 0)
}
/// Retrieves first TerminalNode corresponding to token K
/// Returns `None` if there is no child corresponding to token K
fn K(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(K, 0)
}
/// Retrieves first TerminalNode corresponding to token SIZE
/// Returns `None` if there is no child corresponding to token SIZE
fn SIZE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(SIZE, 0)
}
/// Retrieves first TerminalNode corresponding to token CONTAINS
/// Returns `None` if there is no child corresponding to token CONTAINS
fn CONTAINS(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(CONTAINS, 0)
}
/// Retrieves first TerminalNode corresponding to token IID_
/// Returns `None` if there is no child corresponding to token IID_
fn IID_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(IID_, 0)
}

}

impl<'input> Compute_argContextAttrs<'input> for Compute_argContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compute_arg(&mut self,)
	-> Result<Rc<Compute_argContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Compute_argContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_compute_arg);
        let mut _localctx: Rc<Compute_argContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(673);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 MIN_K 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(661);
					recog.base.match_token(MIN_K,&mut recog.err_handler)?;

					recog.base.set_state(662);
					recog.base.match_token(EQ,&mut recog.err_handler)?;

					recog.base.set_state(663);
					recog.base.match_token(LONG_,&mut recog.err_handler)?;

					}
				}

			 K 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(664);
					recog.base.match_token(K,&mut recog.err_handler)?;

					recog.base.set_state(665);
					recog.base.match_token(EQ,&mut recog.err_handler)?;

					recog.base.set_state(666);
					recog.base.match_token(LONG_,&mut recog.err_handler)?;

					}
				}

			 SIZE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(667);
					recog.base.match_token(SIZE,&mut recog.err_handler)?;

					recog.base.set_state(668);
					recog.base.match_token(EQ,&mut recog.err_handler)?;

					recog.base.set_state(669);
					recog.base.match_token(LONG_,&mut recog.err_handler)?;

					}
				}

			 CONTAINS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(670);
					recog.base.match_token(CONTAINS,&mut recog.err_handler)?;

					recog.base.set_state(671);
					recog.base.match_token(EQ,&mut recog.err_handler)?;

					recog.base.set_state(672);
					recog.base.match_token(IID_,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- type_any ----------------
pub type Type_anyContextAll<'input> = Type_anyContext<'input>;


pub type Type_anyContext<'input> = BaseParserRuleContext<'input,Type_anyContextExt<'input>>;

#[derive(Clone)]
pub struct Type_anyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Type_anyContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Type_anyContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_type_any(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_type_any(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Type_anyContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_type_any(self);
	}
}

impl<'input> CustomRuleContext<'input> for Type_anyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type_any }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type_any }
}
antlr_rust::type_id!{Type_anyContextExt<'a>}

impl<'input> Type_anyContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Type_anyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type_anyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Type_anyContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Type_anyContextExt<'input>>{

fn type_scoped(&self) -> Option<Rc<Type_scopedContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn type_(&self) -> Option<Rc<Type_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token VAR_
/// Returns `None` if there is no child corresponding to token VAR_
fn VAR_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VAR_, 0)
}

}

impl<'input> Type_anyContextAttrs<'input> for Type_anyContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn type_any(&mut self,)
	-> Result<Rc<Type_anyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type_anyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_type_any);
        let mut _localctx: Rc<Type_anyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(678);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(61,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule type_scoped*/
					recog.base.set_state(675);
					recog.type_scoped()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule type_*/
					recog.base.set_state(676);
					recog.type_()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(677);
					recog.base.match_token(VAR_,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- type_scoped ----------------
pub type Type_scopedContextAll<'input> = Type_scopedContext<'input>;


pub type Type_scopedContext<'input> = BaseParserRuleContext<'input,Type_scopedContextExt<'input>>;

#[derive(Clone)]
pub struct Type_scopedContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Type_scopedContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Type_scopedContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_type_scoped(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_type_scoped(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Type_scopedContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_type_scoped(self);
	}
}

impl<'input> CustomRuleContext<'input> for Type_scopedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type_scoped }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type_scoped }
}
antlr_rust::type_id!{Type_scopedContextExt<'a>}

impl<'input> Type_scopedContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Type_scopedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type_scopedContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Type_scopedContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Type_scopedContextExt<'input>>{

fn label_scoped(&self) -> Option<Rc<Label_scopedContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token VAR_
/// Returns `None` if there is no child corresponding to token VAR_
fn VAR_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VAR_, 0)
}

}

impl<'input> Type_scopedContextAttrs<'input> for Type_scopedContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn type_scoped(&mut self,)
	-> Result<Rc<Type_scopedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type_scopedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_type_scoped);
        let mut _localctx: Rc<Type_scopedContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(682);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LABEL_SCOPED_ 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule label_scoped*/
					recog.base.set_state(680);
					recog.label_scoped()?;

					}
				}

			 VAR_ 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(681);
					recog.base.match_token(VAR_,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- type_ ----------------
pub type Type_ContextAll<'input> = Type_Context<'input>;


pub type Type_Context<'input> = BaseParserRuleContext<'input,Type_ContextExt<'input>>;

#[derive(Clone)]
pub struct Type_ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Type_Context<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Type_Context<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_type_(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_type_(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Type_Context<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_type_(self);
	}
}

impl<'input> CustomRuleContext<'input> for Type_ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type_ }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type_ }
}
antlr_rust::type_id!{Type_ContextExt<'a>}

impl<'input> Type_ContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Type_ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type_ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Type_ContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Type_ContextExt<'input>>{

fn label(&self) -> Option<Rc<LabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token VAR_
/// Returns `None` if there is no child corresponding to token VAR_
fn VAR_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VAR_, 0)
}

}

impl<'input> Type_ContextAttrs<'input> for Type_Context<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn type_(&mut self,)
	-> Result<Rc<Type_ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_type_);
        let mut _localctx: Rc<Type_ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(686);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 GET | THING | ENTITY | ATTRIBUTE | RELATION | ROLE | RULE | OFFSET |
			 LIMIT | SORT | VALUE | CONTAINS | GROUP | COUNT | MAX | MIN | MEAN |
			 MEDIAN | STD | SUM | CLUSTER | PATH | DEGREE | K_CORE | CONNECTED_COMPONENT |
			 FROM | TO | OF | IN | WHERE | MIN_K | K | SIZE | LABEL_ 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule label*/
					recog.base.set_state(684);
					recog.label()?;

					}
				}

			 VAR_ 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(685);
					recog.base.match_token(VAR_,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- label_any ----------------
pub type Label_anyContextAll<'input> = Label_anyContext<'input>;


pub type Label_anyContext<'input> = BaseParserRuleContext<'input,Label_anyContextExt<'input>>;

#[derive(Clone)]
pub struct Label_anyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Label_anyContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Label_anyContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_label_any(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_label_any(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Label_anyContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_label_any(self);
	}
}

impl<'input> CustomRuleContext<'input> for Label_anyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_label_any }
	//fn type_rule_index() -> usize where Self: Sized { RULE_label_any }
}
antlr_rust::type_id!{Label_anyContextExt<'a>}

impl<'input> Label_anyContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Label_anyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Label_anyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Label_anyContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Label_anyContextExt<'input>>{

fn label_scoped(&self) -> Option<Rc<Label_scopedContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn label(&self) -> Option<Rc<LabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Label_anyContextAttrs<'input> for Label_anyContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn label_any(&mut self,)
	-> Result<Rc<Label_anyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Label_anyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_label_any);
        let mut _localctx: Rc<Label_anyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(690);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LABEL_SCOPED_ 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule label_scoped*/
					recog.base.set_state(688);
					recog.label_scoped()?;

					}
				}

			 GET | THING | ENTITY | ATTRIBUTE | RELATION | ROLE | RULE | OFFSET |
			 LIMIT | SORT | VALUE | CONTAINS | GROUP | COUNT | MAX | MIN | MEAN |
			 MEDIAN | STD | SUM | CLUSTER | PATH | DEGREE | K_CORE | CONNECTED_COMPONENT |
			 FROM | TO | OF | IN | WHERE | MIN_K | K | SIZE | LABEL_ 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule label*/
					recog.base.set_state(689);
					recog.label()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- label_scoped ----------------
pub type Label_scopedContextAll<'input> = Label_scopedContext<'input>;


pub type Label_scopedContext<'input> = BaseParserRuleContext<'input,Label_scopedContextExt<'input>>;

#[derive(Clone)]
pub struct Label_scopedContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Label_scopedContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Label_scopedContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_label_scoped(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_label_scoped(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Label_scopedContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_label_scoped(self);
	}
}

impl<'input> CustomRuleContext<'input> for Label_scopedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_label_scoped }
	//fn type_rule_index() -> usize where Self: Sized { RULE_label_scoped }
}
antlr_rust::type_id!{Label_scopedContextExt<'a>}

impl<'input> Label_scopedContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Label_scopedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Label_scopedContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Label_scopedContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Label_scopedContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LABEL_SCOPED_
/// Returns `None` if there is no child corresponding to token LABEL_SCOPED_
fn LABEL_SCOPED_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(LABEL_SCOPED_, 0)
}

}

impl<'input> Label_scopedContextAttrs<'input> for Label_scopedContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn label_scoped(&mut self,)
	-> Result<Rc<Label_scopedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Label_scopedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_label_scoped);
        let mut _localctx: Rc<Label_scopedContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(692);
			recog.base.match_token(LABEL_SCOPED_,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- label ----------------
pub type LabelContextAll<'input> = LabelContext<'input>;


pub type LabelContext<'input> = BaseParserRuleContext<'input,LabelContextExt<'input>>;

#[derive(Clone)]
pub struct LabelContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for LabelContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for LabelContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_label(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_label(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for LabelContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_label(self);
	}
}

impl<'input> CustomRuleContext<'input> for LabelContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_label }
	//fn type_rule_index() -> usize where Self: Sized { RULE_label }
}
antlr_rust::type_id!{LabelContextExt<'a>}

impl<'input> LabelContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LabelContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LabelContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LabelContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<LabelContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LABEL_
/// Returns `None` if there is no child corresponding to token LABEL_
fn LABEL_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(LABEL_, 0)
}
fn schema_native(&self) -> Option<Rc<Schema_nativeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn type_native(&self) -> Option<Rc<Type_nativeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn unreserved(&self) -> Option<Rc<UnreservedContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LabelContextAttrs<'input> for LabelContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn label(&mut self,)
	-> Result<Rc<LabelContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LabelContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_label);
        let mut _localctx: Rc<LabelContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(698);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LABEL_ 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(694);
					recog.base.match_token(LABEL_,&mut recog.err_handler)?;

					}
				}

			 RULE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule schema_native*/
					recog.base.set_state(695);
					recog.schema_native()?;

					}
				}

			 THING | ENTITY | ATTRIBUTE | RELATION | ROLE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule type_native*/
					recog.base.set_state(696);
					recog.type_native()?;

					}
				}

			 GET | OFFSET | LIMIT | SORT | VALUE | CONTAINS | GROUP | COUNT | MAX |
			 MIN | MEAN | MEDIAN | STD | SUM | CLUSTER | PATH | DEGREE | K_CORE |
			 CONNECTED_COMPONENT | FROM | TO | OF | IN | WHERE | MIN_K | K | SIZE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule unreserved*/
					recog.base.set_state(697);
					recog.unreserved()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- labels ----------------
pub type LabelsContextAll<'input> = LabelsContext<'input>;


pub type LabelsContext<'input> = BaseParserRuleContext<'input,LabelsContextExt<'input>>;

#[derive(Clone)]
pub struct LabelsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for LabelsContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for LabelsContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_labels(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_labels(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for LabelsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_labels(self);
	}
}

impl<'input> CustomRuleContext<'input> for LabelsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_labels }
	//fn type_rule_index() -> usize where Self: Sized { RULE_labels }
}
antlr_rust::type_id!{LabelsContextExt<'a>}

impl<'input> LabelsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LabelsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LabelsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LabelsContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<LabelsContextExt<'input>>{

fn label(&self) -> Option<Rc<LabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn label_array(&self) -> Option<Rc<Label_arrayContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LabelsContextAttrs<'input> for LabelsContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn labels(&mut self,)
	-> Result<Rc<LabelsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LabelsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_labels);
        let mut _localctx: Rc<LabelsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(702);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 GET | THING | ENTITY | ATTRIBUTE | RELATION | ROLE | RULE | OFFSET |
			 LIMIT | SORT | VALUE | CONTAINS | GROUP | COUNT | MAX | MIN | MEAN |
			 MEDIAN | STD | SUM | CLUSTER | PATH | DEGREE | K_CORE | CONNECTED_COMPONENT |
			 FROM | TO | OF | IN | WHERE | MIN_K | K | SIZE | LABEL_ 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule label*/
					recog.base.set_state(700);
					recog.label()?;

					}
				}

			 T__7 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule label_array*/
					recog.base.set_state(701);
					recog.label_array()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- label_array ----------------
pub type Label_arrayContextAll<'input> = Label_arrayContext<'input>;


pub type Label_arrayContext<'input> = BaseParserRuleContext<'input,Label_arrayContextExt<'input>>;

#[derive(Clone)]
pub struct Label_arrayContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Label_arrayContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Label_arrayContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_label_array(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_label_array(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Label_arrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_label_array(self);
	}
}

impl<'input> CustomRuleContext<'input> for Label_arrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_label_array }
	//fn type_rule_index() -> usize where Self: Sized { RULE_label_array }
}
antlr_rust::type_id!{Label_arrayContextExt<'a>}

impl<'input> Label_arrayContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Label_arrayContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Label_arrayContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Label_arrayContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Label_arrayContextExt<'input>>{

fn label_all(&self) ->  Vec<Rc<LabelContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn label(&self, i: usize) -> Option<Rc<LabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Label_arrayContextAttrs<'input> for Label_arrayContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn label_array(&mut self,)
	-> Result<Rc<Label_arrayContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Label_arrayContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_label_array);
        let mut _localctx: Rc<Label_arrayContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(704);
			recog.base.match_token(T__7,&mut recog.err_handler)?;

			/*InvokeRule label*/
			recog.base.set_state(705);
			recog.label()?;

			recog.base.set_state(710);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__1 {
				{
				{
				recog.base.set_state(706);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				/*InvokeRule label*/
				recog.base.set_state(707);
				recog.label()?;

				}
				}
				recog.base.set_state(712);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(713);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- schema_native ----------------
pub type Schema_nativeContextAll<'input> = Schema_nativeContext<'input>;


pub type Schema_nativeContext<'input> = BaseParserRuleContext<'input,Schema_nativeContextExt<'input>>;

#[derive(Clone)]
pub struct Schema_nativeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Schema_nativeContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Schema_nativeContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_schema_native(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_schema_native(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Schema_nativeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_schema_native(self);
	}
}

impl<'input> CustomRuleContext<'input> for Schema_nativeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_schema_native }
	//fn type_rule_index() -> usize where Self: Sized { RULE_schema_native }
}
antlr_rust::type_id!{Schema_nativeContextExt<'a>}

impl<'input> Schema_nativeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Schema_nativeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Schema_nativeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Schema_nativeContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Schema_nativeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RULE
/// Returns `None` if there is no child corresponding to token RULE
fn RULE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(RULE, 0)
}

}

impl<'input> Schema_nativeContextAttrs<'input> for Schema_nativeContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn schema_native(&mut self,)
	-> Result<Rc<Schema_nativeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Schema_nativeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_schema_native);
        let mut _localctx: Rc<Schema_nativeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(715);
			recog.base.match_token(RULE,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- type_native ----------------
pub type Type_nativeContextAll<'input> = Type_nativeContext<'input>;


pub type Type_nativeContext<'input> = BaseParserRuleContext<'input,Type_nativeContextExt<'input>>;

#[derive(Clone)]
pub struct Type_nativeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Type_nativeContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Type_nativeContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_type_native(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_type_native(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Type_nativeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_type_native(self);
	}
}

impl<'input> CustomRuleContext<'input> for Type_nativeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type_native }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type_native }
}
antlr_rust::type_id!{Type_nativeContextExt<'a>}

impl<'input> Type_nativeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Type_nativeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type_nativeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Type_nativeContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Type_nativeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token THING
/// Returns `None` if there is no child corresponding to token THING
fn THING(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(THING, 0)
}
/// Retrieves first TerminalNode corresponding to token ENTITY
/// Returns `None` if there is no child corresponding to token ENTITY
fn ENTITY(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(ENTITY, 0)
}
/// Retrieves first TerminalNode corresponding to token ATTRIBUTE
/// Returns `None` if there is no child corresponding to token ATTRIBUTE
fn ATTRIBUTE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(ATTRIBUTE, 0)
}
/// Retrieves first TerminalNode corresponding to token RELATION
/// Returns `None` if there is no child corresponding to token RELATION
fn RELATION(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(RELATION, 0)
}
/// Retrieves first TerminalNode corresponding to token ROLE
/// Returns `None` if there is no child corresponding to token ROLE
fn ROLE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(ROLE, 0)
}

}

impl<'input> Type_nativeContextAttrs<'input> for Type_nativeContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn type_native(&mut self,)
	-> Result<Rc<Type_nativeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type_nativeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_type_native);
        let mut _localctx: Rc<Type_nativeContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(717);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << THING) | (1usize << ENTITY) | (1usize << ATTRIBUTE) | (1usize << RELATION) | (1usize << ROLE))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- value_type ----------------
pub type Value_typeContextAll<'input> = Value_typeContext<'input>;


pub type Value_typeContext<'input> = BaseParserRuleContext<'input,Value_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Value_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for Value_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for Value_typeContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_value_type(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_value_type(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for Value_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_value_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Value_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value_type }
}
antlr_rust::type_id!{Value_typeContextExt<'a>}

impl<'input> Value_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Value_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Value_typeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Value_typeContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<Value_typeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LONG
/// Returns `None` if there is no child corresponding to token LONG
fn LONG(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(LONG, 0)
}
/// Retrieves first TerminalNode corresponding to token DOUBLE
/// Returns `None` if there is no child corresponding to token DOUBLE
fn DOUBLE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(DOUBLE, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOLEAN
/// Returns `None` if there is no child corresponding to token BOOLEAN
fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(BOOLEAN, 0)
}
/// Retrieves first TerminalNode corresponding to token DATETIME
/// Returns `None` if there is no child corresponding to token DATETIME
fn DATETIME(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(DATETIME, 0)
}

}

impl<'input> Value_typeContextAttrs<'input> for Value_typeContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn value_type(&mut self,)
	-> Result<Rc<Value_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Value_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_value_type);
        let mut _localctx: Rc<Value_typeContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(719);
			_la = recog.base.input.la(1);
			if { !(((((_la - 82)) & !0x3f) == 0 && ((1usize << (_la - 82)) & ((1usize << (LONG - 82)) | (1usize << (DOUBLE - 82)) | (1usize << (STRING - 82)) | (1usize << (BOOLEAN - 82)) | (1usize << (DATETIME - 82)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- value ----------------
pub type ValueContextAll<'input> = ValueContext<'input>;


pub type ValueContext<'input> = BaseParserRuleContext<'input,ValueContextExt<'input>>;

#[derive(Clone)]
pub struct ValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for ValueContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for ValueContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_value(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_value(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for ValueContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_value(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}
antlr_rust::type_id!{ValueContextExt<'a>}

impl<'input> ValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValueContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<ValueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING_
/// Returns `None` if there is no child corresponding to token STRING_
fn STRING_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(STRING_, 0)
}
/// Retrieves first TerminalNode corresponding to token LONG_
/// Returns `None` if there is no child corresponding to token LONG_
fn LONG_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(LONG_, 0)
}
/// Retrieves first TerminalNode corresponding to token DOUBLE_
/// Returns `None` if there is no child corresponding to token DOUBLE_
fn DOUBLE_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(DOUBLE_, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOLEAN_
/// Returns `None` if there is no child corresponding to token BOOLEAN_
fn BOOLEAN_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(BOOLEAN_, 0)
}
/// Retrieves first TerminalNode corresponding to token DATE_
/// Returns `None` if there is no child corresponding to token DATE_
fn DATE_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(DATE_, 0)
}
/// Retrieves first TerminalNode corresponding to token DATETIME_
/// Returns `None` if there is no child corresponding to token DATETIME_
fn DATETIME_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(DATETIME_, 0)
}

}

impl<'input> ValueContextAttrs<'input> for ValueContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn value(&mut self,)
	-> Result<Rc<ValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 166, RULE_value);
        let mut _localctx: Rc<ValueContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(721);
			_la = recog.base.input.la(1);
			if { !(((((_la - 87)) & !0x3f) == 0 && ((1usize << (_la - 87)) & ((1usize << (BOOLEAN_ - 87)) | (1usize << (STRING_ - 87)) | (1usize << (LONG_ - 87)) | (1usize << (DOUBLE_ - 87)) | (1usize << (DATE_ - 87)) | (1usize << (DATETIME_ - 87)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- regex ----------------
pub type RegexContextAll<'input> = RegexContext<'input>;


pub type RegexContext<'input> = BaseParserRuleContext<'input,RegexContextExt<'input>>;

#[derive(Clone)]
pub struct RegexContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for RegexContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for RegexContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_regex(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_regex(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for RegexContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_regex(self);
	}
}

impl<'input> CustomRuleContext<'input> for RegexContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_regex }
	//fn type_rule_index() -> usize where Self: Sized { RULE_regex }
}
antlr_rust::type_id!{RegexContextExt<'a>}

impl<'input> RegexContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RegexContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RegexContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RegexContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<RegexContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING_
/// Returns `None` if there is no child corresponding to token STRING_
fn STRING_(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(STRING_, 0)
}

}

impl<'input> RegexContextAttrs<'input> for RegexContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn regex(&mut self,)
	-> Result<Rc<RegexContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RegexContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 168, RULE_regex);
        let mut _localctx: Rc<RegexContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(723);
			recog.base.match_token(STRING_,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
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
//------------------- unreserved ----------------
pub type UnreservedContextAll<'input> = UnreservedContext<'input>;


pub type UnreservedContext<'input> = BaseParserRuleContext<'input,UnreservedContextExt<'input>>;

#[derive(Clone)]
pub struct UnreservedContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TypeQLRustParserContext<'input> for UnreservedContext<'input>{}

impl<'input,'a> Listenable<dyn TypeQLRustListener<'input> + 'a> for UnreservedContext<'input>{
	fn enter(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_unreserved(self);
	}
	fn exit(&self,listener: &mut (dyn TypeQLRustListener<'input> + 'a)) {
		listener.exit_unreserved(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn TypeQLRustVisitor<'input> + 'a> for UnreservedContext<'input>{
	fn accept(&self,visitor: &mut (dyn TypeQLRustVisitor<'input> + 'a)) {
		visitor.visit_unreserved(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnreservedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TypeQLRustParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unreserved }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unreserved }
}
antlr_rust::type_id!{UnreservedContextExt<'a>}

impl<'input> UnreservedContextExt<'input>{
	fn new(parent: Option<Rc<dyn TypeQLRustParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnreservedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnreservedContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UnreservedContextAttrs<'input>: TypeQLRustParserContext<'input> + BorrowMut<UnreservedContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VALUE
/// Returns `None` if there is no child corresponding to token VALUE
fn VALUE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(VALUE, 0)
}
/// Retrieves first TerminalNode corresponding to token MIN
/// Returns `None` if there is no child corresponding to token MIN
fn MIN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MIN, 0)
}
/// Retrieves first TerminalNode corresponding to token MAX
/// Returns `None` if there is no child corresponding to token MAX
fn MAX(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MAX, 0)
}
/// Retrieves first TerminalNode corresponding to token MEDIAN
/// Returns `None` if there is no child corresponding to token MEDIAN
fn MEDIAN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MEDIAN, 0)
}
/// Retrieves first TerminalNode corresponding to token MEAN
/// Returns `None` if there is no child corresponding to token MEAN
fn MEAN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MEAN, 0)
}
/// Retrieves first TerminalNode corresponding to token STD
/// Returns `None` if there is no child corresponding to token STD
fn STD(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(STD, 0)
}
/// Retrieves first TerminalNode corresponding to token SUM
/// Returns `None` if there is no child corresponding to token SUM
fn SUM(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(SUM, 0)
}
/// Retrieves first TerminalNode corresponding to token COUNT
/// Returns `None` if there is no child corresponding to token COUNT
fn COUNT(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(COUNT, 0)
}
/// Retrieves first TerminalNode corresponding to token GET
/// Returns `None` if there is no child corresponding to token GET
fn GET(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(GET, 0)
}
/// Retrieves first TerminalNode corresponding to token SORT
/// Returns `None` if there is no child corresponding to token SORT
fn SORT(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(SORT, 0)
}
/// Retrieves first TerminalNode corresponding to token LIMIT
/// Returns `None` if there is no child corresponding to token LIMIT
fn LIMIT(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(LIMIT, 0)
}
/// Retrieves first TerminalNode corresponding to token OFFSET
/// Returns `None` if there is no child corresponding to token OFFSET
fn OFFSET(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(OFFSET, 0)
}
/// Retrieves first TerminalNode corresponding to token GROUP
/// Returns `None` if there is no child corresponding to token GROUP
fn GROUP(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(GROUP, 0)
}
/// Retrieves first TerminalNode corresponding to token PATH
/// Returns `None` if there is no child corresponding to token PATH
fn PATH(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(PATH, 0)
}
/// Retrieves first TerminalNode corresponding to token CLUSTER
/// Returns `None` if there is no child corresponding to token CLUSTER
fn CLUSTER(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(CLUSTER, 0)
}
/// Retrieves first TerminalNode corresponding to token FROM
/// Returns `None` if there is no child corresponding to token FROM
fn FROM(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(FROM, 0)
}
/// Retrieves first TerminalNode corresponding to token TO
/// Returns `None` if there is no child corresponding to token TO
fn TO(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(TO, 0)
}
/// Retrieves first TerminalNode corresponding to token OF
/// Returns `None` if there is no child corresponding to token OF
fn OF(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(OF, 0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}
/// Retrieves first TerminalNode corresponding to token DEGREE
/// Returns `None` if there is no child corresponding to token DEGREE
fn DEGREE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(DEGREE, 0)
}
/// Retrieves first TerminalNode corresponding to token K_CORE
/// Returns `None` if there is no child corresponding to token K_CORE
fn K_CORE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(K_CORE, 0)
}
/// Retrieves first TerminalNode corresponding to token CONNECTED_COMPONENT
/// Returns `None` if there is no child corresponding to token CONNECTED_COMPONENT
fn CONNECTED_COMPONENT(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(CONNECTED_COMPONENT, 0)
}
/// Retrieves first TerminalNode corresponding to token MIN_K
/// Returns `None` if there is no child corresponding to token MIN_K
fn MIN_K(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(MIN_K, 0)
}
/// Retrieves first TerminalNode corresponding to token K
/// Returns `None` if there is no child corresponding to token K
fn K(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(K, 0)
}
/// Retrieves first TerminalNode corresponding to token CONTAINS
/// Returns `None` if there is no child corresponding to token CONTAINS
fn CONTAINS(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(CONTAINS, 0)
}
/// Retrieves first TerminalNode corresponding to token SIZE
/// Returns `None` if there is no child corresponding to token SIZE
fn SIZE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(SIZE, 0)
}
/// Retrieves first TerminalNode corresponding to token WHERE
/// Returns `None` if there is no child corresponding to token WHERE
fn WHERE(&self) -> Option<Rc<TerminalNode<'input,TypeQLRustParserContextType>>> where Self:Sized{
	self.get_token(WHERE, 0)
}

}

impl<'input> UnreservedContextAttrs<'input> for UnreservedContext<'input>{}

impl<'input, I, H> TypeQLRustParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unreserved(&mut self,)
	-> Result<Rc<UnreservedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnreservedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 170, RULE_unreserved);
        let mut _localctx: Rc<UnreservedContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(725);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << GET) | (1usize << OFFSET) | (1usize << LIMIT) | (1usize << SORT) | (1usize << VALUE) | (1usize << CONTAINS) | (1usize << GROUP) | (1usize << COUNT) | (1usize << MAX) | (1usize << MIN) | (1usize << MEAN))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (MEDIAN - 64)) | (1usize << (STD - 64)) | (1usize << (SUM - 64)) | (1usize << (CLUSTER - 64)) | (1usize << (PATH - 64)) | (1usize << (DEGREE - 64)) | (1usize << (K_CORE - 64)) | (1usize << (CONNECTED_COMPONENT - 64)) | (1usize << (FROM - 64)) | (1usize << (TO - 64)) | (1usize << (OF - 64)) | (1usize << (IN - 64)) | (1usize << (WHERE - 64)) | (1usize << (MIN_K - 64)) | (1usize << (K - 64)) | (1usize << (SIZE - 64)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
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
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x69\u{2da}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x04\x46\x09\x46\x04\x47\x09\x47\x04\x48\x09\x48\x04\
	\x49\x09\x49\x04\x4a\x09\x4a\x04\x4b\x09\x4b\x04\x4c\x09\x4c\x04\x4d\x09\
	\x4d\x04\x4e\x09\x4e\x04\x4f\x09\x4f\x04\x50\x09\x50\x04\x51\x09\x51\x04\
	\x52\x09\x52\x04\x53\x09\x53\x04\x54\x09\x54\x04\x55\x09\x55\x04\x56\x09\
	\x56\x04\x57\x09\x57\x03\x02\x03\x02\x03\x02\x03\x03\x06\x03\u{b3}\x0a\x03\
	\x0d\x03\x0e\x03\u{b4}\x03\x03\x03\x03\x03\x04\x03\x04\x03\x04\x03\x05\x03\
	\x05\x03\x05\x03\x06\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\x08\x03\
	\x08\x03\x08\x03\x09\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x05\x0a\u{d4}\x0a\x0a\x03\x0b\x03\x0b\
	\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\
	\x03\x0d\x03\x0d\x05\x0d\u{e3}\x0a\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\
	\x0e\x03\x0e\x05\x0e\u{eb}\x0a\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x10\
	\x03\x10\x03\x10\x03\x11\x03\x11\x03\x11\x03\x12\x03\x12\x03\x12\x03\x13\
	\x03\x13\x03\x13\x03\x13\x03\x14\x03\x14\x03\x14\x05\x14\u{101}\x0a\x14\
	\x03\x14\x03\x14\x03\x14\x05\x14\u{106}\x0a\x14\x03\x14\x03\x14\x03\x14\
	\x05\x14\u{10b}\x0a\x14\x03\x14\x03\x14\x03\x14\x05\x14\u{110}\x0a\x14\x03\
	\x15\x03\x15\x03\x15\x03\x15\x07\x15\u{116}\x0a\x15\x0c\x15\x0e\x15\u{119}\
	\x0b\x15\x03\x16\x03\x16\x03\x16\x05\x16\u{11e}\x0a\x16\x03\x17\x03\x17\
	\x03\x17\x03\x18\x03\x18\x03\x18\x03\x19\x03\x19\x05\x19\u{128}\x0a\x19\
	\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1c\
	\x03\x1c\x03\x1c\x06\x1c\u{135}\x0a\x1c\x0d\x1c\x0e\x1c\u{136}\x03\x1d\x03\
	\x1d\x05\x1d\u{13b}\x0a\x1d\x03\x1e\x03\x1e\x03\x1e\x06\x1e\u{140}\x0a\x1e\
	\x0d\x1e\x0e\x1e\u{141}\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x05\x1f\u{148}\x0a\
	\x1f\x03\x20\x03\x20\x03\x20\x03\x20\x03\x21\x03\x21\x03\x21\x03\x21\x03\
	\x21\x03\x21\x03\x21\x03\x21\x06\x21\u{156}\x0a\x21\x0d\x21\x0e\x21\u{157}\
	\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x23\x03\x23\x03\x23\x05\x23\
	\u{162}\x0a\x23\x03\x24\x03\x24\x03\x24\x03\x24\x03\x25\x03\x25\x03\x25\
	\x03\x25\x07\x25\u{16c}\x0a\x25\x0c\x25\x0e\x25\u{16f}\x0b\x25\x03\x26\x03\
	\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x05\x26\u{178}\x0a\x26\x03\
	\x26\x05\x26\u{17b}\x0a\x26\x03\x26\x03\x26\x03\x26\x03\x26\x05\x26\u{181}\
	\x0a\x26\x03\x26\x03\x26\x03\x26\x03\x26\x05\x26\u{187}\x0a\x26\x03\x26\
	\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\
	\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x05\x26\u{199}\x0a\x26\
	\x03\x27\x03\x27\x03\x27\x06\x27\u{19e}\x0a\x27\x0d\x27\x0e\x27\u{19f}\x03\
	\x28\x03\x28\x03\x28\x05\x28\u{1a5}\x0a\x28\x03\x29\x03\x29\x03\x29\x03\
	\x29\x03\x29\x05\x29\u{1ac}\x0a\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\
	\x29\x05\x29\u{1b3}\x0a\x29\x03\x29\x03\x29\x05\x29\u{1b7}\x0a\x29\x03\x2a\
	\x05\x2a\u{1ba}\x0a\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x05\x2a\
	\u{1c1}\x0a\x2a\x03\x2a\x05\x2a\u{1c4}\x0a\x2a\x03\x2a\x03\x2a\x05\x2a\u{1c8}\
	\x0a\x2a\x05\x2a\u{1ca}\x0a\x2a\x03\x2b\x05\x2b\u{1cd}\x0a\x2b\x03\x2b\x03\
	\x2b\x03\x2b\x03\x2b\x03\x2b\x05\x2b\u{1d4}\x0a\x2b\x03\x2b\x05\x2b\u{1d7}\
	\x0a\x2b\x03\x2b\x03\x2b\x05\x2b\u{1db}\x0a\x2b\x05\x2b\u{1dd}\x0a\x2b\x03\
	\x2c\x03\x2c\x03\x2c\x03\x2c\x07\x2c\u{1e3}\x0a\x2c\x0c\x2c\x0e\x2c\u{1e6}\
	\x0b\x2c\x03\x2c\x03\x2c\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x05\x2d\
	\u{1ef}\x0a\x2d\x03\x2e\x03\x2e\x03\x2f\x03\x2f\x03\x2f\x07\x2f\u{1f6}\x0a\
	\x2f\x0c\x2f\x0e\x2f\u{1f9}\x0b\x2f\x03\x30\x03\x30\x03\x30\x03\x30\x05\
	\x30\u{1ff}\x0a\x30\x03\x30\x03\x30\x05\x30\u{203}\x0a\x30\x03\x31\x03\x31\
	\x03\x31\x03\x31\x03\x31\x03\x31\x03\x31\x05\x31\u{20c}\x0a\x31\x03\x32\
	\x03\x32\x03\x33\x03\x33\x03\x34\x03\x34\x05\x34\u{214}\x0a\x34\x03\x35\
	\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\
	\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x05\x35\u{225}\x0a\x35\x03\x36\
	\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\
	\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x05\x36\u{236}\x0a\x36\x03\x37\
	\x03\x37\x03\x38\x03\x38\x05\x38\u{23c}\x0a\x38\x03\x39\x03\x39\x03\x39\
	\x03\x39\x07\x39\u{242}\x0a\x39\x0c\x39\x0e\x39\u{245}\x0b\x39\x03\x3a\x03\
	\x3a\x03\x3a\x03\x3a\x07\x3a\u{24b}\x0a\x3a\x0c\x3a\x0e\x3a\u{24e}\x0b\x3a\
	\x03\x3b\x03\x3b\x03\x3b\x03\x3b\x07\x3b\u{254}\x0a\x3b\x0c\x3b\x0e\x3b\
	\u{257}\x0b\x3b\x03\x3c\x03\x3c\x03\x3c\x03\x3c\x07\x3c\u{25d}\x0a\x3c\x0c\
	\x3c\x0e\x3c\u{260}\x0b\x3c\x03\x3d\x03\x3d\x03\x3e\x03\x3e\x05\x3e\u{266}\
	\x0a\x3e\x03\x3f\x03\x3f\x03\x3f\x05\x3f\u{26b}\x0a\x3f\x03\x40\x03\x40\
	\x05\x40\u{26f}\x0a\x40\x03\x41\x03\x41\x05\x41\u{273}\x0a\x41\x03\x42\x03\
	\x42\x03\x42\x03\x42\x05\x42\u{279}\x0a\x42\x03\x43\x03\x43\x03\x43\x03\
	\x44\x03\x44\x03\x44\x03\x45\x03\x45\x03\x45\x03\x45\x05\x45\u{285}\x0a\
	\x45\x03\x46\x03\x46\x03\x47\x03\x47\x05\x47\u{28b}\x0a\x47\x03\x48\x03\
	\x48\x03\x48\x03\x48\x07\x48\u{291}\x0a\x48\x0c\x48\x0e\x48\u{294}\x0b\x48\
	\x03\x48\x03\x48\x03\x49\x03\x49\x03\x49\x03\x49\x03\x49\x03\x49\x03\x49\
	\x03\x49\x03\x49\x03\x49\x03\x49\x03\x49\x05\x49\u{2a4}\x0a\x49\x03\x4a\
	\x03\x4a\x03\x4a\x05\x4a\u{2a9}\x0a\x4a\x03\x4b\x03\x4b\x05\x4b\u{2ad}\x0a\
	\x4b\x03\x4c\x03\x4c\x05\x4c\u{2b1}\x0a\x4c\x03\x4d\x03\x4d\x05\x4d\u{2b5}\
	\x0a\x4d\x03\x4e\x03\x4e\x03\x4f\x03\x4f\x03\x4f\x03\x4f\x05\x4f\u{2bd}\
	\x0a\x4f\x03\x50\x03\x50\x05\x50\u{2c1}\x0a\x50\x03\x51\x03\x51\x03\x51\
	\x03\x51\x07\x51\u{2c7}\x0a\x51\x0c\x51\x0e\x51\u{2ca}\x0b\x51\x03\x51\x03\
	\x51\x03\x52\x03\x52\x03\x53\x03\x53\x03\x54\x03\x54\x03\x55\x03\x55\x03\
	\x56\x03\x56\x03\x57\x03\x57\x03\x57\x02\x02\x58\x02\x04\x06\x08\x0a\x0c\
	\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\
	\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x50\x52\x54\
	\x56\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\x72\x74\x76\x78\
	\x7a\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\u{8a}\u{8c}\u{8e}\u{90}\u{92}\
	\u{94}\u{96}\u{98}\u{9a}\u{9c}\u{9e}\u{a0}\u{a2}\u{a4}\u{a6}\u{a8}\u{aa}\
	\u{ac}\x02\x0b\x03\x02\x3e\x44\x03\x02\x35\x3a\x03\x02\x3b\x3c\x03\x02\x3f\
	\x44\x03\x02\x48\x4a\x03\x02\x13\x17\x03\x02\x54\x58\x04\x02\x59\x59\x5c\
	\x60\x08\x02\x0d\x0d\x19\x1b\x31\x31\x3c\x45\x47\x4e\x50\x53\x02\u{2e5}\
	\x02\u{ae}\x03\x02\x02\x02\x04\u{b2}\x03\x02\x02\x02\x06\u{b8}\x03\x02\x02\
	\x02\x08\u{bb}\x03\x02\x02\x02\x0a\u{be}\x03\x02\x02\x02\x0c\u{c1}\x03\x02\
	\x02\x02\x0e\u{c4}\x03\x02\x02\x02\x10\u{c7}\x03\x02\x02\x02\x12\u{d3}\x03\
	\x02\x02\x02\x14\u{d5}\x03\x02\x02\x02\x16\u{d8}\x03\x02\x02\x02\x18\u{e2}\
	\x03\x02\x02\x02\x1a\u{e4}\x03\x02\x02\x02\x1c\u{ec}\x03\x02\x02\x02\x1e\
	\u{f0}\x03\x02\x02\x02\x20\u{f3}\x03\x02\x02\x02\x22\u{f6}\x03\x02\x02\x02\
	\x24\u{f9}\x03\x02\x02\x02\x26\u{100}\x03\x02\x02\x02\x28\u{111}\x03\x02\
	\x02\x02\x2a\u{11a}\x03\x02\x02\x02\x2c\u{11f}\x03\x02\x02\x02\x2e\u{122}\
	\x03\x02\x02\x02\x30\u{125}\x03\x02\x02\x02\x32\u{12b}\x03\x02\x02\x02\x34\
	\u{12d}\x03\x02\x02\x02\x36\u{134}\x03\x02\x02\x02\x38\u{13a}\x03\x02\x02\
	\x02\x3a\u{13f}\x03\x02\x02\x02\x3c\u{147}\x03\x02\x02\x02\x3e\u{149}\x03\
	\x02\x02\x02\x40\u{14d}\x03\x02\x02\x02\x42\u{159}\x03\x02\x02\x02\x44\u{161}\
	\x03\x02\x02\x02\x46\u{163}\x03\x02\x02\x02\x48\u{167}\x03\x02\x02\x02\x4a\
	\u{198}\x03\x02\x02\x02\x4c\u{19d}\x03\x02\x02\x02\x4e\u{1a4}\x03\x02\x02\
	\x02\x50\u{1b6}\x03\x02\x02\x02\x52\u{1c9}\x03\x02\x02\x02\x54\u{1dc}\x03\
	\x02\x02\x02\x56\u{1de}\x03\x02\x02\x02\x58\u{1ee}\x03\x02\x02\x02\x5a\u{1f0}\
	\x03\x02\x02\x02\x5c\u{1f2}\x03\x02\x02\x02\x5e\u{202}\x03\x02\x02\x02\x60\
	\u{20b}\x03\x02\x02\x02\x62\u{20d}\x03\x02\x02\x02\x64\u{20f}\x03\x02\x02\
	\x02\x66\u{213}\x03\x02\x02\x02\x68\u{224}\x03\x02\x02\x02\x6a\u{235}\x03\
	\x02\x02\x02\x6c\u{237}\x03\x02\x02\x02\x6e\u{239}\x03\x02\x02\x02\x70\u{23d}\
	\x03\x02\x02\x02\x72\u{246}\x03\x02\x02\x02\x74\u{24f}\x03\x02\x02\x02\x76\
	\u{258}\x03\x02\x02\x02\x78\u{261}\x03\x02\x02\x02\x7a\u{265}\x03\x02\x02\
	\x02\x7c\u{26a}\x03\x02\x02\x02\x7e\u{26e}\x03\x02\x02\x02\u{80}\u{272}\
	\x03\x02\x02\x02\u{82}\u{278}\x03\x02\x02\x02\u{84}\u{27a}\x03\x02\x02\x02\
	\u{86}\u{27d}\x03\x02\x02\x02\u{88}\u{284}\x03\x02\x02\x02\u{8a}\u{286}\
	\x03\x02\x02\x02\u{8c}\u{28a}\x03\x02\x02\x02\u{8e}\u{28c}\x03\x02\x02\x02\
	\u{90}\u{2a3}\x03\x02\x02\x02\u{92}\u{2a8}\x03\x02\x02\x02\u{94}\u{2ac}\
	\x03\x02\x02\x02\u{96}\u{2b0}\x03\x02\x02\x02\u{98}\u{2b4}\x03\x02\x02\x02\
	\u{9a}\u{2b6}\x03\x02\x02\x02\u{9c}\u{2bc}\x03\x02\x02\x02\u{9e}\u{2c0}\
	\x03\x02\x02\x02\u{a0}\u{2c2}\x03\x02\x02\x02\u{a2}\u{2cd}\x03\x02\x02\x02\
	\u{a4}\u{2cf}\x03\x02\x02\x02\u{a6}\u{2d1}\x03\x02\x02\x02\u{a8}\u{2d3}\
	\x03\x02\x02\x02\u{aa}\u{2d5}\x03\x02\x02\x02\u{ac}\u{2d7}\x03\x02\x02\x02\
	\u{ae}\u{af}\x05\x12\x0a\x02\u{af}\u{b0}\x07\x02\x02\x03\u{b0}\x03\x03\x02\
	\x02\x02\u{b1}\u{b3}\x05\x12\x0a\x02\u{b2}\u{b1}\x03\x02\x02\x02\u{b3}\u{b4}\
	\x03\x02\x02\x02\u{b4}\u{b2}\x03\x02\x02\x02\u{b4}\u{b5}\x03\x02\x02\x02\
	\u{b5}\u{b6}\x03\x02\x02\x02\u{b6}\u{b7}\x07\x02\x02\x03\u{b7}\x05\x03\x02\
	\x02\x02\u{b8}\u{b9}\x05\x3c\x1f\x02\u{b9}\u{ba}\x07\x02\x02\x03\u{ba}\x07\
	\x03\x02\x02\x02\u{bb}\u{bc}\x05\x3a\x1e\x02\u{bc}\u{bd}\x07\x02\x02\x03\
	\u{bd}\x09\x03\x02\x02\x02\u{be}\u{bf}\x05\x36\x1c\x02\u{bf}\u{c0}\x07\x02\
	\x02\x03\u{c0}\x0b\x03\x02\x02\x02\u{c1}\u{c2}\x05\x44\x23\x02\u{c2}\u{c3}\
	\x07\x02\x02\x03\u{c3}\x0d\x03\x02\x02\x02\u{c4}\u{c5}\x05\u{9c}\x4f\x02\
	\u{c5}\u{c6}\x07\x02\x02\x03\u{c6}\x0f\x03\x02\x02\x02\u{c7}\u{c8}\x05\x68\
	\x35\x02\u{c8}\u{c9}\x07\x02\x02\x03\u{c9}\x11\x03\x02\x02\x02\u{ca}\u{d4}\
	\x05\x14\x0b\x02\u{cb}\u{d4}\x05\x16\x0c\x02\u{cc}\u{d4}\x05\x18\x0d\x02\
	\u{cd}\u{d4}\x05\x1a\x0e\x02\u{ce}\u{d4}\x05\x1c\x0f\x02\u{cf}\u{d4}\x05\
	\x20\x11\x02\u{d0}\u{d4}\x05\x22\x12\x02\u{d1}\u{d4}\x05\x24\x13\x02\u{d2}\
	\u{d4}\x05\x1e\x10\x02\u{d3}\u{ca}\x03\x02\x02\x02\u{d3}\u{cb}\x03\x02\x02\
	\x02\u{d3}\u{cc}\x03\x02\x02\x02\u{d3}\u{cd}\x03\x02\x02\x02\u{d3}\u{ce}\
	\x03\x02\x02\x02\u{d3}\u{cf}\x03\x02\x02\x02\u{d3}\u{d0}\x03\x02\x02\x02\
	\u{d3}\u{d1}\x03\x02\x02\x02\u{d3}\u{d2}\x03\x02\x02\x02\u{d4}\x13\x03\x02\
	\x02\x02\u{d5}\u{d6}\x07\x0e\x02\x02\u{d6}\u{d7}\x05\x36\x1c\x02\u{d7}\x15\
	\x03\x02\x02\x02\u{d8}\u{d9}\x07\x0f\x02\x02\u{d9}\u{da}\x05\x36\x1c\x02\
	\u{da}\x17\x03\x02\x02\x02\u{db}\u{dc}\x07\x0c\x02\x02\u{dc}\u{dd}\x05\x3a\
	\x1e\x02\u{dd}\u{de}\x07\x10\x02\x02\u{de}\u{df}\x05\x4c\x27\x02\u{df}\u{e3}\
	\x03\x02\x02\x02\u{e0}\u{e1}\x07\x10\x02\x02\u{e1}\u{e3}\x05\x4c\x27\x02\
	\u{e2}\u{db}\x03\x02\x02\x02\u{e2}\u{e0}\x03\x02\x02\x02\u{e3}\x19\x03\x02\
	\x02\x02\u{e4}\u{e5}\x07\x0c\x02\x02\u{e5}\u{e6}\x05\x3a\x1e\x02\u{e6}\u{e7}\
	\x07\x11\x02\x02\u{e7}\u{ea}\x05\x4c\x27\x02\u{e8}\u{e9}\x07\x10\x02\x02\
	\u{e9}\u{eb}\x05\x4c\x27\x02\u{ea}\u{e8}\x03\x02\x02\x02\u{ea}\u{eb}\x03\
	\x02\x02\x02\u{eb}\x1b\x03\x02\x02\x02\u{ec}\u{ed}\x07\x0c\x02\x02\u{ed}\
	\u{ee}\x05\x3a\x1e\x02\u{ee}\u{ef}\x05\x26\x14\x02\u{ef}\x1d\x03\x02\x02\
	\x02\u{f0}\u{f1}\x07\x12\x02\x02\u{f1}\u{f2}\x05\x6a\x36\x02\u{f2}\x1f\x03\
	\x02\x02\x02\u{f3}\u{f4}\x05\x1c\x0f\x02\u{f4}\u{f5}\x05\x30\x19\x02\u{f5}\
	\x21\x03\x02\x02\x02\u{f6}\u{f7}\x05\x1c\x0f\x02\u{f7}\u{f8}\x05\x34\x1b\
	\x02\u{f8}\x23\x03\x02\x02\x02\u{f9}\u{fa}\x05\x1c\x0f\x02\u{fa}\u{fb}\x05\
	\x34\x1b\x02\u{fb}\u{fc}\x05\x30\x19\x02\u{fc}\x25\x03\x02\x02\x02\u{fd}\
	\u{fe}\x05\x28\x15\x02\u{fe}\u{ff}\x07\x03\x02\x02\u{ff}\u{101}\x03\x02\
	\x02\x02\u{100}\u{fd}\x03\x02\x02\x02\u{100}\u{101}\x03\x02\x02\x02\u{101}\
	\u{105}\x03\x02\x02\x02\u{102}\u{103}\x05\x2a\x16\x02\u{103}\u{104}\x07\
	\x03\x02\x02\u{104}\u{106}\x03\x02\x02\x02\u{105}\u{102}\x03\x02\x02\x02\
	\u{105}\u{106}\x03\x02\x02\x02\u{106}\u{10a}\x03\x02\x02\x02\u{107}\u{108}\
	\x05\x2c\x17\x02\u{108}\u{109}\x07\x03\x02\x02\u{109}\u{10b}\x03\x02\x02\
	\x02\u{10a}\u{107}\x03\x02\x02\x02\u{10a}\u{10b}\x03\x02\x02\x02\u{10b}\
	\u{10f}\x03\x02\x02\x02\u{10c}\u{10d}\x05\x2e\x18\x02\u{10d}\u{10e}\x07\
	\x03\x02\x02\u{10e}\u{110}\x03\x02\x02\x02\u{10f}\u{10c}\x03\x02\x02\x02\
	\u{10f}\u{110}\x03\x02\x02\x02\u{110}\x27\x03\x02\x02\x02\u{111}\u{112}\
	\x07\x0d\x02\x02\u{112}\u{117}\x07\x61\x02\x02\u{113}\u{114}\x07\x04\x02\
	\x02\u{114}\u{116}\x07\x61\x02\x02\u{115}\u{113}\x03\x02\x02\x02\u{116}\
	\u{119}\x03\x02\x02\x02\u{117}\u{115}\x03\x02\x02\x02\u{117}\u{118}\x03\
	\x02\x02\x02\u{118}\x29\x03\x02\x02\x02\u{119}\u{117}\x03\x02\x02\x02\u{11a}\
	\u{11b}\x07\x1b\x02\x02\u{11b}\u{11d}\x07\x61\x02\x02\u{11c}\u{11e}\x07\
	\x1c\x02\x02\u{11d}\u{11c}\x03\x02\x02\x02\u{11d}\u{11e}\x03\x02\x02\x02\
	\u{11e}\x2b\x03\x02\x02\x02\u{11f}\u{120}\x07\x19\x02\x02\u{120}\u{121}\
	\x07\x5d\x02\x02\u{121}\x2d\x03\x02\x02\x02\u{122}\u{123}\x07\x1a\x02\x02\
	\u{123}\u{124}\x07\x5d\x02\x02\u{124}\x2f\x03\x02\x02\x02\u{125}\u{127}\
	\x05\x32\x1a\x02\u{126}\u{128}\x07\x61\x02\x02\u{127}\u{126}\x03\x02\x02\
	\x02\u{127}\u{128}\x03\x02\x02\x02\u{128}\u{129}\x03\x02\x02\x02\u{129}\
	\u{12a}\x07\x03\x02\x02\u{12a}\x31\x03\x02\x02\x02\u{12b}\u{12c}\x09\x02\
	\x02\x02\u{12c}\x33\x03\x02\x02\x02\u{12d}\u{12e}\x07\x3d\x02\x02\u{12e}\
	\u{12f}\x07\x61\x02\x02\u{12f}\u{130}\x07\x03\x02\x02\u{130}\x35\x03\x02\
	\x02\x02\u{131}\u{132}\x05\x38\x1d\x02\u{132}\u{133}\x07\x03\x02\x02\u{133}\
	\u{135}\x03\x02\x02\x02\u{134}\u{131}\x03\x02\x02\x02\u{135}\u{136}\x03\
	\x02\x02\x02\u{136}\u{134}\x03\x02\x02\x02\u{136}\u{137}\x03\x02\x02\x02\
	\u{137}\x37\x03\x02\x02\x02\u{138}\u{13b}\x05\x48\x25\x02\u{139}\u{13b}\
	\x05\x68\x35\x02\u{13a}\u{138}\x03\x02\x02\x02\u{13a}\u{139}\x03\x02\x02\
	\x02\u{13b}\x39\x03\x02\x02\x02\u{13c}\u{13d}\x05\x3c\x1f\x02\u{13d}\u{13e}\
	\x07\x03\x02\x02\u{13e}\u{140}\x03\x02\x02\x02\u{13f}\u{13c}\x03\x02\x02\
	\x02\u{140}\u{141}\x03\x02\x02\x02\u{141}\u{13f}\x03\x02\x02\x02\u{141}\
	\u{142}\x03\x02\x02\x02\u{142}\x3b\x03\x02\x02\x02\u{143}\u{148}\x05\x44\
	\x23\x02\u{144}\u{148}\x05\x3e\x20\x02\u{145}\u{148}\x05\x40\x21\x02\u{146}\
	\u{148}\x05\x42\x22\x02\u{147}\u{143}\x03\x02\x02\x02\u{147}\u{144}\x03\
	\x02\x02\x02\u{147}\u{145}\x03\x02\x02\x02\u{147}\u{146}\x03\x02\x02\x02\
	\u{148}\x3d\x03\x02\x02\x02\u{149}\u{14a}\x07\x05\x02\x02\u{14a}\u{14b}\
	\x05\x3a\x1e\x02\u{14b}\u{14c}\x07\x06\x02\x02\u{14c}\x3f\x03\x02\x02\x02\
	\u{14d}\u{14e}\x07\x05\x02\x02\u{14e}\u{14f}\x05\x3a\x1e\x02\u{14f}\u{155}\
	\x07\x06\x02\x02\u{150}\u{151}\x07\x33\x02\x02\u{151}\u{152}\x07\x05\x02\
	\x02\u{152}\u{153}\x05\x3a\x1e\x02\u{153}\u{154}\x07\x06\x02\x02\u{154}\
	\u{156}\x03\x02\x02\x02\u{155}\u{150}\x03\x02\x02\x02\u{156}\u{157}\x03\
	\x02\x02\x02\u{157}\u{155}\x03\x02\x02\x02\u{157}\u{158}\x03\x02\x02\x02\
	\u{158}\x41\x03\x02\x02\x02\u{159}\u{15a}\x07\x34\x02\x02\u{15a}\u{15b}\
	\x07\x05\x02\x02\u{15b}\u{15c}\x05\x3a\x1e\x02\u{15c}\u{15d}\x07\x06\x02\
	\x02\u{15d}\x43\x03\x02\x02\x02\u{15e}\u{162}\x05\x46\x24\x02\u{15f}\u{162}\
	\x05\x48\x25\x02\u{160}\u{162}\x05\x4e\x28\x02\u{161}\u{15e}\x03\x02\x02\
	\x02\u{161}\u{15f}\x03\x02\x02\x02\u{161}\u{160}\x03\x02\x02\x02\u{162}\
	\x45\x03\x02\x02\x02\u{163}\u{164}\x07\x61\x02\x02\u{164}\u{165}\x07\x32\
	\x02\x02\u{165}\u{166}\x07\x61\x02\x02\u{166}\x47\x03\x02\x02\x02\u{167}\
	\u{168}\x05\u{92}\x4a\x02\u{168}\u{16d}\x05\x4a\x26\x02\u{169}\u{16a}\x07\
	\x04\x02\x02\u{16a}\u{16c}\x05\x4a\x26\x02\u{16b}\u{169}\x03\x02\x02\x02\
	\u{16c}\u{16f}\x03\x02\x02\x02\u{16d}\u{16b}\x03\x02\x02\x02\u{16d}\u{16e}\
	\x03\x02\x02\x02\u{16e}\x49\x03\x02\x02\x02\u{16f}\u{16d}\x03\x02\x02\x02\
	\u{170}\u{199}\x07\x20\x02\x02\u{171}\u{172}\x07\x21\x02\x02\u{172}\u{199}\
	\x05\u{92}\x4a\x02\u{173}\u{174}\x07\x24\x02\x02\u{174}\u{177}\x05\u{96}\
	\x4c\x02\u{175}\u{176}\x07\x27\x02\x02\u{176}\u{178}\x05\u{96}\x4c\x02\u{177}\
	\u{175}\x03\x02\x02\x02\u{177}\u{178}\x03\x02\x02\x02\u{178}\u{17a}\x03\
	\x02\x02\x02\u{179}\u{17b}\x07\x25\x02\x02\u{17a}\u{179}\x03\x02\x02\x02\
	\u{17a}\u{17b}\x03\x02\x02\x02\u{17b}\u{199}\x03\x02\x02\x02\u{17c}\u{17d}\
	\x07\x29\x02\x02\u{17d}\u{180}\x05\u{96}\x4c\x02\u{17e}\u{17f}\x07\x27\x02\
	\x02\u{17f}\u{181}\x05\u{96}\x4c\x02\u{180}\u{17e}\x03\x02\x02\x02\u{180}\
	\u{181}\x03\x02\x02\x02\u{181}\u{199}\x03\x02\x02\x02\u{182}\u{183}\x07\
	\x28\x02\x02\u{183}\u{186}\x05\u{94}\x4b\x02\u{184}\u{185}\x07\x27\x02\x02\
	\u{185}\u{187}\x05\u{96}\x4c\x02\u{186}\u{184}\x03\x02\x02\x02\u{186}\u{187}\
	\x03\x02\x02\x02\u{187}\u{199}\x03\x02\x02\x02\u{188}\u{189}\x07\x31\x02\
	\x02\u{189}\u{199}\x05\u{a6}\x54\x02\u{18a}\u{18b}\x07\x26\x02\x02\u{18b}\
	\u{199}\x07\x5c\x02\x02\u{18c}\u{18d}\x07\x2a\x02\x02\u{18d}\u{18e}\x07\
	\x05\x02\x02\u{18e}\u{18f}\x05\x3a\x1e\x02\u{18f}\u{190}\x07\x06\x02\x02\
	\u{190}\u{199}\x03\x02\x02\x02\u{191}\u{192}\x07\x2b\x02\x02\u{192}\u{193}\
	\x07\x05\x02\x02\u{193}\u{194}\x05\x4c\x27\x02\u{194}\u{195}\x07\x06\x02\
	\x02\u{195}\u{199}\x03\x02\x02\x02\u{196}\u{197}\x07\x1f\x02\x02\u{197}\
	\u{199}\x05\u{98}\x4d\x02\u{198}\u{170}\x03\x02\x02\x02\u{198}\u{171}\x03\
	\x02\x02\x02\u{198}\u{173}\x03\x02\x02\x02\u{198}\u{17c}\x03\x02\x02\x02\
	\u{198}\u{182}\x03\x02\x02\x02\u{198}\u{188}\x03\x02\x02\x02\u{198}\u{18a}\
	\x03\x02\x02\x02\u{198}\u{18c}\x03\x02\x02\x02\u{198}\u{191}\x03\x02\x02\
	\x02\u{198}\u{196}\x03\x02\x02\x02\u{199}\x4b\x03\x02\x02\x02\u{19a}\u{19b}\
	\x05\x4e\x28\x02\u{19b}\u{19c}\x07\x03\x02\x02\u{19c}\u{19e}\x03\x02\x02\
	\x02\u{19d}\u{19a}\x03\x02\x02\x02\u{19e}\u{19f}\x03\x02\x02\x02\u{19f}\
	\u{19d}\x03\x02\x02\x02\u{19f}\u{1a0}\x03\x02\x02\x02\u{1a0}\x4d\x03\x02\
	\x02\x02\u{1a1}\u{1a5}\x05\x50\x29\x02\u{1a2}\u{1a5}\x05\x52\x2a\x02\u{1a3}\
	\u{1a5}\x05\x54\x2b\x02\u{1a4}\u{1a1}\x03\x02\x02\x02\u{1a4}\u{1a2}\x03\
	\x02\x02\x02\u{1a4}\u{1a3}\x03\x02\x02\x02\u{1a5}\x4f\x03\x02\x02\x02\u{1a6}\
	\u{1a7}\x07\x61\x02\x02\u{1a7}\u{1a8}\x07\x2d\x02\x02\u{1a8}\u{1ab}\x05\
	\u{96}\x4c\x02\u{1a9}\u{1aa}\x07\x04\x02\x02\u{1aa}\u{1ac}\x05\x5c\x2f\x02\
	\u{1ab}\u{1a9}\x03\x02\x02\x02\u{1ab}\u{1ac}\x03\x02\x02\x02\u{1ac}\u{1b7}\
	\x03\x02\x02\x02\u{1ad}\u{1ae}\x07\x61\x02\x02\u{1ae}\u{1af}\x07\x2c\x02\
	\x02\u{1af}\u{1b2}\x07\x64\x02\x02\u{1b0}\u{1b1}\x07\x04\x02\x02\u{1b1}\
	\u{1b3}\x05\x5c\x2f\x02\u{1b2}\u{1b0}\x03\x02\x02\x02\u{1b2}\u{1b3}\x03\
	\x02\x02\x02\u{1b3}\u{1b7}\x03\x02\x02\x02\u{1b4}\u{1b5}\x07\x61\x02\x02\
	\u{1b5}\u{1b7}\x05\x5c\x2f\x02\u{1b6}\u{1a6}\x03\x02\x02\x02\u{1b6}\u{1ad}\
	\x03\x02\x02\x02\u{1b6}\u{1b4}\x03\x02\x02\x02\u{1b7}\x51\x03\x02\x02\x02\
	\u{1b8}\u{1ba}\x07\x61\x02\x02\u{1b9}\u{1b8}\x03\x02\x02\x02\u{1b9}\u{1ba}\
	\x03\x02\x02\x02\u{1ba}\u{1bb}\x03\x02\x02\x02\u{1bb}\u{1bc}\x05\x56\x2c\
	\x02\u{1bc}\u{1bd}\x07\x2d\x02\x02\u{1bd}\u{1c0}\x05\u{96}\x4c\x02\u{1be}\
	\u{1bf}\x07\x04\x02\x02\u{1bf}\u{1c1}\x05\x5c\x2f\x02\u{1c0}\u{1be}\x03\
	\x02\x02\x02\u{1c0}\u{1c1}\x03\x02\x02\x02\u{1c1}\u{1ca}\x03\x02\x02\x02\
	\u{1c2}\u{1c4}\x07\x61\x02\x02\u{1c3}\u{1c2}\x03\x02\x02\x02\u{1c3}\u{1c4}\
	\x03\x02\x02\x02\u{1c4}\u{1c5}\x03\x02\x02\x02\u{1c5}\u{1c7}\x05\x56\x2c\
	\x02\u{1c6}\u{1c8}\x05\x5c\x2f\x02\u{1c7}\u{1c6}\x03\x02\x02\x02\u{1c7}\
	\u{1c8}\x03\x02\x02\x02\u{1c8}\u{1ca}\x03\x02\x02\x02\u{1c9}\u{1b9}\x03\
	\x02\x02\x02\u{1c9}\u{1c3}\x03\x02\x02\x02\u{1ca}\x53\x03\x02\x02\x02\u{1cb}\
	\u{1cd}\x07\x61\x02\x02\u{1cc}\u{1cb}\x03\x02\x02\x02\u{1cc}\u{1cd}\x03\
	\x02\x02\x02\u{1cd}\u{1ce}\x03\x02\x02\x02\u{1ce}\u{1cf}\x05\x60\x31\x02\
	\u{1cf}\u{1d0}\x07\x2d\x02\x02\u{1d0}\u{1d3}\x05\u{96}\x4c\x02\u{1d1}\u{1d2}\
	\x07\x04\x02\x02\u{1d2}\u{1d4}\x05\x5c\x2f\x02\u{1d3}\u{1d1}\x03\x02\x02\
	\x02\u{1d3}\u{1d4}\x03\x02\x02\x02\u{1d4}\u{1dd}\x03\x02\x02\x02\u{1d5}\
	\u{1d7}\x07\x61\x02\x02\u{1d6}\u{1d5}\x03\x02\x02\x02\u{1d6}\u{1d7}\x03\
	\x02\x02\x02\u{1d7}\u{1d8}\x03\x02\x02\x02\u{1d8}\u{1da}\x05\x60\x31\x02\
	\u{1d9}\u{1db}\x05\x5c\x2f\x02\u{1da}\u{1d9}\x03\x02\x02\x02\u{1da}\u{1db}\
	\x03\x02\x02\x02\u{1db}\u{1dd}\x03\x02\x02\x02\u{1dc}\u{1cc}\x03\x02\x02\
	\x02\u{1dc}\u{1d6}\x03\x02\x02\x02\u{1dd}\x55\x03\x02\x02\x02\u{1de}\u{1df}\
	\x07\x07\x02\x02\u{1df}\u{1e4}\x05\x58\x2d\x02\u{1e0}\u{1e1}\x07\x04\x02\
	\x02\u{1e1}\u{1e3}\x05\x58\x2d\x02\u{1e2}\u{1e0}\x03\x02\x02\x02\u{1e3}\
	\u{1e6}\x03\x02\x02\x02\u{1e4}\u{1e2}\x03\x02\x02\x02\u{1e4}\u{1e5}\x03\
	\x02\x02\x02\u{1e5}\u{1e7}\x03\x02\x02\x02\u{1e6}\u{1e4}\x03\x02\x02\x02\
	\u{1e7}\u{1e8}\x07\x08\x02\x02\u{1e8}\x57\x03\x02\x02\x02\u{1e9}\u{1ea}\
	\x05\u{96}\x4c\x02\u{1ea}\u{1eb}\x07\x09\x02\x02\u{1eb}\u{1ec}\x05\x5a\x2e\
	\x02\u{1ec}\u{1ef}\x03\x02\x02\x02\u{1ed}\u{1ef}\x05\x5a\x2e\x02\u{1ee}\
	\u{1e9}\x03\x02\x02\x02\u{1ee}\u{1ed}\x03\x02\x02\x02\u{1ef}\x59\x03\x02\
	\x02\x02\u{1f0}\u{1f1}\x07\x61\x02\x02\u{1f1}\x5b\x03\x02\x02\x02\u{1f2}\
	\u{1f7}\x05\x5e\x30\x02\u{1f3}\u{1f4}\x07\x04\x02\x02\u{1f4}\u{1f6}\x05\
	\x5e\x30\x02\u{1f5}\u{1f3}\x03\x02\x02\x02\u{1f6}\u{1f9}\x03\x02\x02\x02\
	\u{1f7}\u{1f5}\x03\x02\x02\x02\u{1f7}\u{1f8}\x03\x02\x02\x02\u{1f8}\x5d\
	\x03\x02\x02\x02\u{1f9}\u{1f7}\x03\x02\x02\x02\u{1fa}\u{1fb}\x07\x30\x02\
	\x02\u{1fb}\u{1fe}\x05\u{9c}\x4f\x02\u{1fc}\u{1ff}\x07\x61\x02\x02\u{1fd}\
	\u{1ff}\x05\x60\x31\x02\u{1fe}\u{1fc}\x03\x02\x02\x02\u{1fe}\u{1fd}\x03\
	\x02\x02\x02\u{1ff}\u{203}\x03\x02\x02\x02\u{200}\u{201}\x07\x30\x02\x02\
	\u{201}\u{203}\x07\x61\x02\x02\u{202}\u{1fa}\x03\x02\x02\x02\u{202}\u{200}\
	\x03\x02\x02\x02\u{203}\x5f\x03\x02\x02\x02\u{204}\u{20c}\x05\u{a8}\x55\
	\x02\u{205}\u{206}\x05\x62\x32\x02\u{206}\u{207}\x05\x66\x34\x02\u{207}\
	\u{20c}\x03\x02\x02\x02\u{208}\u{209}\x05\x64\x33\x02\u{209}\u{20a}\x07\
	\x5c\x02\x02\u{20a}\u{20c}\x03\x02\x02\x02\u{20b}\u{204}\x03\x02\x02\x02\
	\u{20b}\u{205}\x03\x02\x02\x02\u{20b}\u{208}\x03\x02\x02\x02\u{20c}\x61\
	\x03\x02\x02\x02\u{20d}\u{20e}\x09\x03\x02\x02\u{20e}\x63\x03\x02\x02\x02\
	\u{20f}\u{210}\x09\x04\x02\x02\u{210}\x65\x03\x02\x02\x02\u{211}\u{214}\
	\x05\u{a8}\x55\x02\u{212}\u{214}\x07\x61\x02\x02\u{213}\u{211}\x03\x02\x02\
	\x02\u{213}\u{212}\x03\x02\x02\x02\u{214}\x67\x03\x02\x02\x02\u{215}\u{216}\
	\x07\x18\x02\x02\u{216}\u{225}\x05\u{9c}\x4f\x02\u{217}\u{218}\x07\x18\x02\
	\x02\u{218}\u{219}\x05\u{9c}\x4f\x02\u{219}\u{21a}\x07\x09\x02\x02\u{21a}\
	\u{21b}\x07\x2a\x02\x02\u{21b}\u{21c}\x07\x05\x02\x02\u{21c}\u{21d}\x05\
	\x3a\x1e\x02\u{21d}\u{21e}\x07\x06\x02\x02\u{21e}\u{21f}\x07\x2b\x02\x02\
	\u{21f}\u{220}\x07\x05\x02\x02\u{220}\u{221}\x05\x4e\x28\x02\u{221}\u{222}\
	\x07\x03\x02\x02\u{222}\u{223}\x07\x06\x02\x02\u{223}\u{225}\x03\x02\x02\
	\x02\u{224}\u{215}\x03\x02\x02\x02\u{224}\u{217}\x03\x02\x02\x02\u{225}\
	\x69\x03\x02\x02\x02\u{226}\u{227}\x05\x6e\x38\x02\u{227}\u{228}\x07\x03\
	\x02\x02\u{228}\u{236}\x03\x02\x02\x02\u{229}\u{22a}\x05\x70\x39\x02\u{22a}\
	\u{22b}\x07\x03\x02\x02\u{22b}\u{236}\x03\x02\x02\x02\u{22c}\u{22d}\x05\
	\x72\x3a\x02\u{22d}\u{22e}\x07\x03\x02\x02\u{22e}\u{236}\x03\x02\x02\x02\
	\u{22f}\u{230}\x05\x74\x3b\x02\u{230}\u{231}\x07\x03\x02\x02\u{231}\u{236}\
	\x03\x02\x02\x02\u{232}\u{233}\x05\x76\x3c\x02\u{233}\u{234}\x07\x03\x02\
	\x02\u{234}\u{236}\x03\x02\x02\x02\u{235}\u{226}\x03\x02\x02\x02\u{235}\
	\u{229}\x03\x02\x02\x02\u{235}\u{22c}\x03\x02\x02\x02\u{235}\u{22f}\x03\
	\x02\x02\x02\u{235}\u{232}\x03\x02\x02\x02\u{236}\x6b\x03\x02\x02\x02\u{237}\
	\u{238}\x09\x05\x02\x02\u{238}\x6d\x03\x02\x02\x02\u{239}\u{23b}\x07\x3e\
	\x02\x02\u{23a}\u{23c}\x05\x78\x3d\x02\u{23b}\u{23a}\x03\x02\x02\x02\u{23b}\
	\u{23c}\x03\x02\x02\x02\u{23c}\x6f\x03\x02\x02\x02\u{23d}\u{23e}\x05\x6c\
	\x37\x02\u{23e}\u{243}\x05\x7a\x3e\x02\u{23f}\u{240}\x07\x04\x02\x02\u{240}\
	\u{242}\x05\x7a\x3e\x02\u{241}\u{23f}\x03\x02\x02\x02\u{242}\u{245}\x03\
	\x02\x02\x02\u{243}\u{241}\x03\x02\x02\x02\u{243}\u{244}\x03\x02\x02\x02\
	\u{244}\x71\x03\x02\x02\x02\u{245}\u{243}\x03\x02\x02\x02\u{246}\u{247}\
	\x07\x46\x02\x02\u{247}\u{24c}\x05\x7c\x3f\x02\u{248}\u{249}\x07\x04\x02\
	\x02\u{249}\u{24b}\x05\x7c\x3f\x02\u{24a}\u{248}\x03\x02\x02\x02\u{24b}\
	\u{24e}\x03\x02\x02\x02\u{24c}\u{24a}\x03\x02\x02\x02\u{24c}\u{24d}\x03\
	\x02\x02\x02\u{24d}\x73\x03\x02\x02\x02\u{24e}\u{24c}\x03\x02\x02\x02\u{24f}\
	\u{250}\x07\x45\x02\x02\u{250}\u{255}\x05\x7e\x40\x02\u{251}\u{252}\x07\
	\x04\x02\x02\u{252}\u{254}\x05\x7e\x40\x02\u{253}\u{251}\x03\x02\x02\x02\
	\u{254}\u{257}\x03\x02\x02\x02\u{255}\u{253}\x03\x02\x02\x02\u{255}\u{256}\
	\x03\x02\x02\x02\u{256}\x75\x03\x02\x02\x02\u{257}\u{255}\x03\x02\x02\x02\
	\u{258}\u{259}\x07\x47\x02\x02\u{259}\u{25e}\x05\u{80}\x41\x02\u{25a}\u{25b}\
	\x07\x04\x02\x02\u{25b}\u{25d}\x05\u{80}\x41\x02\u{25c}\u{25a}\x03\x02\x02\
	\x02\u{25d}\u{260}\x03\x02\x02\x02\u{25e}\u{25c}\x03\x02\x02\x02\u{25e}\
	\u{25f}\x03\x02\x02\x02\u{25f}\x77\x03\x02\x02\x02\u{260}\u{25e}\x03\x02\
	\x02\x02\u{261}\u{262}\x05\u{86}\x44\x02\u{262}\x79\x03\x02\x02\x02\u{263}\
	\u{266}\x05\u{86}\x44\x02\u{264}\u{266}\x05\u{84}\x43\x02\u{265}\u{263}\
	\x03\x02\x02\x02\u{265}\u{264}\x03\x02\x02\x02\u{266}\x7b\x03\x02\x02\x02\
	\u{267}\u{26b}\x05\u{86}\x44\x02\u{268}\u{26b}\x05\u{84}\x43\x02\u{269}\
	\u{26b}\x05\u{88}\x45\x02\u{26a}\u{267}\x03\x02\x02\x02\u{26a}\u{268}\x03\
	\x02\x02\x02\u{26a}\u{269}\x03\x02\x02\x02\u{26b}\x7d\x03\x02\x02\x02\u{26c}\
	\u{26f}\x05\u{86}\x44\x02\u{26d}\u{26f}\x05\u{88}\x45\x02\u{26e}\u{26c}\
	\x03\x02\x02\x02\u{26e}\u{26d}\x03\x02\x02\x02\u{26f}\x7f\x03\x02\x02\x02\
	\u{270}\u{273}\x05\u{86}\x44\x02\u{271}\u{273}\x05\u{82}\x42\x02\u{272}\
	\u{270}\x03\x02\x02\x02\u{272}\u{271}\x03\x02\x02\x02\u{273}\u{81}\x03\x02\
	\x02\x02\u{274}\u{275}\x07\x4b\x02\x02\u{275}\u{279}\x07\x64\x02\x02\u{276}\
	\u{277}\x07\x4c\x02\x02\u{277}\u{279}\x07\x64\x02\x02\u{278}\u{274}\x03\
	\x02\x02\x02\u{278}\u{276}\x03\x02\x02\x02\u{279}\u{83}\x03\x02\x02\x02\
	\u{27a}\u{27b}\x07\x4d\x02\x02\u{27b}\u{27c}\x05\u{9e}\x50\x02\u{27c}\u{85}\
	\x03\x02\x02\x02\u{27d}\u{27e}\x07\x4e\x02\x02\u{27e}\u{27f}\x05\u{9e}\x50\
	\x02\u{27f}\u{87}\x03\x02\x02\x02\u{280}\u{281}\x07\x4f\x02\x02\u{281}\u{285}\
	\x05\u{8a}\x46\x02\u{282}\u{283}\x07\x50\x02\x02\u{283}\u{285}\x05\u{8c}\
	\x47\x02\u{284}\u{280}\x03\x02\x02\x02\u{284}\u{282}\x03\x02\x02\x02\u{285}\
	\u{89}\x03\x02\x02\x02\u{286}\u{287}\x09\x06\x02\x02\u{287}\u{8b}\x03\x02\
	\x02\x02\u{288}\u{28b}\x05\u{90}\x49\x02\u{289}\u{28b}\x05\u{8e}\x48\x02\
	\u{28a}\u{288}\x03\x02\x02\x02\u{28a}\u{289}\x03\x02\x02\x02\u{28b}\u{8d}\
	\x03\x02\x02\x02\u{28c}\u{28d}\x07\x0a\x02\x02\u{28d}\u{292}\x05\u{90}\x49\
	\x02\u{28e}\u{28f}\x07\x04\x02\x02\u{28f}\u{291}\x05\u{90}\x49\x02\u{290}\
	\u{28e}\x03\x02\x02\x02\u{291}\u{294}\x03\x02\x02\x02\u{292}\u{290}\x03\
	\x02\x02\x02\u{292}\u{293}\x03\x02\x02\x02\u{293}\u{295}\x03\x02\x02\x02\
	\u{294}\u{292}\x03\x02\x02\x02\u{295}\u{296}\x07\x0b\x02\x02\u{296}\u{8f}\
	\x03\x02\x02\x02\u{297}\u{298}\x07\x51\x02\x02\u{298}\u{299}\x07\x35\x02\
	\x02\u{299}\u{2a4}\x07\x5d\x02\x02\u{29a}\u{29b}\x07\x52\x02\x02\u{29b}\
	\u{29c}\x07\x35\x02\x02\u{29c}\u{2a4}\x07\x5d\x02\x02\u{29d}\u{29e}\x07\
	\x53\x02\x02\u{29e}\u{29f}\x07\x35\x02\x02\u{29f}\u{2a4}\x07\x5d\x02\x02\
	\u{2a0}\u{2a1}\x07\x3c\x02\x02\u{2a1}\u{2a2}\x07\x35\x02\x02\u{2a2}\u{2a4}\
	\x07\x64\x02\x02\u{2a3}\u{297}\x03\x02\x02\x02\u{2a3}\u{29a}\x03\x02\x02\
	\x02\u{2a3}\u{29d}\x03\x02\x02\x02\u{2a3}\u{2a0}\x03\x02\x02\x02\u{2a4}\
	\u{91}\x03\x02\x02\x02\u{2a5}\u{2a9}\x05\u{94}\x4b\x02\u{2a6}\u{2a9}\x05\
	\u{96}\x4c\x02\u{2a7}\u{2a9}\x07\x61\x02\x02\u{2a8}\u{2a5}\x03\x02\x02\x02\
	\u{2a8}\u{2a6}\x03\x02\x02\x02\u{2a8}\u{2a7}\x03\x02\x02\x02\u{2a9}\u{93}\
	\x03\x02\x02\x02\u{2aa}\u{2ad}\x05\u{9a}\x4e\x02\u{2ab}\u{2ad}\x07\x61\x02\
	\x02\u{2ac}\u{2aa}\x03\x02\x02\x02\u{2ac}\u{2ab}\x03\x02\x02\x02\u{2ad}\
	\u{95}\x03\x02\x02\x02\u{2ae}\u{2b1}\x05\u{9c}\x4f\x02\u{2af}\u{2b1}\x07\
	\x61\x02\x02\u{2b0}\u{2ae}\x03\x02\x02\x02\u{2b0}\u{2af}\x03\x02\x02\x02\
	\u{2b1}\u{97}\x03\x02\x02\x02\u{2b2}\u{2b5}\x05\u{9a}\x4e\x02\u{2b3}\u{2b5}\
	\x05\u{9c}\x4f\x02\u{2b4}\u{2b2}\x03\x02\x02\x02\u{2b4}\u{2b3}\x03\x02\x02\
	\x02\u{2b5}\u{99}\x03\x02\x02\x02\u{2b6}\u{2b7}\x07\x66\x02\x02\u{2b7}\u{9b}\
	\x03\x02\x02\x02\u{2b8}\u{2bd}\x07\x65\x02\x02\u{2b9}\u{2bd}\x05\u{a2}\x52\
	\x02\u{2ba}\u{2bd}\x05\u{a4}\x53\x02\u{2bb}\u{2bd}\x05\u{ac}\x57\x02\u{2bc}\
	\u{2b8}\x03\x02\x02\x02\u{2bc}\u{2b9}\x03\x02\x02\x02\u{2bc}\u{2ba}\x03\
	\x02\x02\x02\u{2bc}\u{2bb}\x03\x02\x02\x02\u{2bd}\u{9d}\x03\x02\x02\x02\
	\u{2be}\u{2c1}\x05\u{9c}\x4f\x02\u{2bf}\u{2c1}\x05\u{a0}\x51\x02\u{2c0}\
	\u{2be}\x03\x02\x02\x02\u{2c0}\u{2bf}\x03\x02\x02\x02\u{2c1}\u{9f}\x03\x02\
	\x02\x02\u{2c2}\u{2c3}\x07\x0a\x02\x02\u{2c3}\u{2c8}\x05\u{9c}\x4f\x02\u{2c4}\
	\u{2c5}\x07\x04\x02\x02\u{2c5}\u{2c7}\x05\u{9c}\x4f\x02\u{2c6}\u{2c4}\x03\
	\x02\x02\x02\u{2c7}\u{2ca}\x03\x02\x02\x02\u{2c8}\u{2c6}\x03\x02\x02\x02\
	\u{2c8}\u{2c9}\x03\x02\x02\x02\u{2c9}\u{2cb}\x03\x02\x02\x02\u{2ca}\u{2c8}\
	\x03\x02\x02\x02\u{2cb}\u{2cc}\x07\x0b\x02\x02\u{2cc}\u{a1}\x03\x02\x02\
	\x02\u{2cd}\u{2ce}\x07\x18\x02\x02\u{2ce}\u{a3}\x03\x02\x02\x02\u{2cf}\u{2d0}\
	\x09\x07\x02\x02\u{2d0}\u{a5}\x03\x02\x02\x02\u{2d1}\u{2d2}\x09\x08\x02\
	\x02\u{2d2}\u{a7}\x03\x02\x02\x02\u{2d3}\u{2d4}\x09\x09\x02\x02\u{2d4}\u{a9}\
	\x03\x02\x02\x02\u{2d5}\u{2d6}\x07\x5c\x02\x02\u{2d6}\u{ab}\x03\x02\x02\
	\x02\u{2d7}\u{2d8}\x09\x0a\x02\x02\u{2d8}\u{ad}\x03\x02\x02\x02\x46\u{b4}\
	\u{d3}\u{e2}\u{ea}\u{100}\u{105}\u{10a}\u{10f}\u{117}\u{11d}\u{127}\u{136}\
	\u{13a}\u{141}\u{147}\u{157}\u{161}\u{16d}\u{177}\u{17a}\u{180}\u{186}\u{198}\
	\u{19f}\u{1a4}\u{1ab}\u{1b2}\u{1b6}\u{1b9}\u{1c0}\u{1c3}\u{1c7}\u{1c9}\u{1cc}\
	\u{1d3}\u{1d6}\u{1da}\u{1dc}\u{1e4}\u{1ee}\u{1f7}\u{1fe}\u{202}\u{20b}\u{213}\
	\u{224}\u{235}\u{23b}\u{243}\u{24c}\u{255}\u{25e}\u{265}\u{26a}\u{26e}\u{272}\
	\u{278}\u{284}\u{28a}\u{292}\u{2a3}\u{2a8}\u{2ac}\u{2b0}\u{2b4}\u{2bc}\u{2c0}\
	\u{2c8}";

