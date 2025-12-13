# Overlay Combining Marks Test

This file tests whether combining diacritical marks work with styled Unicode characters.

## Strikethrough (U+0336)

Regular: NORMAL̶ ̶TEXT̶
Mathbold: 𝐌̶𝐀̶𝐓̶𝐇̶𝐁̶𝐎̶𝐋̶𝐃̶
Script: 𝓈̶𝒸̶𝓇̶𝒾̶𝓅̶𝓉̶
Negative-squared: 🅽̶🅴̶🅶̶🅰̶🆃̶🅸̶🆅̶🅴̶

## Underline (U+0332)

Regular: NORMAL̲ ̲TEXT̲
Mathbold: 𝐌̲𝐀̲𝐓̲𝐇̲𝐁̲𝐎̲𝐋̲𝐃̲
Script: 𝓈̲𝒸̲𝓇̲𝒾̲𝓅̲𝓉̲
Negative-squared: 🅽̲🅴̲🅶̲🅰̲🆃̲🅸̲🆅̲🅴̲

## Overline (U+0305)

Regular: NORMAL̅ ̅TEXT̅
Mathbold: 𝐌̅𝐀̅𝐓̅𝐇̅𝐁̅𝐎̅𝐋̅𝐃̅
Script: 𝓈̅𝒸̅𝓇̅𝒾̅𝓅̅𝓉̅
Negative-squared: 🅽̅🅴̅🅶̅🅰̅🆃̅🅸̅🆅̅🅴̅

## Double Underline (U+0333)

Regular: DOUBLE̳ ̳LINE̳
Mathbold: 𝐃̳𝐎̳𝐔̳𝐁̳𝐋̳𝐄̳
Script: 𝒹̳ℴ̳𝓊̳𝒷̳𝓁ℯ̳

## Tilde Overlay (U+0334)

Regular: TILDE̴ ̴OVER̴
Mathbold: 𝐓̴𝐈̴𝐋̴𝐃̴𝐄̴
Script: 𝓉̴𝒾̴𝓁̴𝒹̴ℯ̴

---

**How to check:** View this file on:
1. Your terminal (should work)
2. GitHub web interface (might strip marks)
3. VS Code (should work)
4. Mobile browser (probably broken)

If you see the lines/marks over the styled characters, overlays are supported in your environment.
