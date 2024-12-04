pub mod arabic_high_hamza;
pub mod arabic_spacing_symbols;
pub mod base_has_width;
pub mod case_mapping;
pub mod cjk_chws_feature;
pub mod cjk_not_enough_glyphs;
pub mod cmap_format_12;
pub mod color_cpal_brightness;
pub mod colorfont_tables;
pub mod consistent_axes;
pub mod contour_count;
pub mod control_chars;
pub mod empty_glyph_on_gid1_for_colrv0;
pub mod empty_letters;
pub mod family_vertical_metrics;
pub mod family_win_ascent_and_descent;
pub mod file_size;
#[cfg(not(target_family = "wasm"))]
pub mod fontdata_namecheck;
#[cfg(not(target_family = "wasm"))]
pub mod freetype_rasterizer;
pub mod fvar_name_entries;
pub mod glyf_nested_components;
pub mod glyphnames;
pub mod glyphset;
pub mod gpos7;
pub mod hinting_impact;
pub mod integer_ppem_if_hinted;
pub mod interpolation_issues;
pub mod legacy_accents;
pub mod ligature_carets;
pub mod linegaps;
pub mod mandatory_avar_table;
pub mod mandatory_glyphs;
pub mod math_signs_width;
pub mod missing_small_caps_glyphs;
pub mod name_char_restrictions;
pub mod name_family_and_style_max;
pub mod name_italic_names;
pub mod name_no_copyright_on_description;
pub mod name_trailing_spaces;
pub mod no_mac_entries;
pub mod os2_metrics_match_hhea;
pub mod render_own_name;
pub mod required_tables;
pub mod sfnt_version;
pub mod smallcaps_before_ligatures;
pub mod smart_dropout;
pub mod soft_hyphen;
pub mod stat_in_statics;
pub mod stat_strings;
pub mod stylistic_sets;
pub mod tabular_kerning;
pub mod transformed_components;
pub mod typoascender_agrave;
pub mod typographic_family_name;
pub mod unique_glyphnames;
pub mod unreachable_glyphs;
pub mod unsupported_axes;
pub mod unwanted_aat_tables;
pub mod unwanted_tables;
pub mod varfont_instances_in_order;
pub mod vtt_volt_data;
pub mod whitespace_glyphs;
pub mod whitespace_ink;
pub mod whitespace_widths;
