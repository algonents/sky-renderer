# Radar Primitives

Graphics primitives needed for ATM radar visualization, organized by priority.

## Critical (Must Have)

| Primitive | Use Case | Status |
|-----------|----------|--------|
| **Text** | Callsigns, flight levels, speeds, waypoint names, sector labels | Missing |
| **Rotated shapes** | Aircraft symbols oriented to heading, runway representations | Missing |
| **Dashed/dotted lines** | Predicted tracks, airways, FIR boundaries, inactive routes | Missing |
| **Vector/arrow** | Velocity vectors, heading indicators, wind barbs | Missing |

## Important (Should Have)

| Primitive | Use Case | Status |
|-----------|----------|--------|
| **Filled arc (pie slice)** | Radar coverage sectors, airspace sectors | Missing (arc is stroke-only) |
| **Annular sector** | Range ring segments, partial coverage areas | Missing |
| **Leader lines** | Connecting labels to aircraft symbols | Missing |
| **Bezier/spline curves** | SIDs, STARs, smooth airway depictions | Missing |
| **Range rings** | Concentric distance markers (convenience API over circles) | Missing |

## Nice to Have

| Primitive | Use Case | Status |
|-----------|----------|--------|
| **Hatched/patterned fills** | Restricted areas, danger zones, prohibited airspace | Missing |
| **Compass rose** | Bearing reference overlay | Missing |
| **Thick arc** | Holding patterns, procedure turns | Partial (arc has width param) |

## Currently Supported

- Point
- MultiPoint
- Line
- Polyline
- Arc
- Triangle
- Rectangle
- RoundedRectangle
- Circle
- Ellipse
- Polygon
- Image
