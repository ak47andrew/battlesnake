@echo off

echo Stopping existing production container...
docker stop xyz_battlesnake_container >nul 2>&1
docker rm xyz_battlesnake_container >nul 2>&1

echo Building production image...
docker build -t xyz_battlesnake .

echo Starting production container...
docker run -d ^
  -e APP_ENV=prod ^
  --restart unless-stopped ^
  -p 9111:9100 ^
  --name xyz_battlesnake_container ^
  xyz_battlesnake

echo.
echo Production deployed at http://localhost:9111
pause