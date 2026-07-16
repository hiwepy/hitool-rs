#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
hutool_root="${1:-${repo_root}/../hutool}"
output="${2:-${repo_root}/parity/hutool-v5.8.46-api.csv}"
expected_commit="a0bd223dc0d036f55cfe4d8e2f5737ddc31f2b12"

if [[ ! -d "${hutool_root}/.git" ]]; then
  echo "Hutool checkout not found: ${hutool_root}" >&2
  exit 2
fi

actual_commit="$(git -C "${hutool_root}" rev-parse HEAD)"
if [[ "${actual_commit}" != "${expected_commit}" ]]; then
  echo "Expected Hutool v5.8.46 ${expected_commit}, found ${actual_commit}" >&2
  exit 2
fi

if [[ ! -f "${hutool_root}/.codegraph/codegraph.db" ]]; then
  codegraph init -i "${hutool_root}"
else
  codegraph sync "${hutool_root}"
fi

mkdir -p "$(dirname "${output}")"
temporary="${output}.tmp"
sqlite3 -header -csv "${hutool_root}/.codegraph/codegraph.db" >"${temporary}" <<'SQL'
SELECT
  qualified_name || '#' || COALESCE(signature, '') AS api_id,
  substr(file_path, 1, instr(file_path, '/') - 1) AS module,
  kind,
  qualified_name,
  COALESCE(signature, '') AS signature,
  file_path,
  start_line
FROM nodes
WHERE language = 'java'
  AND file_path LIKE '%/src/main/java/%'
  AND (
    (kind IN ('class', 'enum') AND visibility = 'public')
    OR kind = 'interface'
    OR (kind = 'method' AND visibility = 'public')
  )
ORDER BY module, file_path, start_line, qualified_name, signature;
SQL
mv "${temporary}" "${output}"
echo "Generated $(($(wc -l <"${output}") - 1)) Hutool API records at ${output}"

