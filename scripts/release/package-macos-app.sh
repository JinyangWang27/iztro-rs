#!/usr/bin/env bash
set -euo pipefail

target="${1:?usage: package-macos-app.sh <target-triple> <version>}"
version="${2:?usage: package-macos-app.sh <target-triple> <version>}"

case "$target" in
  *-apple-darwin) ;;
  *)
    echo "error: macOS target triple expected, got: $target" >&2
    exit 1
    ;;
esac

app_name="Iztro"
bundle_id="io.github.jinyangwang27.iztro"
binary_name="iztro-gui"
app_dir="dist/${app_name}.app"
zip_path="dist/${app_name}-${target}.zip"
resources_dir="$app_dir/Contents/Resources"
licenses_dir="$resources_dir/licenses"

rm -rf "$app_dir" "$zip_path" "$zip_path.sha256"

mkdir -p "$app_dir/Contents/MacOS"
mkdir -p "$resources_dir"
mkdir -p "$licenses_dir"

cp "target/${target}/release/${binary_name}" \
  "$app_dir/Contents/MacOS/${binary_name}"

chmod 755 "$app_dir/Contents/MacOS/${binary_name}"

# Keep license/readme material inside the app bundle so the distributable remains
# self-contained. Missing optional files should fail loudly only for the project
# license itself.
cp README.md "$resources_dir/README.md"
cp LICENSE "$licenses_dir/LICENSE"
if [[ -f crates/iztro-gui/assets/fonts/LICENSE-SourceHanSerif.txt ]]; then
  cp crates/iztro-gui/assets/fonts/LICENSE-SourceHanSerif.txt \
    "$licenses_dir/LICENSE-SourceHanSerif.txt"
fi
if [[ -f crates/iztro-gui/assets/fonts/NOTICE.md ]]; then
  cp crates/iztro-gui/assets/fonts/NOTICE.md \
    "$licenses_dir/FONT-NOTICE.md"
fi

cat > "$app_dir/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key>
  <string>${app_name}</string>

  <key>CFBundleDisplayName</key>
  <string>${app_name}</string>

  <key>CFBundleIdentifier</key>
  <string>${bundle_id}</string>

  <key>CFBundleExecutable</key>
  <string>${binary_name}</string>

  <key>CFBundlePackageType</key>
  <string>APPL</string>

  <key>CFBundleShortVersionString</key>
  <string>${version}</string>

  <key>CFBundleVersion</key>
  <string>${version}</string>

  <key>LSMinimumSystemVersion</key>
  <string>12.0</string>

  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
EOF

echo "APPL????" > "$app_dir/Contents/PkgInfo"

plutil -lint "$app_dir/Contents/Info.plist"

if [[ "${APPLE_CODESIGN:-}" == "1" ]]; then
  : "${APPLE_DEVELOPER_ID_SIGNING_IDENTITY:?missing signing identity}"

  codesign \
    --force \
    --options runtime \
    --timestamp \
    --sign "$APPLE_DEVELOPER_ID_SIGNING_IDENTITY" \
    "$app_dir/Contents/MacOS/${binary_name}"

  codesign \
    --force \
    --options runtime \
    --timestamp \
    --sign "$APPLE_DEVELOPER_ID_SIGNING_IDENTITY" \
    "$app_dir"

  codesign --verify --strict --verbose=4 "$app_dir"

  ditto -c -k --keepParent "$app_dir" "$zip_path"

  if [[ -n "${APPLE_NOTARY_KEYCHAIN_PROFILE:-}" ]]; then
    notary_args=(--keychain-profile "$APPLE_NOTARY_KEYCHAIN_PROFILE")
  else
    : "${APPLE_ID:?missing Apple ID}"
    : "${APPLE_TEAM_ID:?missing Apple team ID}"
    : "${APPLE_APP_SPECIFIC_PASSWORD:?missing app-specific password}"
    notary_args=(
      --apple-id "$APPLE_ID"
      --team-id "$APPLE_TEAM_ID"
      --password "$APPLE_APP_SPECIFIC_PASSWORD"
    )
  fi

  xcrun notarytool submit "$zip_path" "${notary_args[@]}" --wait

  xcrun stapler staple "$app_dir"

  spctl --assess --type execute --verbose "$app_dir"

  rm -f "$zip_path"
fi

ditto -c -k --keepParent "$app_dir" "$zip_path"

if command -v shasum >/dev/null 2>&1; then
  (cd dist && shasum -a 256 "${app_name}-${target}.zip" | tee "${app_name}-${target}.zip.sha256")
elif command -v sha256sum >/dev/null 2>&1; then
  (cd dist && sha256sum "${app_name}-${target}.zip" | tee "${app_name}-${target}.zip.sha256")
else
  echo "error: shasum or sha256sum is required to create checksum" >&2
  exit 1
fi
