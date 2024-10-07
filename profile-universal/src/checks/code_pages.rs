use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};
use read_fonts::TableProvider;

#[check(
    id = "opentype/code_pages",
    title = "Check code page character ranges",
    rationale = "
        At least some programs (such as Word and Sublime Text) under Windows 7
        do not recognize fonts unless code page bits are properly set on the
        ulCodePageRange1 (and/or ulCodePageRange2) fields of the OS/2 table.

        More specifically, the fonts are selectable in the font menu, but whichever
        Windows API these applications use considers them unsuitable for any
        character set, so anything set in these fonts is rendered with Arial as a
        fallback font.

        This check currently does not identify which code pages should be set.
        Auto-detecting coverage is not trivial since the OpenType specification
        leaves the interpretation of whether a given code page is \"functional\"
        or not open to the font developer to decide.

        So here we simply detect as a FAIL when a given font has no code page
        declared at all.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/2474"
)]
fn code_pages(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let os2 = f.font().os2()?;
    let cpr1 = os2.ul_code_page_range_1();
    let cpr2 = os2.ul_code_page_range_2();
    if cpr1.is_none() || cpr2.is_none() || ((cpr1 == Some(0)) && (cpr2 == Some(0))) {
        Ok(Status::just_one_fail(
            "no-code-pages",
            "No code pages defined in the OS/2 table ulCodePageRange1 \
             and CodePageRange2 fields.",
        ))
    } else {
        Ok(Status::just_one_pass())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use fontspector_checkapi::{
//        StatusCode,
        TEST_FILE,
        codetesting::{
            assert_pass,
//            assert_results_contain,
            run_check,
        }
    };

    #[test]
    fn pass_opentype_code_pages() {
        let font: Testable = TEST_FILE!("merriweather/Merriweather-Regular.ttf");
        assert_pass(run_check(super::code_pages, font));
    }

/*
    #[test]
    fn fail_with_no_code_page_declared() {
        let mut font: Testable = TEST_FILE!("merriweather/Merriweather-Regular.ttf");

//      ttFont["OS/2"].ulCodePageRange1 = 0  # remove all code pages to make the check FAIL
//      ttFont["OS/2"].ulCodePageRange2 = 0

        assert_results_contain(
            run_check(super::code_pages, font),
            StatusCode::Fail, Some("no-code-pages".to_string()));
    }
*/
}
