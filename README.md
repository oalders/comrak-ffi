# comrak-ffi

Thin C FFI wrapper around [comrak](https://github.com/kivikakk/comrak) for GFM markdown rendering. Produces a shared library (`.so`/`.dylib`/`.dll`) callable from any language with C FFI support.

## GFM Extensions Enabled

- Tables
- Strikethrough
- Autolinks
- Tagfilter
- Header IDs (no prefix)

## API

### `comrak_markdown_to_html`

```c
char* comrak_markdown_to_html(const char* markdown, int unsafe_html);
```

Renders a markdown string to HTML. When `unsafe_html` is non-zero, raw HTML in the input is passed through. When zero, raw HTML is stripped.

Returns a newly allocated string. The caller **must** free it with `comrak_free_string`.

### `comrak_free_string`

```c
void comrak_free_string(char* s);
```

Frees a string returned by `comrak_markdown_to_html`.

## Building

```
cargo build --release
```

The shared library is written to `target/release/libcomrak_ffi.so` (Linux), `target/release/libcomrak_ffi.dylib` (macOS), or `target/release/comrak_ffi.dll` (Windows).

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
