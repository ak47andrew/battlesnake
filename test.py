import subprocess
import tqdm
import re
import math

pattern = re.compile(r"(\w+) was the winner")
TOTAL_GAMES = 1000

prod_wins = 0
draws = 0
for _ in tqdm.tqdm(range(TOTAL_GAMES)):
    result = subprocess.run(["battlesnake", "play", 
                             "--name", "main", "--url", "http://localhost:9111",
                             "--name", "stage", "--url", "http://localhost:9112"],
                             capture_output=True, text=True)
    output = result.stderr.splitlines()[-1]
    g = re.findall(pattern, output)
    if len(g) == 0:
        prod_wins += 0.5
        draws += 1
    elif g[0] == "main":
        prod_wins += 1

prod_winrate = round(prod_wins / TOTAL_GAMES * 100, 2)
stage_winrate = round(100 - prod_winrate, 2)
stage_wins = TOTAL_GAMES - prod_wins - draws
D_prod = round(-400 * math.log10(100 / prod_winrate - 1))
D_stage = round(-400 * math.log10(100 / stage_winrate - 1))

print(f"PROD ({D_prod}) vs STAGE ({D_stage})")
print(f"{prod_winrate}% - {stage_winrate}%")
print(f"{prod_wins} - {draws} - {stage_wins}")
