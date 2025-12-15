# Text Styles Guide

Transform ordinary text into stunning Unicode typography. Text styles map ASCII characters to mathematically-defined Unicode symbols that render beautifully across platforms.

## Basic Syntax

```markdown
{{style-name}}Your text here{{/style-name}}
```

With separator for word spacing:
```markdown
{{mathbold:separator=dot}}SPACED WORDS{{/mathbold}}
```

---

## All Text Styles

### Bold Styles

Heavy, attention-grabbing typography.

| Style | Aliases | Supports | Output |
|-------|---------|----------|--------|
| `mathbold` | mb, bold | A-Z, a-z, 0-9 | 𝐁𝐨𝐥𝐝 𝐓𝐞𝐱𝐭 |
| `fullwidth` | fw, wide | A-Z, a-z, 0-9, symbols | Ｆｕｌｌ　Ｗｉｄｔｈ |
| `sans-serif-bold` | ssb, sans-bold | A-Z, a-z, 0-9 | 𝗦𝗮𝗻𝘀 𝗕𝗼𝗹𝗱 |
| `sans-serif-bold-italic` | ssbi, sans-bold-italic | A-Z, a-z | 𝙎𝙖𝙣𝙨 𝘽𝙤𝙡𝙙 𝙄𝙩𝙖𝙡𝙞𝙘 |

**Syntax:**
```markdown
{{mathbold}}ANNOUNCEMENT{{/mathbold}}
{{fullwidth}}WIDE HEADER{{/fullwidth}}
{{sans-serif-bold}}Modern Bold{{/sans-serif-bold}}
```

**Rendered:**

𝐀𝐍𝐍𝐎𝐔𝐍𝐂𝐄𝐌𝐄𝐍𝐓
ＷＩＤＥ ＨＥＡＤＥＲ
𝗠𝗼𝗱𝗲𝗿𝗻 𝗕𝗼𝗹𝗱

---

### Boxed & Enclosed Styles

Letters wrapped in geometric shapes. **Uppercase only** (except parenthesized and inverted).

| Style | Aliases | Output |
|-------|---------|--------|
| `negative-squared` | neg-sq, squared | 🅰🅱🅲 (white on black squares) |
| `negative-circled` | neg-circle, circled | 🅐🅑🅒 (white on black circles) |
| `squared-latin` | sq-latin, boxed | 🄰🄱🄲 (letters in boxes) |
| `circled-latin` | circled, circle | Ⓐⓑⓒ (letters in circles) |
| `parenthesized` | paren, parens | ⒜⒝⒞ (letters in parentheses) |
| `inverted` | upsidedown, flip, flipped | ɐqɔ (upside-down text) |

**Syntax:**
```markdown
{{negative-squared}}VIP{{/negative-squared}}
{{negative-circled}}NEW{{/negative-circled}}
{{squared-latin}}INFO{{/squared-latin}}
{{circled-latin}}ABC{{/circled-latin}}
{{parenthesized}}abc{{/parenthesized}}
{{inverted}}Hello{{/inverted}}
```

**Rendered:**

🆅🅸🅿 🅝🅔🅦 🄸🄽🄵🄾 ⒶⒷⒸ ⒜⒝⒞ Hǝllo

**Use case:** Status labels, badges, highlighted keywords, fun effects.

---

### Elegant & Stylistic

Flowing, decorative typography for artistic emphasis.

| Style | Aliases | Supports | Output |
|-------|---------|----------|--------|
| `small-caps` | sc, smallcaps | a-z (converts to small caps) | ꜱᴍᴀʟʟ ᴄᴀᴘꜱ |
| `italic` | it, slant | A-Z, a-z | 𝐼𝑡𝑎𝑙𝑖𝑐 |
| `bold-italic` | bi, bold-slant | A-Z, a-z | 𝑩𝒐𝒍𝒅 𝑰𝒕𝒂𝒍𝒊𝒄 |
| `script` | scr, cursive, calligraphic | A-Z, a-z | 𝒮𝒸𝓇𝒾𝓅𝓉 |
| `bold-script` | bscr, bold-cursive | A-Z, a-z | 𝓑𝓸𝓵𝓭 𝓢𝓬𝓻𝓲𝓹𝓽 |

**Syntax:**
```markdown
{{small-caps}}elegant header{{/small-caps}}
{{italic}}Emphasized text{{/italic}}
{{script}}Fancy Signature{{/script}}
{{bold-script}}Premium Edition{{/bold-script}}
```

**Rendered:**

ᴇʟᴇɢᴀɴᴛ ʜᴇᴀᴅᴇʀ
𝐸𝑚𝑝ℎ𝑎𝑠𝑖𝑧𝑒𝑑 𝑡𝑒𝑥𝑡
ℱ𝒶𝓃𝒸𝓎 𝒮𝒾ℊ𝓃𝒶𝓉𝓊𝓇ℯ
𝓟𝓻𝓮𝓶𝓲𝓾𝓶 𝓔𝓭𝓲𝓽𝓲𝓸𝓷

---

### Gothic & Blackletter

Medieval manuscript-style typography.

| Style | Aliases | Supports | Output |
|-------|---------|----------|--------|
| `fraktur` | fr, gothic, blackletter | A-Z, a-z | 𝔉𝔯𝔞𝔨𝔱𝔲𝔯 |
| `bold-fraktur` | bfr, bold-gothic | A-Z, a-z | 𝕭𝖔𝖑𝖉 𝕱𝖗𝖆𝖐𝖙𝖚𝖗 |

**Syntax:**
```markdown
{{fraktur}}Dark Fantasy{{/fraktur}}
{{bold-fraktur}}CHAPTER ONE{{/bold-fraktur}}
```

**Rendered:**

𝔇𝔞𝔯𝔨 𝔉𝔞𝔫𝔱𝔞𝔰𝔶
𝕮𝕳𝕬𝕻𝕿𝕰𝕽 𝕺𝕹𝕰

**Use case:** Fantasy themes, historical documents, dramatic headers.

---

### Technical & Mathematical

Clean, precise typography for technical content.

| Style | Aliases | Supports | Output |
|-------|---------|----------|--------|
| `monospace` | mono, code | A-Z, a-z, 0-9 | 𝙼𝚘𝚗𝚘𝚜𝚙𝚊𝚌𝚎 |
| `double-struck` | ds, outline, blackboard | A-Z, a-z, 0-9 | 𝔻𝕠𝕦𝕓𝕝𝕖 |
| `sans-serif` | ss, sans | A-Z, a-z, 0-9 | 𝖲𝖺𝗇𝗌 𝖲𝖾𝗋𝗂𝖿 |
| `sans-serif-italic` | ssi, sans-italic | A-Z, a-z | 𝘚𝘢𝘯𝘴 𝘐𝘵𝘢𝘭𝘪𝘤 |
| `subscript` | sub | a,e,h,i,k-p,r-u,x, 0-9 | H₂O |
| `superscript` | sup, super | a-z, 0-9 | x² |

**Syntax:**
```markdown
{{monospace}}CODE_BLOCK{{/monospace}}
{{double-struck}}MATH SET{{/double-struck}}
{{sans-serif}}Clean Design{{/sans-serif}}
{{subscript}}H2O{{/subscript}}
{{superscript}}x2{{/superscript}}
```

**Rendered:**

𝙲𝙾𝙳𝙴_𝙱𝙻𝙾𝙲𝙺
𝕄𝔸𝕋ℍ 𝕊𝔼𝕋
𝖢𝗅𝖾𝖺𝗇 𝖣𝖾𝗌𝗂𝗀𝗇
H₂O (water)
x² (squared)

**Use case:** Technical documentation, mathematical notation, code references, chemistry formulas.

---

## Separator Parameter

Add visual spacing between words with the `separator` parameter:

**Syntax:**
```markdown
{{mathbold:separator=dot}}HELLO WORLD{{/mathbold}}
{{fullwidth:separator=star}}GET STARTED{{/fullwidth}}
```

**Rendered:**

𝐇·𝐄·𝐋·𝐋·𝐎· ·𝐖·𝐎·𝐑·𝐋·𝐃
Ｇ★Ｅ★Ｔ★ ★Ｓ★Ｔ★Ａ★Ｒ★Ｔ★Ｅ★Ｄ

Available separators: `dot`, `star`, `bullet`, `diamond`, `arrow`, and more.

---

## Support Matrix

| Style | Uppercase | Lowercase | Numbers | Symbols |
|-------|-----------|-----------|---------|---------|
| mathbold | ✓ | ✓ | ✓ | ✗ |
| fullwidth | ✓ | ✓ | ✓ | ✓ |
| sans-serif-bold | ✓ | ✓ | ✓ | ✗ |
| sans-serif-bold-italic | ✓ | ✓ | ✗ | ✗ |
| negative-squared | ✓ | ✗ | ✗ | ✗ |
| negative-circled | ✓ | ✗ | ✗ | ✗ |
| squared-latin | ✓ | ✗ | ✗ | ✗ |
| circled-latin | ✓ | ✓ | ✗ | ✗ |
| small-caps | ✓* | ✓ | ✗ | ✗ |
| italic | ✓ | ✓ | ✗ | ✗ |
| bold-italic | ✓ | ✓ | ✗ | ✗ |
| script | ✓ | ✓ | ✗ | ✗ |
| bold-script | ✓ | ✓ | ✗ | ✗ |
| fraktur | ✓ | ✓ | ✗ | ✗ |
| bold-fraktur | ✓ | ✓ | ✗ | ✗ |
| monospace | ✓ | ✓ | ✓ | ✗ |
| double-struck | ✓ | ✓ | ✓ | ✗ |
| sans-serif | ✓ | ✓ | ✓ | ✗ |
| sans-serif-italic | ✓ | ✓ | ✗ | ✗ |
| subscript | ✗ | ✓* | ✓ | ✗ |
| superscript | ✗ | ✓ | ✓ | ✗ |
| parenthesized | ✗ | ✓ | ✓ | ✗ |
| inverted | ✓ | ✓ | ✓ | ✗ |

*small-caps converts uppercase to small caps as well
*subscript only supports: a, e, h, i, k-p, r-u, x

**Note:** Unsupported characters pass through unchanged.

---

## Combining with Frames

Text styles pair beautifully with frames:

**Syntax:**
```markdown
{{frame:gradient}}{{mathbold}}ANNOUNCEMENT{{/mathbold}}{{/frame}}
{{frame:star}}{{fraktur}}Featured{{/fraktur}}{{/frame}}
{{frame:lenticular}}{{fullwidth}}TITLE{{/fullwidth}}{{/frame}}
```

**Rendered:**

▓▒░ 𝐀𝐍𝐍𝐎𝐔𝐍𝐂𝐄𝐌𝐄𝐍𝐓 ░▒▓
★ 𝔉𝔢𝔞𝔱𝔲𝔯𝔢𝔡 ☆
【ＴＩＴＬＥ】

---

## Practical Examples

### README Header

**Syntax:**
```markdown
{{frame:gradient}}{{mathbold:separator=dot}}PROJECT NAME{{/mathbold}}{{/frame}}
```

**Rendered:**

▓▒░ 𝐏·𝐑·𝐎·𝐉·𝐄·𝐂·𝐓· ·𝐍·𝐀·𝐌·𝐄 ░▒▓

### Gothic Chapter Title

**Syntax:**
```markdown
{{frame:line-double}}{{bold-fraktur}}CHAPTER I{{/bold-fraktur}}{{/frame}}
```

**Rendered:**

═══ 𝕮𝕳𝕬𝕻𝕿𝕰𝕽 𝕴 ═══

### Tech Badge Row

**Syntax:**
```markdown
{{negative-squared}}API{{/negative-squared}} {{negative-circled}}V2{{/negative-circled}}
```

**Rendered:**

🅰🅿🅸 🅥2

### Elegant Signature

**Syntax:**
```markdown
{{frame:heavy-quote}}{{script}}Best regards{{/script}}{{/frame}}
```

**Rendered:**

❝ℬℯ𝓈𝓉 𝓇ℯℊ𝒶𝓇𝒹𝓈❞

### Mathematical Notation

**Syntax:**
```markdown
Let {{double-struck}}R{{/double-struck}} be the set of real numbers
```

**Rendered:**

Let ℝ be the set of real numbers

### Status Labels

**Syntax:**
```markdown
{{negative-squared}}NEW{{/negative-squared}} Feature release
{{squared-latin}}BETA{{/squared-latin}} Testing phase
```

**Rendered:**

🅽🅴🆆 Feature release
🄱🄴🅃🄰 Testing phase

---

## Style Categories

| Category | Styles | Best For |
|----------|--------|----------|
| **Bold** | mathbold, fullwidth, sans-serif-bold | Headers, announcements |
| **Boxed** | negative-squared, negative-circled, squared-latin, parenthesized, inverted | Labels, badges, fun effects |
| **Elegant** | small-caps, italic, script, bold-script | Signatures, quotes |
| **Gothic** | fraktur, bold-fraktur | Fantasy, historical |
| **Technical** | monospace, double-struck, sans-serif, subscript, superscript | Code, math, chemistry |

---

## Unicode Blocks Reference

| Style | Unicode Range |
|-------|---------------|
| mathbold | U+1D400–U+1D7FF |
| fullwidth | U+FF00–U+FFEF |
| negative-squared | U+1F130–U+1F189 |
| negative-circled | U+1F150–U+1F169 |
| squared-latin | U+1F130–U+1F149 |
| circled-latin | U+24B6–U+24E9 |
| small-caps | U+1D00–U+1D7F |
| italic | U+1D434–U+1D467 |
| bold-italic | U+1D468–U+1D49B |
| script | U+1D49C–U+1D4B5 |
| bold-script | U+1D4D0–U+1D4E9 |
| fraktur | U+1D504–U+1D51C |
| bold-fraktur | U+1D56C–U+1D585 |
| monospace | U+1D670–U+1D6A3 |
| double-struck | U+1D538–U+1D56B |
| sans-serif | U+1D5A0–U+1D5B9 |
| sans-serif-italic | U+1D608–U+1D621 |
| subscript | U+2080–U+209C |
| superscript | U+2070–U+207F |
| parenthesized | U+2474–U+249B |
| inverted | Various (rotated glyphs) |

---

## Tips

1. **Test your audience** - Some fonts don't render all Unicode blocks
2. **Use sparingly** - Too many styles make text hard to read
3. **Match the tone** - Gothic for fantasy, sans-serif for modern
4. **Check support** - Boxed styles only work with uppercase
5. **Combine wisely** - One style + one frame usually looks best
6. **Accessibility** - Screen readers may struggle with Unicode text

---

<p align="center">
ʀᴇɴᴅᴇʀᴇᴅ ᴡɪᴛʜ ᴍᴅꜰx
</p>
