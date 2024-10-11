use fontspector_checkapi::{constants::GlyphClass, prelude::*, testfont, FileTypeConvert};
use read_fonts::TableProvider;
use skrifa::MetadataProvider;
use unicode_properties::{GeneralCategory, UnicodeGeneralCategory};

fn is_nonspacing_mark(c: char) -> bool {
    matches!(
        c.general_category(),
        GeneralCategory::NonspacingMark | GeneralCategory::EnclosingMark
    )
}

#[check(
    id = "opentype/gdef_spacing_marks",
    rationale = "
        Glyphs in the GDEF mark glyph class should be non-spacing.

        Spacing glyphs in the GDEF mark glyph class may have incorrect anchor
        positioning that was only intended for building composite glyphs during design.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2877",
    title = "Check glyphs in mark glyph class are non-spacing."
)]
fn gdef_spacing_marks(f: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    let hmtx = font.font().hmtx()?;
    let gdef = font
        .font()
        .gdef()
        .map_err(|_| CheckError::skip("no-gdef", "GDEF table unreadable or not present"))?;
    let glyph_classdef = gdef.glyph_class_def().ok_or_else(|| {
        CheckError::skip("no-glyph-class-def", "GDEF table has no GlyphClassDef")
    })??;
    let nonspacing_mark_glyphs = bullet_list(
        context,
        glyph_classdef
            .iter()
            .filter(|(glyph, class)| *class == 3 && hmtx.advance(*glyph).unwrap_or(0) > 0)
            .map(|(glyph, _)| {
                #[allow(clippy::unwrap_used)] // synthesis=true means this is infallible
                font.glyph_name_for_id(glyph, true).unwrap()
            }),
    );
    if !nonspacing_mark_glyphs.is_empty() {
        return Ok(Status::just_one_fail("spacing-mark-glyphs", &format!(
            "The following glyphs seem to be spacing (because they have width > 0 on the hmtx table) so they may be in the GDEF mark glyph class by mistake, or they should have zero width instead:\n{}",
                nonspacing_mark_glyphs
        )));
    }

    Ok(Status::just_one_pass())
}

#[check(
    id = "opentype/gdef_mark_chars",
    rationale = "Mark characters should be in the GDEF mark glyph class.",
    proposal = "https://github.com/fonttools/fontbakery/issues/2877",
    title = "Check mark characters are in GDEF mark glyph class."
)]
fn gdef_mark_chars(f: &Testable, context: &Context) -> CheckFnResult {
    let font = testfont!(f);
    let mark_chars_not_in_gdef_mark = bullet_list(
        context,
        font.font()
            .charmap()
            .mappings()
            .filter(|(u, gid)| {
                char::from_u32(*u).map_or(false, is_nonspacing_mark)
                    && font.gdef_class(*gid) != Some(GlyphClass::Mark)
            })
            .map(|(u, gid)| {
                #[allow(clippy::unwrap_used)] // synthesis=true means this is infallible
                let name = font.glyph_name_for_id(gid, true).unwrap();
                format!("U+{:04X} ({})", u, name)
            }),
    );
    if !mark_chars_not_in_gdef_mark.is_empty() {
        return Ok(Status::just_one_fail(
            "mark-chars",
            &format!(
                "The following mark characters should be in the GDEF mark glyph class:\n{}",
                mark_chars_not_in_gdef_mark
            ),
        ));
    }

    Ok(Status::just_one_pass())
}