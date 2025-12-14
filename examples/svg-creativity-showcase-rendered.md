# SVG Creativity Showcase

**Exploring the expressive power of mdfx's enhanced SVG primitives**

This document demonstrates the new SVG rendering capabilities introduced in mdfx v2.0:
- Custom dimensions (`width`, `height`)
- Transparency (`opacity`)
- Borders (`border`, `border_width`)
- Text labels (`label`)
- Multiple style variations

---

## The Building Blocks

**★ Insight ─────────────────────────────────────**
These aren't just "colored squares" anymore. With custom dimensions, opacity, borders, and labels, they become **design primitives** for creating visual hierarchies, data visualizations, and branded layouts.
─────────────────────────────────────────────────

### Basic Swatches (Default: 20x20px)

```markdown
{{ui:swatch:F41C80/}}
{{ui:swatch:4A9EFF/}}
{{ui:swatch:22C55E/}}
{{ui:swatch:EAB308/}}
{{ui:swatch:8B5CF6/}}
```

### The Five Styles

**Rounded (flat)**
```markdown
{{ui:swatch:F41C80:style=flat/}}
```

**Sharp corners (flat-square)** - Default
```markdown
{{ui:swatch:F41C80:style=flat-square/}}
```

**Tall (for-the-badge)**
```markdown
{{ui:swatch:F41C80:style=for-the-badge/}}
```

**Shiny (plastic)**
```markdown
{{ui:swatch:F41C80:style=plastic/}}
```

**Very rounded (social)**
```markdown
{{ui:swatch:F41C80:style=social/}}
```

---

## Creative Compositions

### 1. Progress Bars

**Health bar (video game style)**
```markdown
{{ui:swatch:22C55E:width=80:height=16/}}{{ui:swatch:EF4444:width=20:height=16:opacity=0.3/}}
```

**Loading indicator (gradient effect)**
```markdown
{{ui:swatch:4A9EFF:width=60:opacity=1.0/}}{{ui:swatch:4A9EFF:width=40:opacity=0.7/}}{{ui:swatch:4A9EFF:width=30:opacity=0.4/}}{{ui:swatch:4A9EFF:width=20:opacity=0.2/}}
```

### 2. Data Visualization

**Chart legend**
```markdown
{{ui:swatch:22C55E:width=12:height=12/}} Complete (45%)  
{{ui:swatch:EAB308:width=12:height=12/}} In Progress (30%)  
{{ui:swatch:EF4444:width=12:height=12/}} Blocked (15%)  
{{ui:swatch:6B7280:width=12:height=12/}} Not Started (10%)
```

**Priority indicators**
```markdown
🔴 {{ui:swatch:EF4444:width=100:height=8/}} Critical Priority  
🟡 {{ui:swatch:EAB308:width=70:height=8/}} High Priority  
🟢 {{ui:swatch:22C55E:width=40:height=8/}} Normal Priority
```

### 3. Labeled Badges

**Version badges**
```markdown
{{ui:swatch:4A9EFF:width=60:height=28:label=v2.0:style=for-the-badge/}}
{{ui:swatch:22C55E:width=70:height=28:label=stable:style=for-the-badge/}}
{{ui:swatch:EAB308:width=60:height=28:label=beta:style=for-the-badge/}}
```

**Status indicators with labels**
```markdown
{{ui:swatch:22C55E:width=50:height=24:label=API:border=FFFFFF:border_width=1/}}
{{ui:swatch:4A9EFF:width=50:height=24:label=WEB:border=FFFFFF:border_width=1/}}
{{ui:swatch:8B5CF6:width=50:height=24:label=DB:border=FFFFFF:border_width=1/}}
```

### 4. Transparency Overlays

**Layered transparency (depth effect)**
```markdown
{{ui:swatch:F41C80:width=80:height=60:opacity=1.0/}}  
{{ui:swatch:4A9EFF:width=80:height=60:opacity=0.7/}}  
{{ui:swatch:22C55E:width=80:height=60:opacity=0.4/}}
```

**Frosted glass effect**
```markdown
{{ui:swatch:FFFFFF:width=150:height=40:opacity=0.3:border=E5E7EB:border_width=1/}}
```

### 5. Bordered Designs

**Card-like elements**
```markdown
{{ui:swatch:1E293B:width=200:height=80:border=4A9EFF:border_width=3:label=Feature/}}
{{ui:swatch:1E293B:width=200:height=80:border=22C55E:border_width=3:label=Docs/}}
{{ui:swatch:1E293B:width=200:height=80:border=EAB308:border_width=3:label=Tests/}}
```

**Outlined buttons**
```markdown
{{ui:swatch:FFFFFF:width=80:height=32:border=4A9EFF:border_width=2:label=Primary/}}
{{ui:swatch:FFFFFF:width=80:height=32:border=6B7280:border_width=2:label=Secondary/}}
{{ui:swatch:FFFFFF:width=80:height=32:border=EF4444:border_width=2:label=Danger/}}
```

### 6. Custom Dimensions

**Wide banners**
```markdown
{{ui:swatch:4A9EFF:width=400:height=12/}}
```

**Tall dividers**
```markdown
{{ui:swatch:E5E7EB:width=2:height=60/}} Content here {{ui:swatch:E5E7EB:width=2:height=60/}}
```

**Icon-sized blocks**
```markdown
{{ui:swatch:22C55E:width=16:height=16/}} Success  
{{ui:swatch:EF4444:width=16:height=16/}} Error  
{{ui:swatch:EAB308:width=16:height=16/}} Warning
```

---

## Real-World Use Cases

### Project Status Dashboard

```markdown
## 🚀 Project Velocity

**Sprint Progress**
{{ui:swatch:22C55E:width=150:height=20:label=75%/}}{{ui:swatch:E5E7EB:width=50:height=20:opacity=0.3/}}

**Team Capacity**
{{ui:swatch:4A9EFF:width=120:height=20:label=60%/}}{{ui:swatch:E5E7EB:width=80:height=20:opacity=0.3/}}

**Code Coverage**
{{ui:swatch:8B5CF6:width=180:height=20:label=90%/}}{{ui:swatch:E5E7EB:width=20:height=20:opacity=0.3/}}
```

### Feature Comparison Matrix

```markdown
## Feature Comparison

| Feature | Free | Pro | Enterprise |
|---------|------|-----|------------|
| API Calls | {{ui:swatch:6B7280:width=40:height=12:label=100/}} | {{ui:swatch:4A9EFF:width=60:height=12:label=10K/}} | {{ui:swatch:22C55E:width=80:height=12:label=∞/}} |
| Storage | {{ui:swatch:6B7280:width=30:height=12:label=1GB/}} | {{ui:swatch:4A9EFF:width=50:height=12:label=100GB/}} | {{ui:swatch:22C55E:width=70:height=12:label=1TB/}} |
| Support | {{ui:swatch:EF4444:width=20:height=12/}} None | {{ui:swatch:EAB308:width=40:height=12:label=Email/}} | {{ui:swatch:22C55E:width=60:height=12:label=24/7/}} |
```

### Architecture Diagram

```markdown
## System Architecture

**Frontend Layer**
{{ui:swatch:4A9EFF:width=200:height=50:border=1E293B:border_width=2:label=React/}}

↓

**API Gateway**
{{ui:swatch:8B5CF6:width=200:height=40:border=1E293B:border_width=2:label=GraphQL/}}

↓

**Services**
{{ui:swatch:22C55E:width=95:height=35:border=1E293B:border_width=1:label=Auth/}} 
{{ui:swatch:22C55E:width=95:height=35:border=1E293B:border_width=1:label=Data/}}

↓

**Database**
{{ui:swatch:EAB308:width=200:height=40:border=1E293B:border_width=2:label=PostgreSQL/}}
```

### Color Palette Documentation

```markdown
## Brand Colors

**Primary Palette**
{{ui:swatch:0F172A:width=100:height=60:label=Navy:border=FFFFFF:border_width=1/}}
{{ui:swatch:4A9EFF:width=100:height=60:label=Blue:border=FFFFFF:border_width=1/}}
{{ui:swatch:22C55E:width=100:height=60:label=Green:border=FFFFFF:border_width=1/}}

**Semantic Colors**
{{ui:swatch:22C55E:width=80:height=50:label=Success/}}
{{ui:swatch:EAB308:width=80:height=50:label=Warning/}}
{{ui:swatch:EF4444:width=80:height=50:label=Error/}}
{{ui:swatch:6B7280:width=80:height=50:label=Neutral/}}

**Opacity Variations**
{{ui:swatch:4A9EFF:width=60:height=40:opacity=1.0:label=100%/}}
{{ui:swatch:4A9EFF:width=60:height=40:opacity=0.8:label=80%/}}
{{ui:swatch:4A9EFF:width=60:height=40:opacity=0.6:label=60%/}}
{{ui:swatch:4A9EFF:width=60:height=40:opacity=0.4:label=40%/}}
{{ui:swatch:4A9EFF:width=60:height=40:opacity=0.2:label=20%/}}
```

---

## Advanced Techniques

### 1. Pixel Art Style

```markdown
{{ui:swatch:000000:width=10:height=10/}}{{ui:swatch:000000:width=10:height=10/}}{{ui:swatch:FFFFFF:width=10:height=10/}}{{ui:swatch:FFFFFF:width=10:height=10/}}{{ui:swatch:000000:width=10:height=10/}}{{ui:swatch:000000:width=10:height=10/}}
{{ui:swatch:000000:width=10:height=10/}}{{ui:swatch:FFFFFF:width=10:height=10/}}{{ui:swatch:000000:width=10:height=10/}}{{ui:swatch:000000:width=10:height=10/}}{{ui:swatch:FFFFFF:width=10:height=10/}}{{ui:swatch:000000:width=10:height=10/}}
{{ui:swatch:FFFFFF:width=10:height=10/}}{{ui:swatch:000000:width=10:height=10/}}{{ui:swatch:000000:width=10:height=10/}}{{ui:swatch:000000:width=10:height=10/}}{{ui:swatch:000000:width=10:height=10/}}{{ui:swatch:FFFFFF:width=10:height=10/}}
```

### 2. Timeline Visualization

```markdown
**2024 Roadmap**

Q1 {{ui:swatch:22C55E:width=100:height=16:label=v1.0/}} ✓ Complete  
Q2 {{ui:swatch:4A9EFF:width=100:height=16:label=v1.5/}} ← Current  
Q3 {{ui:swatch:E5E7EB:width=100:height=16:label=v2.0:opacity=0.4/}} Planned  
Q4 {{ui:swatch:E5E7EB:width=100:height=16:label=v2.5:opacity=0.4/}} Planned
```

### 3. Heat Map

```markdown
**Contribution Activity**

Mon {{ui:swatch:22C55E:width=12:height=12:opacity=0.2/}}{{ui:swatch:22C55E:width=12:height=12:opacity=0.4/}}{{ui:swatch:22C55E:width=12:height=12:opacity=0.8/}}{{ui:swatch:22C55E:width=12:height=12:opacity=1.0/}}  
Tue {{ui:swatch:22C55E:width=12:height=12:opacity=0.6/}}{{ui:swatch:22C55E:width=12:height=12:opacity=0.8/}}{{ui:swatch:22C55E:width=12:height=12:opacity=0.4/}}{{ui:swatch:22C55E:width=12:height=12:opacity=0.2/}}  
Wed {{ui:swatch:22C55E:width=12:height=12:opacity=0.8/}}{{ui:swatch:22C55E:width=12:height=12:opacity=1.0/}}{{ui:swatch:22C55E:width=12:height=12:opacity=0.6/}}{{ui:swatch:22C55E:width=12:height=12:opacity=0.4/}}
```

### 4. Tag Cloud

```markdown
{{ui:swatch:4A9EFF:width=60:height=22:label=rust:style=social/}}
{{ui:swatch:8B5CF6:width=80:height=26:label=markdown:style=social/}}
{{ui:swatch:22C55E:width=50:height=20:label=cli:style=social/}}
{{ui:swatch:EAB308:width=90:height=28:label=compiler:style=social/}}
{{ui:swatch:F41C80:width=70:height=24:label=design:style=social/}}
```

### 5. Signal Strength Indicator

```markdown
**WiFi Signal**

{{ui:swatch:22C55E:width=8:height=12/}}
{{ui:swatch:22C55E:width=8:height=18/}}
{{ui:swatch:22C55E:width=8:height=24/}}
{{ui:swatch:E5E7EB:width=8:height=30:opacity=0.3/}}
{{ui:swatch:E5E7EB:width=8:height=36:opacity=0.3/}}
```

---

## Style Mixing

### Combining Styles and Parameters

**Plastic style with custom dimensions**
```markdown
{{ui:swatch:4A9EFF:width=100:height=40:style=plastic:label=SHINY/}}
```

**Social style with borders**
```markdown
{{ui:swatch:F41C80:width=80:height=32:style=social:border=FFFFFF:border_width=2:label=SOFT/}}
```

**For-the-badge with opacity**
```markdown
{{ui:swatch:22C55E:width=120:height=28:style=for-the-badge:opacity=0.8:label=ALPHA/}}
```

---

## Integration with Text Styles

### Combining with mathbold

```markdown
{{ui:swatch:4A9EFF:width=16:height=16/}} {{mathbold}}PROJECT STATUS{{/mathbold}}
```

### Mixed with frames

```markdown
{{frame:gradient}}
{{ui:swatch:F41C80:width=20:height=20/}} {{mathbold}}FEATURED{{/mathbold}}
{{/frame}}
```

---

## Performance Notes

Each unique combination of parameters generates a separate SVG file with a deterministic hash-based filename. This ensures:
- **Reproducible builds** - Same input always produces same output
- **Efficient caching** - Identical swatches reuse the same file
- **Version control friendly** - Asset filenames are stable

---

## Tips for Best Results

1. **Consistency** - Use a limited color palette (5-7 colors max)
2. **Hierarchy** - Vary size/opacity to create visual hierarchy
3. **Alignment** - Keep dimensions consistent within a group
4. **Contrast** - Ensure labels are readable (light text on dark bg, dark on light)
5. **Spacing** - Let swatches breathe, don't pack too tightly

---

**These primitives are your canvas. What will you build?**

*Generated with mdfx v2.0 - SVG backend*
