initSidebarItems({"fn":[["extents_to_pixels","Converts extents from Pango units to device units, dividing by the `PANGO_SCALE` factor and performing rounding."],["find_base_dir","Searches a string the first character that has a strong direction, according to the Unicode bidirectional algorithm."],["find_paragraph_boundary","Locates a paragraph boundary in `text`. A boundary is caused by delimiter characters, such as a newline, carriage return, carriage return-newline pair, or Unicode paragraph separator character. The index of the run of delimiters is returned in `paragraph_delimiter_index`. The index of the start of the paragraph (index after all delimiters) is stored in `next_paragraph_start`."],["is_zero_width","Checks `ch` to see if it is a character that should not be normally rendered on the screen. This includes all Unicode characters with “ZERO WIDTH” in their name, as well as `<firstterm>`bidi`</firstterm>` formatting characters, and a few other ones. This is totally different from `g_unichar_iszerowidth()` and is at best misnamed."],["itemize","Breaks a piece of text into segments with consistent directional level and shaping engine. Each byte of `text` will be contained in exactly one of the items in the returned list; the generated list of items will be in logical order (the start offsets of the items are ascending)."],["itemize_with_base_dir","Like `pango_itemize()`, but the base direction to use when computing bidirectional levels (see pango_context_set_base_dir ()), is specified explicitly rather than gotten from the [`crate::Context`]."],["parse_markup","Parses marked-up text (see"],["parse_stretch","Parses a font stretch. The allowed values are “ultra_condensed”, “extra_condensed”, “condensed”, “semi_condensed”, “normal”, “semi_expanded”, “expanded”, “extra_expanded” and “ultra_expanded”. Case variations are ignored and the ‘_’ characters may be omitted."],["parse_style","Parses a font style. The allowed values are “normal”, “italic” and “oblique”, case variations being ignored."],["parse_variant","Parses a font variant. The allowed values are “normal” and “smallcaps” or “small_caps”, case variations being ignored."],["parse_weight","Parses a font weight. The allowed values are “heavy”, “ultrabold”, “bold”, “normal”, “light”, “ultraleight” and integers. Case variations are ignored."],["quantize_line_geometry","Quantizes the thickness and position of a line, typically an underline or strikethrough, to whole device pixels, that is integer multiples of `PANGO_SCALE`. The purpose of this function is to avoid such lines looking blurry."],["shape","Given a segment of text and the corresponding [`crate::Analysis`] structure returned from `pango_itemize()`, convert the characters into glyphs. You may also pass in only a substring of the item from `pango_itemize()`."],["unichar_direction","Determines the inherent direction of a character; either [`crate::Direction::Ltr`], [`crate::Direction::Rtl`], or [`crate::Direction::Neutral`]."],["units_from_double","Converts a floating-point number to Pango units: multiplies it by `PANGO_SCALE` and rounds to nearest integer."],["units_to_double","Converts a number in Pango units to floating-point: divides it by `PANGO_SCALE`."],["version","This is similar to the macro `PANGO_VERSION` except that it returns the encoded version of Pango available at run-time, as opposed to the version available at compile-time."],["version_check","Checks that the Pango library in use is compatible with the given version. Generally you would pass in the constants `PANGO_VERSION_MAJOR`, `PANGO_VERSION_MINOR`, `PANGO_VERSION_MICRO` as the three arguments to this function; that produces a check that the library in use at run-time is compatible with the version of Pango the application or module was compiled against."],["version_string","This is similar to the macro `PANGO_VERSION_STRING` except that it returns the version of Pango available at run-time, as opposed to the version available at compile-time."]]});