use serde_json::json;
use os_query_builder_rs::misc::rewrite::Rewrite;

#[test]
fn test_serialize() {
    assert_eq!(json!(Rewrite::ConstantScore), "constant_score");
    assert_eq!(json!(Rewrite::ScoringBoolean), "scoring_boolean");
    assert_eq!(json!(Rewrite::ConstantScoreBoolean), "constant_score_boolean");


    assert_eq!(json!(Rewrite::TopTerms(1)), "top_terms_1");
    assert_eq!(json!(Rewrite::TopTerms(22)), "top_terms_22");
    assert_eq!(json!(Rewrite::TopTerms(1000)), "top_terms_1000");
    assert_eq!(json!(Rewrite::TopTerms(50000)), "top_terms_50000");
    assert_eq!(json!(Rewrite::TopTerms(40000000)), "top_terms_40000000");


    assert_eq!(json!(Rewrite::TopTermsBoost(1)), "top_terms_boost_1");
    assert_eq!(json!(Rewrite::TopTermsBoost(22)), "top_terms_boost_22");
    assert_eq!(json!(Rewrite::TopTermsBoost(1000)), "top_terms_boost_1000");
    assert_eq!(json!(Rewrite::TopTermsBoost(50000)), "top_terms_boost_50000");
    assert_eq!(json!(Rewrite::TopTermsBoost(40000000)), "top_terms_boost_40000000");


    assert_eq!(json!(Rewrite::TopTermsBlendedFreqs(1)), "top_terms_blended_freqs_1");
    assert_eq!(json!(Rewrite::TopTermsBlendedFreqs(22)), "top_terms_blended_freqs_22");
    assert_eq!(json!(Rewrite::TopTermsBlendedFreqs(1000)), "top_terms_blended_freqs_1000");
    assert_eq!(json!(Rewrite::TopTermsBlendedFreqs(50000)), "top_terms_blended_freqs_50000");
    assert_eq!(json!(Rewrite::TopTermsBlendedFreqs(40000000)), "top_terms_blended_freqs_40000000");
}