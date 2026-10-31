#!/bin/bash
# Cross-era Minecraft version smoke matrix for CraftCraft Launcher.
# Each version: real download (client jar + libraries + natives + assets) then a
# 20s JVM boot against the assembled command line.
set -u
cd "$(dirname "$0")/../.." || exit 1  # go to project root
SMOKE=./src-tauri/target/release/smoke
INSTANCE="/Users/linminhao/Library/Application Support/CraftCraft/instances/smoke-test"
LOG=~/Desktop/craftcraft-matrix.log
: > "$LOG"

VERSIONS=(1.5.2 1.7.10 1.8.9 1.12.2 1.16.5 1.18.2 1.20.1 1.21 26.3)
declare -a RESULTS

for v in "${VERSIONS[@]}"; do
  echo "==================== $v ====================" | tee -a "$LOG"
  rm -rf "$INSTANCE"
  out=$("$SMOKE" "$v" 2>&1)
  rc=$?
  echo "$out" | tail -40 | tee -a "$LOG"
  if [ $rc -eq 0 ] && echo "$out" | grep -q "PASS"; then
    RESULTS+=("PASS  $v")
  else
    reason=$(echo "$out" | grep -E "^Error:|classpath unhealthy|no arguments" | head -2 | tr '\n' ' ')
    RESULTS+=("FAIL  $v  ${reason}")
  fi
  echo "" | tee -a "$LOG"
done

echo "==================== SUMMARY ====================" | tee -a "$LOG"
for r in "${RESULTS[@]}"; do echo "$r" | tee -a "$LOG"; done
