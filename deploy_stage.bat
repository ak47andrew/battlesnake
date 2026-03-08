@echo off

echo Stopping existing staging container...
docker stop xyz_battlesnake_stage_container >nul 2>&1
docker rm xyz_battlesnake_stage_container >nul 2>&1

echo Building staging image...
docker build -t xyz_battlesnake_stage .

echo Starting staging container...
docker run -d ^
  -e APP_ENV=dev ^
  --restart unless-stopped ^
  -p 9112:9100 ^
  --name xyz_battlesnake_stage_container ^
  xyz_battlesnake_stage

echo.
echo Staging deployed at http://localhost:9112
pause