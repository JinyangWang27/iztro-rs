# iztro compatibility fixtures

Reference fixtures used to check `iztro-rs` chart output against the upstream
JavaScript implementation.

## Pinned target

- Package: `npm:iztro`
- Version: `2.5.8`

## Generation

Each fixture records its own `metadata.generation_command`. The current command
(from the JSON metadata) is:

```bash
npm install iztro@2.5.8 --prefix /tmp/iztro-fixture && \
cd /tmp/iztro-fixture && \
node --input-type=module -e "import { astro } from 'iztro'; const a = astro.bySolar('1990-5-17', 4, '女', true, 'zh-CN'); console.log(JSON.stringify(a, null, 2));"
```

The raw upstream output is stored under `iztro_output`. The normalized
`supported_fields` block is what the compatibility test asserts against.

## Supported-field-only policy

Fixtures compare **only** fields currently implemented by `iztro-rs`: birth
time, gender, life/body palace branches, palace branches, and palace names.
`metadata.supported_fields_only` is `true`.

### Explicitly excluded fields

- stars
- brightness
- mutagens
- decadal scopes
- yearly scopes
- narrative output

## Scope

The current fixture covers **minimal natal compatibility only**.
