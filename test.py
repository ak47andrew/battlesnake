import subprocess
from alive_progress import alive_bar
import re
import math

pattern = re.compile(r"(\w+) was the winner")
TOTAL_GAMES = 50

prod_wins = 0
draws = 0
with alive_bar(TOTAL_GAMES) as bar:
    for _ in range(TOTAL_GAMES):
        result = subprocess.run(["battlesnake", "play",
                                 "--name", "main", "--url", "https://battlesnake.321657325.xyz",
                                 "--name", "stage", "--url", "http://localhost:9100"],
                                 capture_output=True, text=True)
        output = result.stderr.splitlines()[-1]
        g = re.findall(pattern, output)
        if len(g) == 0:
            prod_wins += 0.5
            draws += 1
        elif g[0] == "main":
            prod_wins += 1
        bar()

prod_winrate = round(prod_wins / TOTAL_GAMES * 100, 2)
stage_winrate = round(100 - prod_winrate, 2)
stage_wins = TOTAL_GAMES - prod_wins - draws
D_prod = round(-400 * math.log10(100 / prod_winrate - 1))
D_stage = round(-400 * math.log10(100 / stage_winrate - 1))

print(f"PROD ({D_prod}) vs STAGE ({D_stage})")
print(f"{prod_winrate}% - {stage_winrate}%")
print(f"{prod_wins} - {draws} - {stage_wins}")
