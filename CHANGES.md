# CHANGES.md — Production-Grade Overhaul Specification

> This document specifies every change needed to transform the current prototype into a polished, believable operations control center that demonstrates senior-level engineering quality.

---

## Audit Summary

After reviewing every file in the project against the `PROJECT.md` vision, here is what exists vs. what's missing or broken:

| Area | Current State | Target State |
|------|--------------|--------------|
| **Simulation Engines** | Weather, Transport, Arrival, Gate, Crowd work. Medical/Security only auto-resolve. Cleaning/Food are no-ops. | All 9 subsystems fully functional with causal chains |
| **Event Generation** | Only gate overcrowding and zone density events | Medical, security, weather, transport, maintenance events |
| **Stadium Map** | Dots and circles on Canvas — looks like a debug viz | Proper stadium outline with zone polygons, heatmap gradient, animated crowd flow |
| **Dashboard Metrics** | 8 MetricCards, no sparklines, static "42" for teams | Dynamic data for every card, sparklines showing 5-min history |
| **Incident Feed** | Basic list with resolve button | Filterable, expandable, severity-sorted, with detail panels |
| **AI Agent** | Skeleton: parsePlan returns a mock, ToolExecutor only handles 3 tools | Full OODA loop, real Ollama integration, all 19 tools implemented |
| **Command Bar** | Types to console.log | Actually dispatches to Ollama, results appear inline |
| **AI Panel** | Shows empty AgentHistory | Shows live execution plans with Approve/Reject/Modify |
| **Tab Views** | Security/Medical/Maintenance are near-identical team lists | Each tab has unique, department-specific operational data |
| **CSS / Animations** | Inline styles everywhere, no CSS classes | Proper CSS file with reusable classes, all specified animations working |
| **TopBar** | Static "Speed: 2x" text, no actual speed display | Reactive speed indicator tied to Zustand |
| **Match Timeline** | Only pre-kickoff arrivals | Full timeline: halftime rush, second half, exodus |

---

## Phase 1: Simulation Engine Realism

### [x] 1.1 Complete the Placeholder Engines

#### [x] `MedicalEngine.ts` — Full Implementation
- Track 4 medical teams (expand from 2) with individual cooldown timers
- Generate medical incidents probabilistically: base rate ~2/hour, modified by:
  - Temperature > 35°C → +40% heat exhaustion risk
  - Rain > 0.5 → +25% slip/fall risk  
  - High crowd density zones → +15% risk
  - Match phase (halftime rush) → +20% risk
- Incident types: heat_exhaustion, fall, cardiac, allergic_reaction, intoxication
- Track response times per team (dispatch tick → arrival tick based on distance)
- Auto-resolve: deployed team at location resolves in 60-300 seconds depending on severity

#### [x] `SecurityEngine.ts` — Full Implementation
- Track 6 security teams (expand from 2) with zone assignments
- Generate security incidents: base rate ~1/hour, modified by:
  - Match tension (late game, close score) → +30%
  - High alcohol zones → +20%
  - Gate scanner failures → +15%
- Incident types: unattended_bag, altercation, unauthorized_access, crowd_surge
- Scanner degradation: each gate scanner has a health value (0-100) that randomly degrades. At <50 → degraded, at <20 → failed
- Auto-resolve: similar to medical, but security incidents take 120-600 seconds

#### [x] `CleaningEngine.ts` — Full Implementation
- Track 4 cleaning crews
- Track litter levels per zone (0-1 float), increases with crowd density and food court proximity
- Track restroom status per concourse: clean → needs_attention → critical based on usage rate
- Generate cleaning events when litter > 0.7 or restroom status becomes critical

#### [x] `FoodEngine.ts` — Full Implementation  
- Track 8 food courts with:
  - Queue lengths (based on nearby crowd density + match phase)
  - Stock levels for drinks and food (deplete over time, faster in heat)
  - Revenue tracking (cosmetic but adds realism to the dashboard)
- Generate events: stock below 20%, queue > 50 people, vendor equipment failure

### [x] 1.2 Weather Engine Enhancement
- Add humidity tracking (30-95%, affects heat exhaustion risk)
- Add weather "events" — sudden rain squall (probability ramp from 0 to 0.7 over 10 minutes), temperature spike
- Generate weather events in the EventEngine: rain > 0.7 triggers a weather incident, temp > 38°C triggers heat advisory

### [x] 1.3 Match Timeline Phases
Currently the sim only models pre-kickoff arrivals. Add all phases from PROJECT.md §5.4:

| Phase | SimTime | Behavior Changes |
|-------|---------|-----------------|
| First Half | T+0 to T+2700 | Arrivals stop. Concourses empty. Seating fills. Low incident rate. |
| Halftime | T+2700 to T+3600 | Mass movement from seats → concourses → food → restrooms. Food queues spike. Cleaning urgency spikes. Medical risk from crowded corridors. |
| Second Half | T+3600 to T+6300 | Similar to first half but slightly elevated tension. |
| Full Time | T+6300 | Mass exit begins. Gates reverse to outflow mode. Transport coordination needed. |
| Exodus | T+6300 to T+10800 | Gradual emptying. Cleaning sweep. Shift handover window. |

### [x] 1.4 EventEngine Expansion
Currently only generates gate overcrowding and zone density events. Add:
- Medical incidents (from MedicalEngine probabilities)
- Security incidents (from SecurityEngine probabilities)
- Weather events (rain onset, temperature warnings)
- Transport events (major delay, delay resolution surge)
- Maintenance events (scanner degradation, equipment failure)
- Cleaning events (restroom critical, litter threshold)
- Food events (stock depletion, long queues)

Each event should have a realistic, context-aware title and description (not "Manual Test Incident").

---

## Phase 2: UI/UX Overhaul — "Make It Look Real"

### [x] 2.1 Move ALL Inline Styles to CSS Classes

Every component currently uses inline `style={{...}}` objects. This is the single biggest indicator of "vibe coded" to a senior engineer. 

**Action**: Create a comprehensive CSS class system in `index.css` using the existing design tokens. Every component should use `className` instead of inline styles. This is a large refactor but non-negotiable for professional quality.

CSS class categories to create:
- `.card`, `.card--elevated`, `.card--critical`, `.card--warning`
- `.metric-card`, `.metric-value`, `.metric-label`, `.metric-trend`, `.metric-trend--up`, `.metric-trend--down`
- `.incident-card`, `.incident-card--critical`, `.incident-card--resolved`
- `.btn`, `.btn--primary`, `.btn--danger`, `.btn--ghost`, `.btn--sm`
- `.badge`, `.badge--critical`, `.badge--warning`, `.badge--ok`, `.badge--info`
- `.status-dot`, `.status-dot--active`, `.status-dot--idle`
- `.panel`, `.panel-header`, `.panel-body`
- `.tab`, `.tab--active`
- `.topbar`, `.statusbar`, `.tabbar`
- `.grid-2`, `.grid-3`, `.grid-4` (column layouts)
- `.feed`, `.feed-item`
- `.mono` (already exists, expand)

### [x] 2.2 Stadium Map — Complete Redesign

The current map is circles and rectangles on a plain canvas. It needs to look like an actual stadium schematic:

- Draw a proper oval stadium outline with double-ring structure (seating bowl + concourse ring)
- Draw distinct zone polygons (not circles) — trapezoidal seating sections arranged around the oval
- Implement a smooth heatmap gradient per zone (blue → green → yellow → orange → red) using zone density
- Draw the pitch as a proper rectangle with field markings (center circle, penalty areas, halfway line)
- Gate indicators at the perimeter with directional arrows showing inflow
- Animated crowd dots: small semi-transparent circles that drift through concourse zones, density-proportional
- Team position markers: distinct icons for medical (red cross), security (shield), cleaning (broom) teams showing their current location
- Hover tooltip on zones showing: name, occupancy, capacity, density percentage
- Legend in corner showing the heatmap color scale

### 2.3 TopBar Redesign
- Logo: "FIFA 26" in bold + "OCC" in accent color with a small shield/stadium icon (CSS-drawn or inline SVG)
- Speed indicator should be reactive (currently hardcoded "Speed: 2x")
- Ollama status indicator should actually ping localhost:11434 and show real connection status (green=connected, red=disconnected, yellow=connecting)
- Add a clock showing current sim time prominently (digital clock style, JetBrains Mono)

### [x] 2.4 MetricCard Redesign
- Implement actual digit-flip animation (the AnimatedCounter currently just sets the value with no animation)
- Add sparkline (mini line chart) at the bottom of each card showing the last 5 minutes of that metric's history
- The progress bar at the bottom should represent capacity utilization (e.g., total inside / max capacity), not a static 70%
- Color-code the entire card border based on severity threshold (not just the progress bar)

### 2.5 IncidentFeed Overhaul
- Add filter tabs at the top: All | Critical | Medical | Security | Crowd | Maintenance
- Sort by severity (critical first, then by recency)
- Resolved incidents should collapse into a "Resolved" section at the bottom (collapsible)
- Each card should show: severity badge, type icon (inline SVG), timestamp, title, location, assigned team
- Click to expand should show: full description, root cause, related events, AI plan link, resolve/acknowledge buttons
- Auto-scroll to new critical events
- Show total count badge on each filter tab

### [x] 2.6 Overview Tab Layout
- Dashboard weather strip: Temperature, Rain, Wind, Humidity as compact inline indicators
- Stadium map should take ~60% of the vertical space (it's the hero element)
- Right column: AI Predictive Monitor at top, then Incident Feed filling remaining space

### [x] 2.7 Gates Tab — Richer Gate Cards
- Each gate card should show:
  - Gate name + status badge (OPEN/CLOSED) + toggle button
  - Queue length with sparkline (last 5 min history) (already implemented)
  - Wait time with color coding (green < 10min, yellow < 20min, red > 20min)
  - Throughput rate (people/hour, current vs capacity)
  - Active lanes indicator (visual: 4 boxes, filled = active)
  - Scanner health bar (100% green → 0% red)
- Add a summary row at top: Total throughput, average wait, gates open count

### 2.8 Security / Medical / Maintenance Tabs — Distinct Identities
Currently these three tabs are near-identical (team list + incident list). Each needs unique operational data:

**Security Tab:**
- Zone coverage map (mini version of stadium map showing security team positions)
- Scanner health dashboard (6 gates, each with a health percentage bar)
- Threat level indicator (calculated from active security incidents)
- Active deployments with ETA and duration timers

**Medical Tab:**
- Response time statistics (average, fastest, slowest)
- Active patient tracking (incident → dispatch → arrival → treatment → resolved timeline)
- Heat index warning (derived from temperature + humidity)
- Equipment/supply status

**Maintenance Tab:**
- Equipment status grid (scanners, turnstiles, lighting, HVAC, signage — each with status indicator)
- SLA tracking (time since ticket opened vs. target resolution time)
- Cleaning schedule with restroom status indicators (4 concourses × status)
- Ticket priority queue (sorted by urgency)

### 2.9 Agent Tab — Full Conversation Interface
Currently just shows "Total historical plans: 0" and empty history. Needs:
- Full scrollable conversation history with the AI agent
- Each message shows: role (user/agent), timestamp, content
- Execution plans rendered as rich cards (not just text)
- "Generate Handover Report" button should actually call the agent
- "Query SOPs" button should open an inline SOP search interface
- Status indicators: Agent idle / thinking / executing

### 2.10 AI Panel (Slide-In) — Real Execution Plans
Currently an empty shell. Needs:
- Current plan card with: title, severity badge, reasoning text, numbered action list
- Each action shows: tool name, parameters, human-readable description, status indicator (pending, executing, done, failed)
- Three action buttons at the bottom: Approve (green), Reject (red), Modify (amber)
- After approval: sequential execution with visual progress
- Plan history below current plan (scrollable, with status badges)

---

## Phase 3: AI Agent — Make It Actually Work

### 3.1 Agent OODA Loop — Real Integration
The Agent.ts parsePlan method currently returns a hardcoded mock. It needs to:
- Actually parse the Ollama response for tool_calls
- If tool_calls are present, map each to a PlanAction
- If no tool_calls (model doesn't support it), fall back to parsing structured JSON from the response content
- Build a proper ExecutionPlan with real reasoning, root cause, and estimated impact from the LLM response

### [x] 3.2 ToolExecutor — All 19 Tools
Currently only open_gate, close_gate, and adjust_gate_lanes are implemented. Implement ALL tools from PROJECT.md §6.3:

**Operations tools:**
- dispatch_security — Update team status to 'busy', set location, create assignment
- dispatch_medical — Same pattern for medical teams
- dispatch_cleaning — Same pattern for cleaning crews
- send_announcement — Add a visual announcement banner to the UI + log to incident feed
- update_signage — Visual indicator on the stadium map
- create_maintenance_ticket — Add to a maintenance ticket queue in the store
- reserve_emergency_route — Visual indicator on the stadium map (highlighted path)

**Information retrieval tools:**
- get_zone_density — Return current density data from stadiumStore
- get_gate_status — Return gate data
- get_team_status — Return team data filtered by department
- get_active_incidents — Return filtered incidents
- get_weather — Return weather state
- get_transport_status — Return transport state
- query_sop — Search the SOPs array by keyword match and return content

**Reporting tools:**
- generate_situation_summary — Build a text summary from current state
- generate_shift_handover — Build a structured handover report
- generate_incident_report — Build a report for a specific incident

### 3.3 Command Bar — Real Agent Interaction
Currently logs to console. Needs to:
- On Enter: dispatch the query to Agent.react() (or a new Agent.query() method)
- Show a loading spinner while waiting for Ollama response
- Display results in a dropdown panel below the command bar (not a modal)
- Support slash commands: /summary → calls generate_situation_summary, /handover → calls generate_shift_handover, /sop <topic> → calls query_sop
- Show recent queries as suggestions on focus

### 3.4 Agent Auto-Triggering
The agent should automatically react to events:
- **Reactive**: When a critical or high severity event is generated, call Agent.react([event])
- **Proactive**: Every 30 ticks (when not paused), agent scans for emerging patterns
- Plans generated automatically should appear in the AI Panel for human approval before execution

### 3.5 PromptBuilder Enhancement
Build richer context prompts that include:
- Current weather conditions
- All gate statuses (queue, wait, scanner)
- Zone densities (sorted by density descending)
- Active incidents (grouped by type)
- Available teams (with locations and status)
- Recent event history (last 5 events)
- Current match phase

---

## Phase 4: Polish and Professional Quality

### 4.1 Keyboard Shortcuts
Per PROJECT.md §5.5:
- 1/2/3/4 — Set speed to 1x/2x/5x/10x
- Space — Pause/Play toggle
- Ctrl+K or / — Focus command bar
- Escape — Close AI panel, unfocus command bar

### 4.2 Sound Effects for Critical Alerts (Stretch)
- Short alert tone when a critical incident appears
- Subtle click on button presses
- Use Web Audio API (no external audio files needed — generate tones programmatically)

### [x] 4.3 Sparkline Component
The Sparkline.tsx in /common is referenced in PROJECT.md but never implemented. Build it using Recharts:
- Tiny (60px × 20px) line chart
- No axes, no labels — just the line
- Color: green (trending down for queues), red (trending up for queues)
- Used inside MetricCards and GateCards

### [x] 4.4 SeverityBadge Component Enhancement
Create a proper SeverityBadge.tsx that's used consistently across all incident displays:
- Pill shape with background color matching severity
- Pulsing animation for critical badges
- Used in: IncidentCard, IncidentFeed, ExecutionPlan, PredictiveAlerts

### 4.5 StatusIndicator Component
Pulsing dot component for live status:
- Green pulsing = active/connected
- Red pulsing = critical/disconnected
- Yellow static = degraded/idle
- Used in: TopBar (Ollama status), team lists, gate status

### 4.6 Error Handling and Edge Cases
- Graceful handling when Ollama is not running (show connection error in TopBar, disable agent features, still allow simulation)
- Handle empty states elegantly (no incidents = show an "All Clear" state, not just blank space)
- Handle simulation edge cases (no open gates, all teams deployed, max capacity reached)

### 4.7 Performance
- Memoize expensive computations (total occupancy, total queue) with useMemo
- Use Zustand selectors to prevent unnecessary re-renders
- Canvas rendering: only redraw changed zones (dirty flag system)
- Throttle MetricCard updates to 1/second max (avoid flicker)

---

## Priority Order for Implementation

Execute in this order. Each phase builds on the previous one.

1. [x] **CSS Refactor** (Phase 2.1) — Foundation for everything else 
2. [x] **Simulation Engines** (Phase 1.1-1.4) — The sim must be believable first
3. [x] **Stadium Map** (Phase 2.2) — The visual centerpiece
4. [x] **OverviewTab + MetricCards + Sparklines** (Phase 2.4, 2.6, 4.3) — Main dashboard polish
5. [x] **IncidentFeed** (Phase 2.5) — Core operational UI
6. [x] **Gates Tab** (Phase 2.7) — Most-used operational tab
7. **ToolExecutor + Agent** (Phase 3.2, 3.1, 3.3) — Make the AI actually work
8. **AI Panel + Execution Plans** (Phase 2.10, 3.4) — The differentiator
9. **Department Tabs** (Phase 2.8) — Unique operational views
10. **Agent Tab** (Phase 2.9) — Full conversation UI
11. **TopBar + StatusBar + Keyboard** (Phase 2.3, 4.1) — Final polish
12. **Performance + Error Handling** (Phase 4.6, 4.7) — Production quality
