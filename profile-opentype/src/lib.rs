#![deny(clippy::unwrap_used, clippy::expect_used)]
mod checks;
use fontspector_checkapi::{Profile, Registry};

pub struct OpenType;

impl fontspector_checkapi::Plugin for OpenType {
    fn register(&self, cr: &mut Registry) -> Result<(), String> {
        cr.register_check(checks::bold_italic_unique::bold_italic_unique);
        cr.register_check(checks::code_pages::code_pages);
        cr.register_check(checks::fvar::axis_ranges_correct);
        cr.register_check(checks::fvar::regular_coords_correct);
        cr.register_check(checks::fvar::varfont_foundry_defined_tag_name);
        cr.register_check(checks::fvar::varfont_valid_default_instance_nameids);
        cr.register_check(checks::fvar::varfont_valid_nameids);
        cr.register_check(checks::gdef::gdef_mark_chars);
        cr.register_check(checks::gdef::gdef_spacing_marks);
        cr.register_check(checks::glyf::check_glyf_non_transformed_duplicate_components);
        cr.register_check(checks::glyf::check_point_out_of_bounds);
        cr.register_check(checks::glyf::glyf_unused_data);
        cr.register_check(checks::head::equal_font_versions);
        cr.register_check(checks::head::font_version);
        cr.register_check(checks::head::mac_style);
        cr.register_check(checks::head::unitsperem);
        cr.register_check(checks::hhea::caret_slope);
        cr.register_check(checks::hhea::maxadvancewidth);
        cr.register_check(checks::layout::layout_valid_feature_tags);
        cr.register_check(checks::layout::layout_valid_language_tags);
        cr.register_check(checks::layout::layout_valid_script_tags);
        cr.register_check(checks::name::check_name_match_familyname_fullfont);
        cr.register_check(checks::name::check_name_no_copyright_on_description);
        cr.register_check(checks::name::consistent_family_name);
        cr.register_check(checks::name::family_max_4_fonts_per_family_name);
        cr.register_check(checks::name::family_naming_recommendations);
        cr.register_check(checks::name::name_empty_records);
        cr.register_check(checks::name::name_italic_names);
        cr.register_check(checks::name::postscript_name);
        cr.register_check(checks::os2::check_vendor_id);
        cr.register_check(checks::os2::fsselection);
        cr.register_check(checks::os2::panose_familytype);
        cr.register_check(checks::os2::xavgcharwidth);
        cr.register_check(checks::post::post_table_version);
        cr.register_check(checks::post::underline_thickness);
        cr.register_check(checks::stat::ital_axis);
        cr.register_check(checks::stat::stat_axis_record);
        cr.register_check(checks::stat::stat_has_axis_value_tables);
        cr.register_check(checks::stat::weight_class_fvar);

        let opentype_profile = Profile::from_toml(
            r#"
[sections]
"OpenType Specification Checks" = [
    # Checks which we have definitely ported already
    "opentype/fvar/regular_coords_correct",
    "opentype/maxadvancewidth",
    "opentype/caret_slope",
    "opentype/name/empty_records",
    "opentype/family/underline_thickness",
    "opentype/post_table_version",
    "opentype/varfont/stat_axis_record_for_each_axis",
    "opentype/family/bold_italic_unique_for_nameid1",
    "opentype/font_version",
    "opentype/mac_style",
    "opentype/family/equal_font_versions",
    "opentype/unitsperem",
    "opentype/fsselection",

    # Checks left to port
    "opentype/cff2_call_depth",
    "opentype/cff_ascii_strings",
    "opentype/cff_call_depth",
    "opentype/cff_deprecated_operators",
    "opentype/code_pages",
    "opentype/family/consistent_family_name",
    "opentype/family/max_4_fonts_per_family_name",
    "opentype/family_naming_recommendations",

    # Checks we don't need because they have been integrated into other checks
    # "opentype/dsig", (unwanted_tables)
    # "opentype/varfont/ital_range", (opentype/fvar/axis_ranges_correct)
    # "opentype/varfont/slnt_range",
    # "opentype/varfont/regular_ital_coord", (opentype/fvar/regular_coords_correct)
    # "opentype/varfont/regular_opsz_coord",
    # "opentype/varfont/regular_slnt_coord",
    # "opentype/varfont/regular_wdth_coord",
    # "opentype/varfont/regular_wght_coord",
    # "opentype/fsselection_matches_macstyle", (merged into opentype/fsselection)

    # Checks I haven't got around to classifying yet
    "opentype/family/panose_familytype",
    "opentype/fvar/axis_ranges_correct",
    "opentype/gdef_mark_chars",
    "opentype/gdef_non_mark_chars",
    "opentype/gdef_spacing_marks",
    "opentype/glyf_non_transformed_duplicate_components",
    "opentype/glyf_unused_data",
    "opentype/gpos_kerning_info",
    "opentype/italic_angle",
    "opentype/italic_axis_in_stat",
    "opentype/italic_axis_in_stat_is_boolean", # Merge into above
    "opentype/italic_axis_last", # Merge into above
    "opentype/kern_table",
    "opentype/layout_valid_feature_tags",
    "opentype/layout_valid_language_tags",
    "opentype/layout_valid_script_tags",
    "opentype/loca/maxp_num_glyphs",
    "opentype/monospace",
    "opentype/name/italic_names",
    "opentype/name/match_familyname_fullfont",
    "opentype/name/no_copyright_on_description",
    "opentype/name/postscript_name_consistency",
    "opentype/name/postscript_vs_cff",
    "opentype/points_out_of_bounds",
    "opentype/postscript_name",
    "opentype/slant_direction",
    "opentype/stat_has_axis_value_tables",
    "opentype/varfont/distinct_instance_records",
    "opentype/varfont/family_axis_ranges",
    "opentype/varfont/foundry_defined_tag_name",
    "opentype/varfont/same_size_instance_records",
    "opentype/varfont/valid_axis_nameid",
    "opentype/varfont/valid_default_instance_nameids",
    "opentype/varfont/valid_postscript_nameid",
    "opentype/varfont/valid_subfamily_nameid",
    "opentype/varfont/wdth_valid_range",
    "opentype/varfont/wght_valid_range",
    "opentype/vendor_id",
    "opentype/weight_class_fvar",
    "opentype/xavgcharwidth",
]
"#,
        )
        .map_err(|_| "Couldn't parse profile")?;
        cr.register_profile("opentype", opentype_profile)
    }
}
