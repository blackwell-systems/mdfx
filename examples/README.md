# utf8fx Examples

This directory contains demonstration files showing utf8fx capabilities.

## Files

### demo-input.md
Markdown file with template syntax showing all 11 Unicode styles.

### demo-output.md
The processed result with Unicode characters applied.

## Comparison

**Before (demo-input.md):**
```markdown
# {{mathbold}}UNICODE STYLING DEMO{{/mathbold}}
```

**After (demo-output.md):**
```markdown
# 𝐔𝐍𝐈𝐂𝐎𝐃𝐄 𝐒𝐓𝐘𝐋𝐈𝐍𝐆 𝐃𝐄𝐌𝐎
```

## Try It Yourself

```bash
# View original
cat examples/demo-input.md

# Process and view result
utf8fx process examples/demo-input.md

# Compare with saved output
diff <(utf8fx process examples/demo-input.md) examples/demo-output.md
```

## Template Syntax

```markdown
{{style}}content{{/style}}

# With spacing parameter
{{style:spacing=N}}content{{/style}}

# With separator parameter
{{style:separator=name}}content{{/style}}

# With decorative frame
{{frame:style}}content{{/frame}}

# Composition (style + separator + frame)
{{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}
```

### Spacing Parameter

Add spaces between each character for artistic headers and design elements:

```markdown
# {{mathbold:spacing=1}}H E A D E R{{/mathbold}}
→ # 𝐇 𝐄 𝐀 𝐃 𝐄 𝐑

## {{script:spacing=2}}E l e g a n t{{/script}}
→ ## ℰ   𝓁   ℯ   ℊ   𝒶   𝓃   𝓉

{{negative-squared:spacing=1}}WARNING{{/negative-squared}}
→ 🆆 🅰 🆁 🅽 🅸 🅽 🅶

{{fraktur:spacing=3}}Gothic{{/fraktur}}
→ 𝔊   𝔬   𝔱   𝔥   𝔦   𝔠
```

### Separator Parameter

Use custom separator characters between letters:

```markdown
{{mathbold:separator=dot}}T I T L E{{/mathbold}}
→ 𝐓·𝐈·𝐓·𝐋·𝐄

{{mathbold:separator=dash}}H E A D E R{{/mathbold}}
→ 𝐇─𝐄─𝐀─𝐃─𝐄─𝐑

{{mathbold:separator=bolddash}}B O L D{{/mathbold}}
→ 𝐁━𝐎━𝐋━𝐃

{{mathbold:separator=arrow}}F L O W{{/mathbold}}
→ 𝐅→𝐋→𝐎→𝐖

{{script:separator=bullet}}Note{{/script}}
→ 𝒩•𝑜•𝓉•ℯ
```

**Available separators:**
- `dot` (·) - Middle dot
- `bullet` (•) - Bullet point
- `dash` (─) - Box drawing horizontal
- `bolddash` (━) - Box drawing heavy
- `arrow` (→) - Rightward arrow

### Badges

Enclose numbers (0-20) or letters (a-z) with pre-composed Unicode characters:

```markdown
Step {{badge:circle}}1{{/badge}}: Install
→ Step ①: Install

Priority {{badge:negative-circle}}1{{/badge}} task
→ Priority ❶ task

Option {{badge:paren-letter}}a{{/badge}}: Accept
→ Option ⒜: Accept

Section {{badge:paren}}3{{/badge}} complete
→ Section ⑶ complete
```

**Available badge types:**
- `circle` (①②③) - Circled numbers 0-20
- `negative-circle` (❶❷❸) - White on black circles 0-20
- `double-circle` (⓵⓶⓷) - Double circles 1-10
- `paren` (⑴⑵⑶) - Parenthesized numbers 1-20
- `period` (🄁🄂🄃) - Period-terminated numbers 0-20
- `paren-letter` (⒜⒝⒞) - Parenthesized letters a-z

**Note:** Badges have limited charset support. Using unsupported characters (like "99" or uppercase) will return an error.

### Decorative Frames

Wrap text with decorative elements:

```markdown
{{frame:gradient}}Important Note{{/frame}}
→ ▓▒░ Important Note ░▒▓

{{frame:solid-left}}Action Item{{/frame}}
→ █▌Action Item

{{frame:line-bold}}Section Header{{/frame}}
→ ━━━ Section Header ━━━

{{frame:arrow-right}}Next Step{{/frame}}
→ → Next Step →
```

**Available frames:**
- `gradient` - Gradient blocks (▓▒░ ... ░▒▓)
- `solid-left` - Left solid block (█▌...)
- `solid-right` - Right solid block (...▐█)
- `solid-both` - Both sides (█▌...▐█)
- `line-light` - Light lines (─── ... ───)
- `line-bold` - Bold lines (━━━ ... ━━━)
- `line-double` - Double lines (═══ ... ═══)
- `line-dashed` - Dashed lines (╌╌╌ ... ╌╌╌)
- `block-top` - Top blocks (▀▀▀ ... ▀▀▀)
- `block-bottom` - Bottom blocks (▄▄▄ ... ▄▄▄)
- `arrow-right` - Arrows (→ ... →)
- `dot` - Middle dots (· ... ·)
- `bullet` - Bullet points (• ... •)

### Composition Examples

Combine styles, separators, and frames:

```markdown
# Styled + Framed
{{frame:gradient}}{{mathbold}}TITLE{{/mathbold}}{{/frame}}
→ ▓▒░ 𝐓𝐈𝐓𝐋𝐄 ░▒▓

# Styled + Separator + Framed
{{frame:solid-left}}{{mathbold:separator=dash}}HEADER{{/mathbold}}{{/frame}}
→ █▌𝐇─𝐄─𝐀─𝐃─𝐄─𝐑

# Multiple styles in one frame
{{frame:gradient}}{{mathbold}}Bold{{/mathbold}} and {{italic}}Italic{{/italic}}{{/frame}}
→ ▓▒░ 𝐁𝐨𝐥𝐝 and 𝐼𝑡𝑎𝑙𝑖𝑐 ░▒▓
```

**Available styles:**
- `mathbold` (alias: `mb`)
- `fullwidth` (alias: `fw`)
- `negative-squared` (alias: `neg-sq`)
- `negative-circled` (alias: `neg-circle`)
- `squared-latin` (alias: `sq-latin`)
- `circled-latin` (alias: `circled`, `circle`)
- `small-caps` (alias: `sc`)
- `monospace` (alias: `mono`)
- `double-struck` (alias: `ds`)
- `sans-serif` (alias: `ss`, `sans`)
- `sans-serif-bold` (alias: `ssb`)
- `sans-serif-italic` (alias: `ssi`, `sans-italic`)
- `sans-serif-bold-italic` (alias: `ssbi`, `sans-bold-italic`)
- `italic` (alias: `it`)
- `bold-italic` (alias: `bi`)
- `script` (alias: `scr`, `cursive`, `calligraphic`)
- `bold-script` (alias: `bscr`, `bold-cursive`)
- `fraktur` (alias: `fr`, `gothic`, `blackletter`)
- `bold-fraktur` (alias: `bfr`, `bold-gothic`)

## Integration Examples

### Hugo Blog

```bash
# Process all posts before building
utf8fx process content/posts/*.md --in-place
hugo build
```

### Jekyll

```bash
# Process posts
utf8fx process _posts/*.md --in-place
jekyll build
```

### MkDocs

```bash
# Process docs
utf8fx process docs/**/*.md --in-place
mkdocs build
```

### As a Build Step

```makefile
.PHONY: preprocess build

preprocess:
	utf8fx process content/**/*.md --in-place

build: preprocess
	hugo build
```
