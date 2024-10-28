use crate::compile::rewrite::{
    cube_scan_wrapper, distinct, rewrite, rewriter::CubeRewrite, rules::wrapper::WrapperRules,
    wrapped_select, wrapper_pullup_replacer,
};

impl WrapperRules {
    pub fn distinct_rules(&self, rules: &mut Vec<CubeRewrite>) {
        rules.extend(vec![rewrite(
            "wrapper-push-down-distinct-to-cube-scan",
            distinct(cube_scan_wrapper(
                wrapper_pullup_replacer(
                    wrapped_select(
                        "?select_type",
                        "?projection_expr",
                        "?subqueries",
                        "?group_expr",
                        "?aggr_expr",
                        "?window_expr",
                        "?cube_scan_input",
                        "?joins",
                        "?filter_expr",
                        "?having_expr",
                        "?limit",
                        "?offset",
                        "?order_expr",
                        "?select_alias",
                        "?select_distinct",
                        "WrappedSelectPushToCube:false",
                        "?select_ungrouped_scan",
                    ),
                    "?alias_to_cube",
                    "?push_to_cube",
                    "?ungrouped_scan",
                    "?in_projection",
                    "?cube_members",
                ),
                "CubeScanWrapperFinalized:false".to_string(),
            )),
            cube_scan_wrapper(
                wrapper_pullup_replacer(
                    wrapped_select(
                        "?select_type",
                        "?projection_expr",
                        "?subqueries",
                        "?group_expr",
                        "?aggr_expr",
                        "?window_expr",
                        "?cube_scan_input",
                        "?joins",
                        "?filter_expr",
                        "?having_expr",
                        "?limit",
                        "?offset",
                        "?order_expr",
                        "?select_alias",
                        "WrappedSelectDistinct:true",
                        "WrappedSelectPushToCube:false",
                        // TODO why is it passthrough? distinct is a kind of aggregation
                        "?select_ungrouped_scan",
                    ),
                    "?alias_to_cube",
                    "?push_to_cube",
                    // TODO why is it passthrough? distinct is a kind of aggregation. this field just reflects ?select_ungrouped_scan
                    "?ungrouped_scan",
                    "?in_projection",
                    "?cube_members",
                ),
                "CubeScanWrapperFinalized:false",
            ),
        )])
    }
}
