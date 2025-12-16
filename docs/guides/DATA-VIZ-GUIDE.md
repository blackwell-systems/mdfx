# Data Visualization Guide

mdfx includes powerful components for inline data visualization. This guide covers advanced usage and practical examples for progress bars, donuts, gauges, sparklines, and ratings.

For basic parameters and syntax, see [COMPONENTS-GUIDE.md](COMPONENTS-GUIDE.md).

---

## Quick Reference

| Component | Use Case | Example |
|-----------|----------|---------|
| `progress` | Linear completion, loading, skill bars | `{{ui:progress:75/}}` |
| `donut` | Circular percentages, completion rings | `{{ui:donut:85/}}` |
| `gauge` | Speedometers, meters, half-circle displays | `{{ui:gauge:60/}}` |
| `sparkline` | Trend lines, mini charts, activity graphs | `{{ui:sparkline:1,4,2,5,3/}}` |
| `rating` | Star ratings, scores, reviews | `{{ui:rating:4.5/}}` |

---

## Progress Bars

### Skill Bars

```markdown
| Skill | Level |
|-------|-------|
| Rust | {{ui:progress:95:width=120:fill=success/}} |
| TypeScript | {{ui:progress:85:width=120:fill=info/}} |
| Python | {{ui:progress:75:width=120/}} |
```

### Volume Slider

```markdown
🔊 {{ui:progress:65:width=150:thumb=12:thumb_color=white/}}
```

### Battery Indicator

```markdown
{{ui:progress:80:width=40:height=16:fill=success:rx=2/}}
{{ui:progress:30:width=40:height=16:fill=warning:rx=2/}}
{{ui:progress:10:width=40:height=16:fill=error:rx=2/}}
```

---

## Donut Charts

### Completion Status

```markdown
{{ui:row:align=center}}
{{ui:donut:100:label=true:fill=success/}} Tests
{{ui:donut:85:label=true:fill=info/}} Coverage
{{ui:donut:60:label=true:fill=warning/}} Docs
{{/ui}}
```

### Mini Metrics

```markdown
| Metric | Value |
|--------|-------|
| CPU | {{ui:donut:45:size=30:thickness=3/}} |
| Memory | {{ui:donut:72:size=30:thickness=3/}} |
| Disk | {{ui:donut:88:size=30:thickness=3:fill=warning/}} |
```

---

## Gauges

### System Dashboard

```markdown
{{ui:row:align=center}}
{{ui:gauge:35:fill=success:label=true/}}
{{ui:gauge:65:fill=warning:label=true/}}
{{ui:gauge:90:fill=error:label=true/}}
{{/ui}}

Low | Medium | High
```

### Temperature Display

```markdown
{{ui:gauge:72:size=100:thickness=10:fill=warning:label=true/}}

Temperature: 72°F
```

---

## Sparklines

### Chart Types

```markdown
Line: {{ui:sparkline:10,25,15,30,20,35/}}

Bar: {{ui:sparkline:5,8,3,9,4,7:type=bar/}}

Area: {{ui:sparkline:1,4,2,5,3,6:type=area:fill=info/}}
```

### Activity Timeline

```markdown
**Weekly Commits**
{{ui:sparkline:12,45,28,67,34,89,52:width=200:height=30:type=area:fill=success/}}
```

### Performance Trends

```markdown
| Service | Last 7 Days |
|---------|-------------|
| API | {{ui:sparkline:98,99,97,99,100,98,99:width=80:fill=success/}} |
| DB | {{ui:sparkline:95,92,98,94,96,97,95:width=80:fill=info/}} |
| Cache | {{ui:sparkline:100,99,85,90,95,98,99:width=80:fill=warning/}} |
```

---

## Ratings

### Product Reviews

```markdown
Overall: {{ui:rating:4.5/}} (4.5/5)

| Aspect | Rating |
|--------|--------|
| Quality | {{ui:rating:5/}} |
| Value | {{ui:rating:4/}} |
| Service | {{ui:rating:4.5/}} |
```

### Different Scales

```markdown
5-star: {{ui:rating:3.5/}}
10-point: {{ui:rating:7.5:max=10:size=14/}}
Hearts: {{ui:rating:4:icon=heart:fill=error/}}
```

### Difficulty Levels

```markdown
| Level | Difficulty |
|-------|------------|
| Easy | {{ui:rating:1:icon=circle:fill=success/}} |
| Medium | {{ui:rating:3:icon=circle:fill=warning/}} |
| Hard | {{ui:rating:5:icon=circle:fill=error/}} |
```

---

## Combined Dashboard Example

```markdown
## System Status

| Metric | Current | Trend |
|--------|---------|-------|
| CPU | {{ui:gauge:45:size=50:thickness=5/}} | {{ui:sparkline:30,45,35,50,45,40,45:width=80/}} |
| Memory | {{ui:gauge:72:size=50:thickness=5:fill=warning/}} | {{ui:sparkline:60,65,70,68,72,75,72:width=80:fill=warning/}} |
| Disk | {{ui:gauge:88:size=50:thickness=5:fill=error/}} | {{ui:sparkline:80,82,84,85,86,87,88:width=80:fill=error/}} |

### User Satisfaction

{{ui:rating:4.2/}} Average rating (4.2/5 based on 1,234 reviews)

### Build Status

{{ui:progress:100:width=200:fill=success:label=true/}} All checks passed
```

---

## Tips

1. **Match sizes to context** - Use smaller sizes for inline table cells, larger for standalone displays
2. **Color semantics** - success=green (good), warning=yellow (caution), error=red (bad)
3. **Partial ratings** - Use decimals (3.5) for precise review scores
4. **Sparkline types** - Line for trends, bar for comparisons, area for volume
5. **Combine with row** - Use `{{ui:row}}` to create centered dashboard layouts
