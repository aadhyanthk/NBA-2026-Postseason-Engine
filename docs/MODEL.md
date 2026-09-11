# NBA Postseason Simulation Model

## 1. Overview
The engine uses a deterministic, discrete possession-based Markov model to simulate NBA games. Rather than abstracting entire games into a single continuous normal distribution (which fails to capture basketball's possession-by-possession reality), this model plays out every single possession.

## 2. Tournament Structure
- **Play-In Tournament**: Seeds 7 vs 8 and 9 vs 10, strictly matching the NBA format.
- **Playoff Bracket**: Best-of-7 series (First Round through NBA Finals).
- **Home Court Advantage**: Series follow the standard 2-2-1-1-1 format.

## 3. Game Simulation
Before simulating a game, the engine calculates the expected match pace and per-possession efficiencies based on the participating teams' regular-season statistics:

### Pace Calculation
$$\text{Pace}_{\text{game}} = \frac{\text{Pace}_{\text{home}} \times \text{Pace}_{\text{away}}}{\text{Pace}_{\text{league}}}$$
*We use 99.0 as the baseline league pace.*

### Efficiency Calculation
The model uses standard Offensive (ORtg) and Defensive (DRtg) ratings.
$$\text{Eff}_{\text{home}} = \text{ORtg}_{\text{home}} + \text{DRtg}_{\text{away}} - 115.0 + 3.2\text{ (HCA)}$$
$$\text{Eff}_{\text{away}} = \text{ORtg}_{\text{away}} + \text{DRtg}_{\text{home}} - 115.0$$
*Where 115.0 is the baseline league efficiency and 3.2 represents Home Court Advantage (HCA).*

Expected Points Per Possession (PPP) is calculated as:
$$\text{PPP} = \frac{\text{Eff}}{100}$$

## 4. Possession Micro-Model
For each possession, a pseudo-random draw dictates the outcome, dynamically scaled by the team's matchup PPP.

$$\text{Scaling Factor } S = \frac{\text{PPP}_{\text{matchup}}}{1.15}$$

- **3-Point Play (3 pts)**: $P(3) = 0.12 \times S$ (~12%)
- **2-Point Play (2 pts)**: $P(2) = 0.35 \times S$ (~35%)
- **Free Throw / And-1 (1 pt)**: $P(1) = 0.09 \times S$ (~9%)
- **Empty Possession (0 pts)**: $P(0) = 1 - (P_3 + P_2 + P_1)$ (~44%)

*Note: This simplifies the intricate mechanics of offensive rebounds, steals, and blocks, but accurately replicates total possession-level efficiency.*

## 5. Overtime Resolution
If regulation (typically ~100 possessions) ends in a tie, the simulation enters 5-minute overtime periods. Each team is given an additional 10 possessions until the tie is broken.

## 6. Deterministic PRNG
The simulation ensures bit-for-bit reproducible results across threads by using a splittable ChaCha20 random number generator. The seed for each simulation is generated via:
$$\text{Seed}_{\text{sim}} = \text{Hash}(\text{MasterSeed}, \text{SimID}, \text{ContextID})$$

## 7. Limitations & Future Improvements
- **Limitations**: The model lacks individual player impacts (injuries, star fatigue) and ignores time-sensitive late-game fouling strategies.
- **Future Improvements**:
  - Implement dynamic variance (e.g., jump-shooting teams have higher standard deviations in point totals).
  - Add specific four-factor metrics (eTOV%, OREB%) for deeper interaction instead of monolithic ORtg/DRtg.
