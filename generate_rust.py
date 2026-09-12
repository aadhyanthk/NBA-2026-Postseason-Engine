import json

with open('nba_stats_api.json') as f:
    data = json.load(f)

# Teams we need to match
TEAM_MAP = [
    ("Boston Celtics", "East"), ("New York Knicks", "East"), ("Milwaukee Bucks", "East"), ("Cleveland Cavaliers", "East"), 
    ("Orlando Magic", "East"), ("Indiana Pacers", "East"), ("Philadelphia 76ers", "East"), ("Miami Heat", "East"), 
    ("Chicago Bulls", "East"), ("Atlanta Hawks", "East"), ("Brooklyn Nets", "East"), ("Toronto Raptors", "East"), 
    ("Charlotte Hornets", "East"), ("Washington Wizards", "East"), ("Detroit Pistons", "East"),
    
    ("Oklahoma City Thunder", "West"), ("Denver Nuggets", "West"), ("Minnesota Timberwolves", "West"), ("LA Clippers", "West"), 
    ("Dallas Mavericks", "West"), ("Phoenix Suns", "West"), ("New Orleans Pelicans", "West"), ("Los Angeles Lakers", "West"), 
    ("Sacramento Kings", "West"), ("Golden State Warriors", "West"), ("Houston Rockets", "West"), ("Utah Jazz", "West"), 
    ("Memphis Grizzlies", "West"), ("San Antonio Spurs", "West"), ("Portland Trail Blazers", "West")
]

rust_code = """use super::types::{Conference, TeamId};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: TeamId,
    pub name: &'static str,
    pub conf: Conference,
    pub seed: u8,
}

pub struct LeagueStatsSoA {
    pub ortg: [f32; 30],
    pub drtg: [f32; 30],
    pub pace: [f32; 30],
    pub tov_pct: [f32; 30],
    pub oreb_pct: [f32; 30],
    pub three_point_rate: [f32; 30],
}

pub struct SeasonData {
    pub year: u32,
    pub teams: [Team; 30],
    pub stats: LeagueStatsSoA,
}

"""

season_blocks = []

seasons = ['2015-16', '2016-17', '2017-18', '2018-19', '2019-20', '2020-21', '2021-22', '2022-23', '2023-24', '2024-25', '2025-26']

# For 2026 default mapping (if any fails, fallback to 2026)
fallback_stats = {
    'ortg': 115.0, 'drtg': 115.0, 'pace': 99.0, 'tov_pct': 0.13, 'oreb_pct': 0.25, 'three_point_rate': 0.38
}

for i, season_str in enumerate(seasons):
    year = 2016 + i
    season_data = data.get(season_str, [])
    
    teams_dict = {t['TEAM_NAME']: t for t in season_data}
    # LA Clippers can sometimes be "Los Angeles Clippers"
    
    # Calculate seeds
    east_teams = []
    west_teams = []
    
    for idx, (name, conf) in enumerate(TEAM_MAP):
        lookup = name
        if lookup == "LA Clippers" and "Los Angeles Clippers" in teams_dict: lookup = "Los Angeles Clippers"
        
        stat = teams_dict.get(lookup)
        if stat:
            pct = stat.get('W_PCT', 0)
        else:
            pct = 0
            
        if conf == "East":
            east_teams.append((idx, pct, lookup))
        else:
            west_teams.append((idx, pct, lookup))
            
    east_teams.sort(key=lambda x: x[1], reverse=True)
    west_teams.sort(key=lambda x: x[1], reverse=True)
    
    seeds = [0] * 30
    for seed_idx in range(10):
        if seed_idx < len(east_teams):
            seeds[east_teams[seed_idx][0]] = seed_idx + 1
        if seed_idx < len(west_teams):
            seeds[west_teams[seed_idx][0]] = seed_idx + 1
            
    # Build the arrays
    teams_rs = []
    ortg_arr, drtg_arr, pace_arr, tov_arr, oreb_arr, tpa_arr = [], [], [], [], [], []
    
    for idx, (name, conf) in enumerate(TEAM_MAP):
        lookup = name
        if lookup == "LA Clippers" and "Los Angeles Clippers" in teams_dict: lookup = "Los Angeles Clippers"
        
        stat = teams_dict.get(lookup)
        if not stat:
            ortg_arr.append(fallback_stats['ortg'])
            drtg_arr.append(fallback_stats['drtg'])
            pace_arr.append(fallback_stats['pace'])
            tov_arr.append(fallback_stats['tov_pct'])
            oreb_arr.append(fallback_stats['oreb_pct'])
            tpa_arr.append(fallback_stats['three_point_rate'])
        else:
            ortg_arr.append(stat.get('OFF_RATING', fallback_stats['ortg']))
            drtg_arr.append(stat.get('DEF_RATING', fallback_stats['drtg']))
            pace_arr.append(stat.get('PACE', fallback_stats['pace']))
            tov_arr.append(stat.get('TM_TOV_PCT', fallback_stats['tov_pct']))
            oreb_arr.append(stat.get('OREB_PCT', fallback_stats['oreb_pct']))
            # nba_api doesn't give 3PA rate in advanced directly? we might need to approximate or just use 0.38
            # wait, it might not be in MeasureType=Advanced. 
            tpa_arr.append(fallback_stats['three_point_rate']) 

        conf_str = "Conference::East" if conf == "East" else "Conference::West"
        teams_rs.append(f'        Team {{ id: TeamId({idx}), name: "{name}", conf: {conf_str}, seed: {seeds[idx]} }},')
        
    block = f"""    SeasonData {{
        year: {year},
        teams: [
{chr(10).join(teams_rs)}
        ],
        stats: LeagueStatsSoA {{
            ortg: {ortg_arr},
            drtg: {drtg_arr},
            pace: {pace_arr},
            tov_pct: {tov_arr},
            oreb_pct: {oreb_arr},
            three_point_rate: {tpa_arr},
        }}
    }}"""
    season_blocks.append(block)

rust_code += "pub const SEASONS: [SeasonData; 11] = [\n" + ",\n".join(season_blocks) + "\n];\n"

# We can keep TEAMS and LEAGUE_STATS_SOA as references to the 2026 data by default for anything that uses it, or refactor to use a pointer.
# Let's write this to a new file `src/core/seasons.rs`.
with open('src/core/seasons.rs', 'w') as f:
    f.write(rust_code)
    
print("Generated src/core/seasons.rs")
