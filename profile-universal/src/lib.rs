#![deny(clippy::unwrap_used, clippy::expect_used)]
mod checks;
use fontspector_checkapi::{Profile, Registry};

pub struct Universal;

impl fontspector_checkapi::Plugin for Universal {
    fn register(&self, cr: &mut Registry) -> Result<(), String> {
        cr.register_check(checks::arabic_spacing_symbols::arabic_spacing_symbols);
        cr.register_check(checks::case_mapping::case_mapping);
        cr.register_check(checks::cmap_format_12::cmap_format_12);
        cr.register_check(checks::colorfont_tables::colorfont_tables);
        cr.register_check(checks::color_cpal_brightness::color_cpal_brightness);
        cr.register_check(checks::consistent_axes::consistent_axes);
        cr.register_check(checks::control_chars::control_chars);
        cr.register_check(checks::fvar_name_entries::fvar_name_entries);
        cr.register_check(checks::glyf_nested_components::glyf_nested_components);
        cr.register_check(checks::glyphnames::valid_glyphnames);
        cr.register_check(checks::glyphset::check_rupee);
        cr.register_check(checks::integer_ppem_if_hinted::integer_ppem_if_hinted);
        cr.register_check(checks::interpolation_issues::interpolation_issues);
        cr.register_check(checks::linegaps::linegaps);
        cr.register_check(checks::name_italic_names::name_italic_names);
        cr.register_check(
            checks::name_no_copyright_on_description::name_no_copyright_on_description,
        );
        cr.register_check(checks::mandatory_avar_table::mandatory_avar_table);
        cr.register_check(checks::name_char_restrictions::name_char_restrictions);
        cr.register_check(checks::name_trailing_spaces::name_trailing_spaces);
        cr.register_check(checks::no_mac_entries::no_mac_entries);
        cr.register_check(checks::os2_metrics_match_hhea::os2_metrics_match_hhea);
        cr.register_check(checks::render_own_name::render_own_name);
        cr.register_check(checks::required_tables::required_tables);
        cr.register_check(checks::sfnt_version::sfnt_version);
        cr.register_check(checks::soft_hyphen::soft_hyphen);
        cr.register_check(checks::stylistic_sets::stylisticset_description);
        cr.register_check(checks::transformed_components::transformed_components);
        cr.register_check(checks::typographic_family_name::typographic_family_name);
        cr.register_check(checks::unique_glyphnames::unique_glyphnames);
        cr.register_check(checks::unsupported_axes::unsupported_axes);
        cr.register_check(checks::unwanted_aat_tables::unwanted_aat_tables);
        cr.register_check(checks::unwanted_tables::unwanted_tables);
        cr.register_check(checks::whitespace_ink::whitespace_ink);
        cr.register_check(checks::whitespace_glyphs::whitespace_glyphs);
        cr.register_check(checks::whitespace_widths::whitespace_widths);

        let universal_profile = Profile::from_toml(
            r#"
include_profiles = ["opentype"]
[sections]
"Superfamily Checks" = [
    "superfamily/list",
    "superfamily/vertical_metrics",
]
"UFO Sources" = [
    "designspace_has_consistent_codepoints",
    "designspace_has_consistent_glyphset",
    "designspace_has_consistent_groups",
    "designspace_has_default_master",
    "designspace_has_sources",
    "ufolint",
    # "ufo_consistent_curve_type",  # FIXME (orphan check) https://github.com/fonttools/fontbakery/pull/4809
    "ufo_features_default_languagesystem",
    # "ufo_no_open_corners",  # FIXME (orphan check) https://github.com/fonttools/fontbakery/pull/4809
    "ufo_recommended_fields",
    "ufo_required_fields",
    "ufo_unnecessary_fields",
]
"Universal Profile Checks" = [
    # Checks which we have definitely ported already
    "arabic_spacing_symbols",
    "case_mapping",
    "cmap/format_12",
    "color_cpal_brightness",
    "colorfont_tables",
    "control_chars",
    "fvar_name_entries",
    "glyf_nested_components",
    "integer_ppem_if_hinted",
    "interpolation_issues",
    "linegaps",
    "valid_glyphnames",
    "mandatory_avar_table",
    "name/char_restrictions",
    "name/italic_names",
    "name/no_copyright_on_description",
    "name/trailing_spaces",
    "no_mac_entries",
    "os2_metrics_match_hhea",
    "render_own_name",
    "required_tables",
    "rupee",
    "sfnt_version",
    "soft_hyphen",
    "stylisticset_description",
    "transformed_components",
    "typographic_family_name",
    "unique_glyphnames",
    "unwanted_aat_tables",
    "unwanted_tables",
    "varfont/consistent_axes",
    "varfont/unsupported_axes",
    "whitespace_glyphs",
    "whitespace_ink",
    "whitespace_widths",

    # Checks which don't make sense any more
    # "family/single_directory", # Fontspector assumes families are in a directory

    # Checks left to port
    "alt_caron",
    "arabic_high_hamza",
    # "caps_vertically_centered",  # Disabled: issue #4274
    "cjk_chws_feature",
    "cjk_not_enough_glyphs",
    "contour_count",
    "empty_glyph_on_gid1_for_colrv0",
    "empty_letters",
    "family/vertical_metrics",
    "family/win_ascent_and_descent",
    "file_size",
    "fontspector_version",
    "fontdata_namecheck",
    "freetype_rasterizer",
    "gpos7",
    "gsub/smallcaps_before_ligatures",
    "hinting_impact",
    "inconsistencies_between_fvar_stat",
    "legacy_accents",
    "ligature_carets",
    "kerning_for_non_ligated_sequences",
    "mandatory_glyphs",
    "math_signs_width",
    "missing_small_caps_glyphs",
    "name/family_and_style_max_length",
    "no_debugging_tables",
    "ots",
    "smart_dropout",
    "STAT_in_statics",
    "STAT_strings",
    "tabular_kerning",
    "ttx_roundtrip",
    "typoascender_exceeds_Agrave",
    "unreachable_glyphs",
    "varfont/duplexed_axis_reflow",
    "varfont/instances_in_order",
    "vtt_volt_data",  # very similar to vttclean, may be a good idea to merge them.
    "vttclean",
]
"#,
        )
        .map_err(|_| "Couldn't parse profile")?;
        cr.register_profile("universal", universal_profile)
    }
}
