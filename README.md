# comrak-ffi

Thin C FFI wrapper around [comrak](https://github.com/kivikakk/comrak) for GFM markdown rendering. Produces a shared library (`.so`/`.dylib`/`.dll`) callable from any language with C FFI support.

## GFM Extensions Enabled

- Tables
- Strikethrough
- Autolinks
- Header IDs (no prefix)

## API

### `comrak_markdown_to_html`

```c
char* comrak_markdown_to_html(const char* markdown, int unsafe_html);
```

Renders a markdown string to HTML. When `unsafe_html` is non-zero, raw HTML in the input is passed through. When zero, raw HTML is omitted and dangerous link and image URLs are blanked (e.g. a `javascript:` href becomes `href=""`). With `unsafe_html` non-zero the output is not sanitized: raw HTML and link and image URLs are passed through unfiltered, so run it through an HTML sanitizer that also restricts URL schemes before serving untrusted input.

Returns a newly allocated string. The caller **must** free it with `comrak_free_string`.

### `comrak_free_string`

```c
void comrak_free_string(char* s);
```

Frees a string returned by `comrak_markdown_to_html`.

## Building

```
cargo build --release --locked
```

The shared library is written to `target/release/libcomrak_ffi.so` (Linux), `target/release/libcomrak_ffi.dylib` (macOS), or `target/release/comrak_ffi.dll` (Windows).

## Verifying Releases

Each release library has a signed build provenance attestation. Verify it, replacing `<tag>` with the release's tag (e.g. `v0.1.0`):

```sh
gh attestation verify <file> --repo oalders/comrak-ffi \
  --signer-workflow oalders/comrak-ffi/.github/workflows/release.yml \
  --source-ref refs/tags/<tag> \
  --deny-self-hosted-runners
```

Checksums are published as the `SHA256SUMS` release asset. Download it next to your files and run (on macOS, use `shasum -a 256` in place of `sha256sum`):

```sh
sha256sum -c --ignore-missing SHA256SUMS
```

Checksums only detect corruption; use `gh attestation verify` to confirm where a file came from.

## Usage from Perl

```perl
use FFI::Platypus;

my $ffi = FFI::Platypus->new( api => 2 );
$ffi->lib('libcomrak_ffi.so');

my $render = $ffi->function('comrak_markdown_to_html' => ['string', 'int'] => 'opaque');
my $free   = $ffi->function('comrak_free_string' => ['opaque'] => 'void');

my $ptr  = $render->call($markdown, 1);
my $html = $ffi->cast('opaque', 'string', $ptr);
$free->call($ptr);
```

## License

MIT
