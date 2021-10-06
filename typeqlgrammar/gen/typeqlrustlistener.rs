#![allow(nonstandard_style)]
// Generated from /home/vmax/.cache/bazel/_bazel_vmax/78a5ef47aa0f794ae8d175ad83930027/sandbox/linux-sandbox/19/execroot/__main__/bazel-out/k8-fastbuild/bin/TypeQLRust.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::typeqlrustparser::*;

pub trait TypeQLRustListener<'input> : ParseTreeListener<'input,TypeQLRustParserContextType>{

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#eof_query}.
 * @param ctx the parse tree
 */
fn enter_eof_query(&mut self, _ctx: &Eof_queryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#eof_query}.
 * @param ctx the parse tree
 */
fn exit_eof_query(&mut self, _ctx: &Eof_queryContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#eof_queries}.
 * @param ctx the parse tree
 */
fn enter_eof_queries(&mut self, _ctx: &Eof_queriesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#eof_queries}.
 * @param ctx the parse tree
 */
fn exit_eof_queries(&mut self, _ctx: &Eof_queriesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#eof_pattern}.
 * @param ctx the parse tree
 */
fn enter_eof_pattern(&mut self, _ctx: &Eof_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#eof_pattern}.
 * @param ctx the parse tree
 */
fn exit_eof_pattern(&mut self, _ctx: &Eof_patternContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#eof_patterns}.
 * @param ctx the parse tree
 */
fn enter_eof_patterns(&mut self, _ctx: &Eof_patternsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#eof_patterns}.
 * @param ctx the parse tree
 */
fn exit_eof_patterns(&mut self, _ctx: &Eof_patternsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#eof_definables}.
 * @param ctx the parse tree
 */
fn enter_eof_definables(&mut self, _ctx: &Eof_definablesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#eof_definables}.
 * @param ctx the parse tree
 */
fn exit_eof_definables(&mut self, _ctx: &Eof_definablesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#eof_variable}.
 * @param ctx the parse tree
 */
fn enter_eof_variable(&mut self, _ctx: &Eof_variableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#eof_variable}.
 * @param ctx the parse tree
 */
fn exit_eof_variable(&mut self, _ctx: &Eof_variableContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#eof_label}.
 * @param ctx the parse tree
 */
fn enter_eof_label(&mut self, _ctx: &Eof_labelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#eof_label}.
 * @param ctx the parse tree
 */
fn exit_eof_label(&mut self, _ctx: &Eof_labelContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#eof_schema_rule}.
 * @param ctx the parse tree
 */
fn enter_eof_schema_rule(&mut self, _ctx: &Eof_schema_ruleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#eof_schema_rule}.
 * @param ctx the parse tree
 */
fn exit_eof_schema_rule(&mut self, _ctx: &Eof_schema_ruleContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#query}.
 * @param ctx the parse tree
 */
fn enter_query(&mut self, _ctx: &QueryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#query}.
 * @param ctx the parse tree
 */
fn exit_query(&mut self, _ctx: &QueryContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#query_define}.
 * @param ctx the parse tree
 */
fn enter_query_define(&mut self, _ctx: &Query_defineContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#query_define}.
 * @param ctx the parse tree
 */
fn exit_query_define(&mut self, _ctx: &Query_defineContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#query_undefine}.
 * @param ctx the parse tree
 */
fn enter_query_undefine(&mut self, _ctx: &Query_undefineContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#query_undefine}.
 * @param ctx the parse tree
 */
fn exit_query_undefine(&mut self, _ctx: &Query_undefineContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#query_insert}.
 * @param ctx the parse tree
 */
fn enter_query_insert(&mut self, _ctx: &Query_insertContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#query_insert}.
 * @param ctx the parse tree
 */
fn exit_query_insert(&mut self, _ctx: &Query_insertContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#query_delete_or_update}.
 * @param ctx the parse tree
 */
fn enter_query_delete_or_update(&mut self, _ctx: &Query_delete_or_updateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#query_delete_or_update}.
 * @param ctx the parse tree
 */
fn exit_query_delete_or_update(&mut self, _ctx: &Query_delete_or_updateContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#query_match}.
 * @param ctx the parse tree
 */
fn enter_query_match(&mut self, _ctx: &Query_matchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#query_match}.
 * @param ctx the parse tree
 */
fn exit_query_match(&mut self, _ctx: &Query_matchContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#query_compute}.
 * @param ctx the parse tree
 */
fn enter_query_compute(&mut self, _ctx: &Query_computeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#query_compute}.
 * @param ctx the parse tree
 */
fn exit_query_compute(&mut self, _ctx: &Query_computeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#query_match_aggregate}.
 * @param ctx the parse tree
 */
fn enter_query_match_aggregate(&mut self, _ctx: &Query_match_aggregateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#query_match_aggregate}.
 * @param ctx the parse tree
 */
fn exit_query_match_aggregate(&mut self, _ctx: &Query_match_aggregateContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#query_match_group}.
 * @param ctx the parse tree
 */
fn enter_query_match_group(&mut self, _ctx: &Query_match_groupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#query_match_group}.
 * @param ctx the parse tree
 */
fn exit_query_match_group(&mut self, _ctx: &Query_match_groupContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#query_match_group_agg}.
 * @param ctx the parse tree
 */
fn enter_query_match_group_agg(&mut self, _ctx: &Query_match_group_aggContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#query_match_group_agg}.
 * @param ctx the parse tree
 */
fn exit_query_match_group_agg(&mut self, _ctx: &Query_match_group_aggContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#modifiers}.
 * @param ctx the parse tree
 */
fn enter_modifiers(&mut self, _ctx: &ModifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#modifiers}.
 * @param ctx the parse tree
 */
fn exit_modifiers(&mut self, _ctx: &ModifiersContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#filter_}.
 * @param ctx the parse tree
 */
fn enter_filter_(&mut self, _ctx: &Filter_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#filter_}.
 * @param ctx the parse tree
 */
fn exit_filter_(&mut self, _ctx: &Filter_Context<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#sort}.
 * @param ctx the parse tree
 */
fn enter_sort(&mut self, _ctx: &SortContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#sort}.
 * @param ctx the parse tree
 */
fn exit_sort(&mut self, _ctx: &SortContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#offset}.
 * @param ctx the parse tree
 */
fn enter_offset(&mut self, _ctx: &OffsetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#offset}.
 * @param ctx the parse tree
 */
fn exit_offset(&mut self, _ctx: &OffsetContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#limit}.
 * @param ctx the parse tree
 */
fn enter_limit(&mut self, _ctx: &LimitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#limit}.
 * @param ctx the parse tree
 */
fn exit_limit(&mut self, _ctx: &LimitContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#match_aggregate}.
 * @param ctx the parse tree
 */
fn enter_match_aggregate(&mut self, _ctx: &Match_aggregateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#match_aggregate}.
 * @param ctx the parse tree
 */
fn exit_match_aggregate(&mut self, _ctx: &Match_aggregateContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#aggregate_method}.
 * @param ctx the parse tree
 */
fn enter_aggregate_method(&mut self, _ctx: &Aggregate_methodContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#aggregate_method}.
 * @param ctx the parse tree
 */
fn exit_aggregate_method(&mut self, _ctx: &Aggregate_methodContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#match_group}.
 * @param ctx the parse tree
 */
fn enter_match_group(&mut self, _ctx: &Match_groupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#match_group}.
 * @param ctx the parse tree
 */
fn exit_match_group(&mut self, _ctx: &Match_groupContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#definables}.
 * @param ctx the parse tree
 */
fn enter_definables(&mut self, _ctx: &DefinablesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#definables}.
 * @param ctx the parse tree
 */
fn exit_definables(&mut self, _ctx: &DefinablesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#definable}.
 * @param ctx the parse tree
 */
fn enter_definable(&mut self, _ctx: &DefinableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#definable}.
 * @param ctx the parse tree
 */
fn exit_definable(&mut self, _ctx: &DefinableContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#patterns}.
 * @param ctx the parse tree
 */
fn enter_patterns(&mut self, _ctx: &PatternsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#patterns}.
 * @param ctx the parse tree
 */
fn exit_patterns(&mut self, _ctx: &PatternsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#pattern}.
 * @param ctx the parse tree
 */
fn enter_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#pattern}.
 * @param ctx the parse tree
 */
fn exit_pattern(&mut self, _ctx: &PatternContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#pattern_conjunction}.
 * @param ctx the parse tree
 */
fn enter_pattern_conjunction(&mut self, _ctx: &Pattern_conjunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#pattern_conjunction}.
 * @param ctx the parse tree
 */
fn exit_pattern_conjunction(&mut self, _ctx: &Pattern_conjunctionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#pattern_disjunction}.
 * @param ctx the parse tree
 */
fn enter_pattern_disjunction(&mut self, _ctx: &Pattern_disjunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#pattern_disjunction}.
 * @param ctx the parse tree
 */
fn exit_pattern_disjunction(&mut self, _ctx: &Pattern_disjunctionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#pattern_negation}.
 * @param ctx the parse tree
 */
fn enter_pattern_negation(&mut self, _ctx: &Pattern_negationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#pattern_negation}.
 * @param ctx the parse tree
 */
fn exit_pattern_negation(&mut self, _ctx: &Pattern_negationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#pattern_variable}.
 * @param ctx the parse tree
 */
fn enter_pattern_variable(&mut self, _ctx: &Pattern_variableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#pattern_variable}.
 * @param ctx the parse tree
 */
fn exit_pattern_variable(&mut self, _ctx: &Pattern_variableContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#variable_concept}.
 * @param ctx the parse tree
 */
fn enter_variable_concept(&mut self, _ctx: &Variable_conceptContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#variable_concept}.
 * @param ctx the parse tree
 */
fn exit_variable_concept(&mut self, _ctx: &Variable_conceptContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#variable_type}.
 * @param ctx the parse tree
 */
fn enter_variable_type(&mut self, _ctx: &Variable_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#variable_type}.
 * @param ctx the parse tree
 */
fn exit_variable_type(&mut self, _ctx: &Variable_typeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#type_constraint}.
 * @param ctx the parse tree
 */
fn enter_type_constraint(&mut self, _ctx: &Type_constraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#type_constraint}.
 * @param ctx the parse tree
 */
fn exit_type_constraint(&mut self, _ctx: &Type_constraintContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#variable_things}.
 * @param ctx the parse tree
 */
fn enter_variable_things(&mut self, _ctx: &Variable_thingsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#variable_things}.
 * @param ctx the parse tree
 */
fn exit_variable_things(&mut self, _ctx: &Variable_thingsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#variable_thing_any}.
 * @param ctx the parse tree
 */
fn enter_variable_thing_any(&mut self, _ctx: &Variable_thing_anyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#variable_thing_any}.
 * @param ctx the parse tree
 */
fn exit_variable_thing_any(&mut self, _ctx: &Variable_thing_anyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#variable_thing}.
 * @param ctx the parse tree
 */
fn enter_variable_thing(&mut self, _ctx: &Variable_thingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#variable_thing}.
 * @param ctx the parse tree
 */
fn exit_variable_thing(&mut self, _ctx: &Variable_thingContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#variable_relation}.
 * @param ctx the parse tree
 */
fn enter_variable_relation(&mut self, _ctx: &Variable_relationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#variable_relation}.
 * @param ctx the parse tree
 */
fn exit_variable_relation(&mut self, _ctx: &Variable_relationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#variable_attribute}.
 * @param ctx the parse tree
 */
fn enter_variable_attribute(&mut self, _ctx: &Variable_attributeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#variable_attribute}.
 * @param ctx the parse tree
 */
fn exit_variable_attribute(&mut self, _ctx: &Variable_attributeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#relation}.
 * @param ctx the parse tree
 */
fn enter_relation(&mut self, _ctx: &RelationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#relation}.
 * @param ctx the parse tree
 */
fn exit_relation(&mut self, _ctx: &RelationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#role_player}.
 * @param ctx the parse tree
 */
fn enter_role_player(&mut self, _ctx: &Role_playerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#role_player}.
 * @param ctx the parse tree
 */
fn exit_role_player(&mut self, _ctx: &Role_playerContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#player}.
 * @param ctx the parse tree
 */
fn enter_player(&mut self, _ctx: &PlayerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#player}.
 * @param ctx the parse tree
 */
fn exit_player(&mut self, _ctx: &PlayerContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#attributes}.
 * @param ctx the parse tree
 */
fn enter_attributes(&mut self, _ctx: &AttributesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#attributes}.
 * @param ctx the parse tree
 */
fn exit_attributes(&mut self, _ctx: &AttributesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#attribute}.
 * @param ctx the parse tree
 */
fn enter_attribute(&mut self, _ctx: &AttributeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#attribute}.
 * @param ctx the parse tree
 */
fn exit_attribute(&mut self, _ctx: &AttributeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#predicate}.
 * @param ctx the parse tree
 */
fn enter_predicate(&mut self, _ctx: &PredicateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#predicate}.
 * @param ctx the parse tree
 */
fn exit_predicate(&mut self, _ctx: &PredicateContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#predicate_equality}.
 * @param ctx the parse tree
 */
fn enter_predicate_equality(&mut self, _ctx: &Predicate_equalityContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#predicate_equality}.
 * @param ctx the parse tree
 */
fn exit_predicate_equality(&mut self, _ctx: &Predicate_equalityContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#predicate_substring}.
 * @param ctx the parse tree
 */
fn enter_predicate_substring(&mut self, _ctx: &Predicate_substringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#predicate_substring}.
 * @param ctx the parse tree
 */
fn exit_predicate_substring(&mut self, _ctx: &Predicate_substringContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#predicate_value}.
 * @param ctx the parse tree
 */
fn enter_predicate_value(&mut self, _ctx: &Predicate_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#predicate_value}.
 * @param ctx the parse tree
 */
fn exit_predicate_value(&mut self, _ctx: &Predicate_valueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#schema_rule}.
 * @param ctx the parse tree
 */
fn enter_schema_rule(&mut self, _ctx: &Schema_ruleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#schema_rule}.
 * @param ctx the parse tree
 */
fn exit_schema_rule(&mut self, _ctx: &Schema_ruleContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#compute_conditions}.
 * @param ctx the parse tree
 */
fn enter_compute_conditions(&mut self, _ctx: &Compute_conditionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#compute_conditions}.
 * @param ctx the parse tree
 */
fn exit_compute_conditions(&mut self, _ctx: &Compute_conditionsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#compute_method}.
 * @param ctx the parse tree
 */
fn enter_compute_method(&mut self, _ctx: &Compute_methodContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#compute_method}.
 * @param ctx the parse tree
 */
fn exit_compute_method(&mut self, _ctx: &Compute_methodContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#conditions_count}.
 * @param ctx the parse tree
 */
fn enter_conditions_count(&mut self, _ctx: &Conditions_countContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#conditions_count}.
 * @param ctx the parse tree
 */
fn exit_conditions_count(&mut self, _ctx: &Conditions_countContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#conditions_value}.
 * @param ctx the parse tree
 */
fn enter_conditions_value(&mut self, _ctx: &Conditions_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#conditions_value}.
 * @param ctx the parse tree
 */
fn exit_conditions_value(&mut self, _ctx: &Conditions_valueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#conditions_central}.
 * @param ctx the parse tree
 */
fn enter_conditions_central(&mut self, _ctx: &Conditions_centralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#conditions_central}.
 * @param ctx the parse tree
 */
fn exit_conditions_central(&mut self, _ctx: &Conditions_centralContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#conditions_cluster}.
 * @param ctx the parse tree
 */
fn enter_conditions_cluster(&mut self, _ctx: &Conditions_clusterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#conditions_cluster}.
 * @param ctx the parse tree
 */
fn exit_conditions_cluster(&mut self, _ctx: &Conditions_clusterContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#conditions_path}.
 * @param ctx the parse tree
 */
fn enter_conditions_path(&mut self, _ctx: &Conditions_pathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#conditions_path}.
 * @param ctx the parse tree
 */
fn exit_conditions_path(&mut self, _ctx: &Conditions_pathContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#input_count}.
 * @param ctx the parse tree
 */
fn enter_input_count(&mut self, _ctx: &Input_countContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#input_count}.
 * @param ctx the parse tree
 */
fn exit_input_count(&mut self, _ctx: &Input_countContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#input_value}.
 * @param ctx the parse tree
 */
fn enter_input_value(&mut self, _ctx: &Input_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#input_value}.
 * @param ctx the parse tree
 */
fn exit_input_value(&mut self, _ctx: &Input_valueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#input_central}.
 * @param ctx the parse tree
 */
fn enter_input_central(&mut self, _ctx: &Input_centralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#input_central}.
 * @param ctx the parse tree
 */
fn exit_input_central(&mut self, _ctx: &Input_centralContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#input_cluster}.
 * @param ctx the parse tree
 */
fn enter_input_cluster(&mut self, _ctx: &Input_clusterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#input_cluster}.
 * @param ctx the parse tree
 */
fn exit_input_cluster(&mut self, _ctx: &Input_clusterContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#input_path}.
 * @param ctx the parse tree
 */
fn enter_input_path(&mut self, _ctx: &Input_pathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#input_path}.
 * @param ctx the parse tree
 */
fn exit_input_path(&mut self, _ctx: &Input_pathContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#compute_direction}.
 * @param ctx the parse tree
 */
fn enter_compute_direction(&mut self, _ctx: &Compute_directionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#compute_direction}.
 * @param ctx the parse tree
 */
fn exit_compute_direction(&mut self, _ctx: &Compute_directionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#compute_target}.
 * @param ctx the parse tree
 */
fn enter_compute_target(&mut self, _ctx: &Compute_targetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#compute_target}.
 * @param ctx the parse tree
 */
fn exit_compute_target(&mut self, _ctx: &Compute_targetContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#compute_scope}.
 * @param ctx the parse tree
 */
fn enter_compute_scope(&mut self, _ctx: &Compute_scopeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#compute_scope}.
 * @param ctx the parse tree
 */
fn exit_compute_scope(&mut self, _ctx: &Compute_scopeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#compute_config}.
 * @param ctx the parse tree
 */
fn enter_compute_config(&mut self, _ctx: &Compute_configContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#compute_config}.
 * @param ctx the parse tree
 */
fn exit_compute_config(&mut self, _ctx: &Compute_configContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#compute_algorithm}.
 * @param ctx the parse tree
 */
fn enter_compute_algorithm(&mut self, _ctx: &Compute_algorithmContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#compute_algorithm}.
 * @param ctx the parse tree
 */
fn exit_compute_algorithm(&mut self, _ctx: &Compute_algorithmContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#compute_args}.
 * @param ctx the parse tree
 */
fn enter_compute_args(&mut self, _ctx: &Compute_argsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#compute_args}.
 * @param ctx the parse tree
 */
fn exit_compute_args(&mut self, _ctx: &Compute_argsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#compute_args_array}.
 * @param ctx the parse tree
 */
fn enter_compute_args_array(&mut self, _ctx: &Compute_args_arrayContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#compute_args_array}.
 * @param ctx the parse tree
 */
fn exit_compute_args_array(&mut self, _ctx: &Compute_args_arrayContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#compute_arg}.
 * @param ctx the parse tree
 */
fn enter_compute_arg(&mut self, _ctx: &Compute_argContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#compute_arg}.
 * @param ctx the parse tree
 */
fn exit_compute_arg(&mut self, _ctx: &Compute_argContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#type_any}.
 * @param ctx the parse tree
 */
fn enter_type_any(&mut self, _ctx: &Type_anyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#type_any}.
 * @param ctx the parse tree
 */
fn exit_type_any(&mut self, _ctx: &Type_anyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#type_scoped}.
 * @param ctx the parse tree
 */
fn enter_type_scoped(&mut self, _ctx: &Type_scopedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#type_scoped}.
 * @param ctx the parse tree
 */
fn exit_type_scoped(&mut self, _ctx: &Type_scopedContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#type_}.
 * @param ctx the parse tree
 */
fn enter_type_(&mut self, _ctx: &Type_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#type_}.
 * @param ctx the parse tree
 */
fn exit_type_(&mut self, _ctx: &Type_Context<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#label_any}.
 * @param ctx the parse tree
 */
fn enter_label_any(&mut self, _ctx: &Label_anyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#label_any}.
 * @param ctx the parse tree
 */
fn exit_label_any(&mut self, _ctx: &Label_anyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#label_scoped}.
 * @param ctx the parse tree
 */
fn enter_label_scoped(&mut self, _ctx: &Label_scopedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#label_scoped}.
 * @param ctx the parse tree
 */
fn exit_label_scoped(&mut self, _ctx: &Label_scopedContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#label}.
 * @param ctx the parse tree
 */
fn enter_label(&mut self, _ctx: &LabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#label}.
 * @param ctx the parse tree
 */
fn exit_label(&mut self, _ctx: &LabelContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#labels}.
 * @param ctx the parse tree
 */
fn enter_labels(&mut self, _ctx: &LabelsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#labels}.
 * @param ctx the parse tree
 */
fn exit_labels(&mut self, _ctx: &LabelsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#label_array}.
 * @param ctx the parse tree
 */
fn enter_label_array(&mut self, _ctx: &Label_arrayContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#label_array}.
 * @param ctx the parse tree
 */
fn exit_label_array(&mut self, _ctx: &Label_arrayContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#schema_native}.
 * @param ctx the parse tree
 */
fn enter_schema_native(&mut self, _ctx: &Schema_nativeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#schema_native}.
 * @param ctx the parse tree
 */
fn exit_schema_native(&mut self, _ctx: &Schema_nativeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#type_native}.
 * @param ctx the parse tree
 */
fn enter_type_native(&mut self, _ctx: &Type_nativeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#type_native}.
 * @param ctx the parse tree
 */
fn exit_type_native(&mut self, _ctx: &Type_nativeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#value_type}.
 * @param ctx the parse tree
 */
fn enter_value_type(&mut self, _ctx: &Value_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#value_type}.
 * @param ctx the parse tree
 */
fn exit_value_type(&mut self, _ctx: &Value_typeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#value}.
 * @param ctx the parse tree
 */
fn enter_value(&mut self, _ctx: &ValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#value}.
 * @param ctx the parse tree
 */
fn exit_value(&mut self, _ctx: &ValueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#regex}.
 * @param ctx the parse tree
 */
fn enter_regex(&mut self, _ctx: &RegexContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#regex}.
 * @param ctx the parse tree
 */
fn exit_regex(&mut self, _ctx: &RegexContext<'input>) { }

/**
 * Enter a parse tree produced by {@link TypeQLRustParser#unreserved}.
 * @param ctx the parse tree
 */
fn enter_unreserved(&mut self, _ctx: &UnreservedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TypeQLRustParser#unreserved}.
 * @param ctx the parse tree
 */
fn exit_unreserved(&mut self, _ctx: &UnreservedContext<'input>) { }

}
