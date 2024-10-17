use crate::compile::rewrite::{
    cube_scan_wrapper, rewrite, rewriter::CubeRewrite, rules::wrapper::WrapperRules,
    wrapped_select, wrapped_select_aggr_expr_empty_tail, wrapped_select_filter_expr,
    wrapped_select_filter_expr_empty_tail, wrapped_select_group_expr_empty_tail,
    wrapped_select_having_expr_empty_tail, wrapped_select_joins_empty_tail,
    wrapped_select_order_expr_empty_tail, wrapped_select_projection_expr_empty_tail,
    wrapped_select_subqueries_empty_tail, wrapped_select_window_expr_empty_tail,
    wrapper_pullup_replacer,
};

impl WrapperRules {
    pub fn merge_rules(&self, rules: &mut Vec<CubeRewrite>) {
        rules.extend(vec![rewrite(
            "wrapper-pull-up-inner-wrapper-with-filter",
            cube_scan_wrapper(
                wrapped_select(
                    "?select_type",
                    wrapper_pullup_replacer(
                        "?projection_expr",
                        "?alias_to_cube",
                        "?ungrouped",
                        "?in_projection",
                        "?cube_members",
                    ),
                    wrapper_pullup_replacer(
                        "?subqueries",
                        "?alias_to_cube",
                        "?ungrouped",
                        "?in_projection",
                        "?cube_members",
                    ),
                    wrapper_pullup_replacer(
                        "?group_expr",
                        "?alias_to_cube",
                        "?ungrouped",
                        "?in_projection",
                        "?cube_members",
                    ),
                    wrapper_pullup_replacer(
                        "?aggr_expr",
                        "?alias_to_cube",
                        "?ungrouped",
                        "?in_projection",
                        "?cube_members",
                    ),
                    wrapper_pullup_replacer(
                        "?window_expr",
                        "?alias_to_cube",
                        "?ungrouped",
                        "?in_projection",
                        "?cube_members",
                    ),
                    wrapper_pullup_replacer(
                        wrapped_select(
                            "WrappedSelectSelectType:Projection",
                            wrapped_select_projection_expr_empty_tail(),
                            wrapped_select_subqueries_empty_tail(),
                            wrapped_select_group_expr_empty_tail(),
                            wrapped_select_aggr_expr_empty_tail(),
                            wrapped_select_window_expr_empty_tail(),
                            "?inner_cube_scan_input",
                            wrapped_select_joins_empty_tail(),
                            wrapped_select_filter_expr(
                                "?inner_filter_expr_left",
                                "?inner_filter_expr_right",
                            ),
                            wrapped_select_having_expr_empty_tail(),
                            "WrappedSelectLimit:None",
                            "WrappedSelectOffset:None",
                            wrapped_select_order_expr_empty_tail(),
                            "WrappedSelectAlias:None",
                            "WrappedSelectDistinct:false",
                            "WrappedSelectUngrouped:true",
                            "WrappedSelectUngroupedScan:true",
                        ),
                        "?alias_to_cube",
                        "?ungrouped",
                        "?in_projection",
                        "?cube_members",
                    ),
                    wrapped_select_joins_empty_tail(),
                    wrapper_pullup_replacer(
                        wrapped_select_filter_expr_empty_tail(),
                        "?alias_to_cube",
                        "?ungrouped",
                        "?in_projection",
                        "?cube_members",
                    ),
                    wrapped_select_having_expr_empty_tail(),
                    "?limit",
                    "?offset",
                    wrapper_pullup_replacer(
                        "?order_expr",
                        "?alias_to_cube",
                        "?ungrouped",
                        "?in_projection",
                        "?cube_members",
                    ),
                    "?select_alias",
                    "?select_distinct",
                    "?select_ungrouped",
                    "?select_ungrouped_scan",
                ),
                "CubeScanWrapperFinalized:false",
            ),
            cube_scan_wrapper(
                wrapper_pullup_replacer(
                    wrapped_select(
                        "?select_type",
                        "?projection_expr",
                        "?subqueries",
                        "?group_expr",
                        "?aggr_expr",
                        "?window_expr",
                        "?inner_cube_scan_input",
                        wrapped_select_joins_empty_tail(),
                        wrapped_select_filter_expr(
                            "?inner_filter_expr_left",
                            "?inner_filter_expr_right",
                        ),
                        wrapped_select_having_expr_empty_tail(),
                        "?limit",
                        "?offset",
                        "?order_expr",
                        "?select_alias",
                        "?select_distinct",
                        // this should inherit lower instead of higher
                        // sematics is that "input of WS in ungrouped", and we use input from lower
                        // lower WrappedSelect is simple, but if it's ungrouped, then
                        // "?select_ungrouped",
                        "WrappedSelectUngrouped:true",
                        "?select_ungrouped_scan",
                    ),
                    "?alias_to_cube",
                    // TODO why fixed false? is
                    // "WrapperPullupReplacerUngrouped:false",
                    // "WrapperPullupReplacerUngrouped:true",
                    "?ungrouped",
                    "?in_projection",
                    "?cube_members",
                ),
                "CubeScanWrapperFinalized:false",
            ),
        )]);
    }
}
