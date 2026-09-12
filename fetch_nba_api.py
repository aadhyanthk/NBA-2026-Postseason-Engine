from nba_api.stats.endpoints import leaguedashteamstats
import time
import json
import traceback

seasons = [
    '2015-16', '2016-17', '2017-18', '2018-19', '2019-20', 
    '2020-21', '2021-22', '2022-23', '2023-24', '2024-25', '2025-26'
]

results = {}

for season in seasons:
    print(f"Fetching {season}...")
    try:
        # MeasureType=Advanced gives Pace, ORtg, DRtg, TOV%, OREB%
        stats = leaguedashteamstats.LeagueDashTeamStats(season=season, measure_type_detailed_defense='Advanced')
        df = stats.get_data_frames()[0]
        results[season] = df.to_dict('records')
        time.sleep(1)
    except Exception as e:
        print(f"Failed to fetch {season}: {e}")
        traceback.print_exc()

with open('nba_stats_api.json', 'w') as f:
    json.dump(results, f)

print("Done generating JSON.")
