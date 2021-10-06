#![allow(nonstandard_style)]
// Generated from /home/vmax/.cache/bazel/_bazel_vmax/78a5ef47aa0f794ae8d175ad83930027/sandbox/linux-sandbox/19/execroot/__main__/bazel-out/k8-fastbuild/bin/TypeQLRust.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor};
use super::typeqlrustparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link TypeQLRustParser}.
 */
pub trait TypeQLRustVisitor<'input>: ParseTreeVisitor<'input,TypeQLRustParserContextType>{
	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#eof_query}.
	 * @param ctx the parse tree
	 */
	fn visit_eof_query(&mut self, ctx: &Eof_queryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#eof_queries}.
	 * @param ctx the parse tree
	 */
	fn visit_eof_queries(&mut self, ctx: &Eof_queriesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#eof_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_eof_pattern(&mut self, ctx: &Eof_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#eof_patterns}.
	 * @param ctx the parse tree
	 */
	fn visit_eof_patterns(&mut self, ctx: &Eof_patternsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#eof_definables}.
	 * @param ctx the parse tree
	 */
	fn visit_eof_definables(&mut self, ctx: &Eof_definablesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#eof_variable}.
	 * @param ctx the parse tree
	 */
	fn visit_eof_variable(&mut self, ctx: &Eof_variableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#eof_label}.
	 * @param ctx the parse tree
	 */
	fn visit_eof_label(&mut self, ctx: &Eof_labelContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#eof_schema_rule}.
	 * @param ctx the parse tree
	 */
	fn visit_eof_schema_rule(&mut self, ctx: &Eof_schema_ruleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#query}.
	 * @param ctx the parse tree
	 */
	fn visit_query(&mut self, ctx: &QueryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#query_define}.
	 * @param ctx the parse tree
	 */
	fn visit_query_define(&mut self, ctx: &Query_defineContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#query_undefine}.
	 * @param ctx the parse tree
	 */
	fn visit_query_undefine(&mut self, ctx: &Query_undefineContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#query_insert}.
	 * @param ctx the parse tree
	 */
	fn visit_query_insert(&mut self, ctx: &Query_insertContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#query_delete_or_update}.
	 * @param ctx the parse tree
	 */
	fn visit_query_delete_or_update(&mut self, ctx: &Query_delete_or_updateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#query_match}.
	 * @param ctx the parse tree
	 */
	fn visit_query_match(&mut self, ctx: &Query_matchContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#query_compute}.
	 * @param ctx the parse tree
	 */
	fn visit_query_compute(&mut self, ctx: &Query_computeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#query_match_aggregate}.
	 * @param ctx the parse tree
	 */
	fn visit_query_match_aggregate(&mut self, ctx: &Query_match_aggregateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#query_match_group}.
	 * @param ctx the parse tree
	 */
	fn visit_query_match_group(&mut self, ctx: &Query_match_groupContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#query_match_group_agg}.
	 * @param ctx the parse tree
	 */
	fn visit_query_match_group_agg(&mut self, ctx: &Query_match_group_aggContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#modifiers}.
	 * @param ctx the parse tree
	 */
	fn visit_modifiers(&mut self, ctx: &ModifiersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#filter_}.
	 * @param ctx the parse tree
	 */
	fn visit_filter_(&mut self, ctx: &Filter_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#sort}.
	 * @param ctx the parse tree
	 */
	fn visit_sort(&mut self, ctx: &SortContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#offset}.
	 * @param ctx the parse tree
	 */
	fn visit_offset(&mut self, ctx: &OffsetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#limit}.
	 * @param ctx the parse tree
	 */
	fn visit_limit(&mut self, ctx: &LimitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#match_aggregate}.
	 * @param ctx the parse tree
	 */
	fn visit_match_aggregate(&mut self, ctx: &Match_aggregateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#aggregate_method}.
	 * @param ctx the parse tree
	 */
	fn visit_aggregate_method(&mut self, ctx: &Aggregate_methodContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#match_group}.
	 * @param ctx the parse tree
	 */
	fn visit_match_group(&mut self, ctx: &Match_groupContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#definables}.
	 * @param ctx the parse tree
	 */
	fn visit_definables(&mut self, ctx: &DefinablesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#definable}.
	 * @param ctx the parse tree
	 */
	fn visit_definable(&mut self, ctx: &DefinableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#patterns}.
	 * @param ctx the parse tree
	 */
	fn visit_patterns(&mut self, ctx: &PatternsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_pattern(&mut self, ctx: &PatternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#pattern_conjunction}.
	 * @param ctx the parse tree
	 */
	fn visit_pattern_conjunction(&mut self, ctx: &Pattern_conjunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#pattern_disjunction}.
	 * @param ctx the parse tree
	 */
	fn visit_pattern_disjunction(&mut self, ctx: &Pattern_disjunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#pattern_negation}.
	 * @param ctx the parse tree
	 */
	fn visit_pattern_negation(&mut self, ctx: &Pattern_negationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#pattern_variable}.
	 * @param ctx the parse tree
	 */
	fn visit_pattern_variable(&mut self, ctx: &Pattern_variableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#variable_concept}.
	 * @param ctx the parse tree
	 */
	fn visit_variable_concept(&mut self, ctx: &Variable_conceptContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#variable_type}.
	 * @param ctx the parse tree
	 */
	fn visit_variable_type(&mut self, ctx: &Variable_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#type_constraint}.
	 * @param ctx the parse tree
	 */
	fn visit_type_constraint(&mut self, ctx: &Type_constraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#variable_things}.
	 * @param ctx the parse tree
	 */
	fn visit_variable_things(&mut self, ctx: &Variable_thingsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#variable_thing_any}.
	 * @param ctx the parse tree
	 */
	fn visit_variable_thing_any(&mut self, ctx: &Variable_thing_anyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#variable_thing}.
	 * @param ctx the parse tree
	 */
	fn visit_variable_thing(&mut self, ctx: &Variable_thingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#variable_relation}.
	 * @param ctx the parse tree
	 */
	fn visit_variable_relation(&mut self, ctx: &Variable_relationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#variable_attribute}.
	 * @param ctx the parse tree
	 */
	fn visit_variable_attribute(&mut self, ctx: &Variable_attributeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#relation}.
	 * @param ctx the parse tree
	 */
	fn visit_relation(&mut self, ctx: &RelationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#role_player}.
	 * @param ctx the parse tree
	 */
	fn visit_role_player(&mut self, ctx: &Role_playerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#player}.
	 * @param ctx the parse tree
	 */
	fn visit_player(&mut self, ctx: &PlayerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#attributes}.
	 * @param ctx the parse tree
	 */
	fn visit_attributes(&mut self, ctx: &AttributesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#attribute}.
	 * @param ctx the parse tree
	 */
	fn visit_attribute(&mut self, ctx: &AttributeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#predicate}.
	 * @param ctx the parse tree
	 */
	fn visit_predicate(&mut self, ctx: &PredicateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#predicate_equality}.
	 * @param ctx the parse tree
	 */
	fn visit_predicate_equality(&mut self, ctx: &Predicate_equalityContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#predicate_substring}.
	 * @param ctx the parse tree
	 */
	fn visit_predicate_substring(&mut self, ctx: &Predicate_substringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#predicate_value}.
	 * @param ctx the parse tree
	 */
	fn visit_predicate_value(&mut self, ctx: &Predicate_valueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#schema_rule}.
	 * @param ctx the parse tree
	 */
	fn visit_schema_rule(&mut self, ctx: &Schema_ruleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#compute_conditions}.
	 * @param ctx the parse tree
	 */
	fn visit_compute_conditions(&mut self, ctx: &Compute_conditionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#compute_method}.
	 * @param ctx the parse tree
	 */
	fn visit_compute_method(&mut self, ctx: &Compute_methodContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#conditions_count}.
	 * @param ctx the parse tree
	 */
	fn visit_conditions_count(&mut self, ctx: &Conditions_countContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#conditions_value}.
	 * @param ctx the parse tree
	 */
	fn visit_conditions_value(&mut self, ctx: &Conditions_valueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#conditions_central}.
	 * @param ctx the parse tree
	 */
	fn visit_conditions_central(&mut self, ctx: &Conditions_centralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#conditions_cluster}.
	 * @param ctx the parse tree
	 */
	fn visit_conditions_cluster(&mut self, ctx: &Conditions_clusterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#conditions_path}.
	 * @param ctx the parse tree
	 */
	fn visit_conditions_path(&mut self, ctx: &Conditions_pathContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#input_count}.
	 * @param ctx the parse tree
	 */
	fn visit_input_count(&mut self, ctx: &Input_countContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#input_value}.
	 * @param ctx the parse tree
	 */
	fn visit_input_value(&mut self, ctx: &Input_valueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#input_central}.
	 * @param ctx the parse tree
	 */
	fn visit_input_central(&mut self, ctx: &Input_centralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#input_cluster}.
	 * @param ctx the parse tree
	 */
	fn visit_input_cluster(&mut self, ctx: &Input_clusterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#input_path}.
	 * @param ctx the parse tree
	 */
	fn visit_input_path(&mut self, ctx: &Input_pathContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#compute_direction}.
	 * @param ctx the parse tree
	 */
	fn visit_compute_direction(&mut self, ctx: &Compute_directionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#compute_target}.
	 * @param ctx the parse tree
	 */
	fn visit_compute_target(&mut self, ctx: &Compute_targetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#compute_scope}.
	 * @param ctx the parse tree
	 */
	fn visit_compute_scope(&mut self, ctx: &Compute_scopeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#compute_config}.
	 * @param ctx the parse tree
	 */
	fn visit_compute_config(&mut self, ctx: &Compute_configContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#compute_algorithm}.
	 * @param ctx the parse tree
	 */
	fn visit_compute_algorithm(&mut self, ctx: &Compute_algorithmContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#compute_args}.
	 * @param ctx the parse tree
	 */
	fn visit_compute_args(&mut self, ctx: &Compute_argsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#compute_args_array}.
	 * @param ctx the parse tree
	 */
	fn visit_compute_args_array(&mut self, ctx: &Compute_args_arrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#compute_arg}.
	 * @param ctx the parse tree
	 */
	fn visit_compute_arg(&mut self, ctx: &Compute_argContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#type_any}.
	 * @param ctx the parse tree
	 */
	fn visit_type_any(&mut self, ctx: &Type_anyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#type_scoped}.
	 * @param ctx the parse tree
	 */
	fn visit_type_scoped(&mut self, ctx: &Type_scopedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#type_}.
	 * @param ctx the parse tree
	 */
	fn visit_type_(&mut self, ctx: &Type_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#label_any}.
	 * @param ctx the parse tree
	 */
	fn visit_label_any(&mut self, ctx: &Label_anyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#label_scoped}.
	 * @param ctx the parse tree
	 */
	fn visit_label_scoped(&mut self, ctx: &Label_scopedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#label}.
	 * @param ctx the parse tree
	 */
	fn visit_label(&mut self, ctx: &LabelContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#labels}.
	 * @param ctx the parse tree
	 */
	fn visit_labels(&mut self, ctx: &LabelsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#label_array}.
	 * @param ctx the parse tree
	 */
	fn visit_label_array(&mut self, ctx: &Label_arrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#schema_native}.
	 * @param ctx the parse tree
	 */
	fn visit_schema_native(&mut self, ctx: &Schema_nativeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#type_native}.
	 * @param ctx the parse tree
	 */
	fn visit_type_native(&mut self, ctx: &Type_nativeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#value_type}.
	 * @param ctx the parse tree
	 */
	fn visit_value_type(&mut self, ctx: &Value_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#value}.
	 * @param ctx the parse tree
	 */
	fn visit_value(&mut self, ctx: &ValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#regex}.
	 * @param ctx the parse tree
	 */
	fn visit_regex(&mut self, ctx: &RegexContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TypeQLRustParser#unreserved}.
	 * @param ctx the parse tree
	 */
	fn visit_unreserved(&mut self, ctx: &UnreservedContext<'input>) { self.visit_children(ctx) }


}