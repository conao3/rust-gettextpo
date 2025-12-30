# rust-gettextpo

Rust bindings for `libgettextpo`, the GNU gettext PO file library.

This crate provides safe access to the `libgettextpo` C library, enabling you to read, write, and manipulate GNU gettext PO (Portable Object) translation files directly from Rust.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gettextpo = "0.1"
```

**Note:** This crate requires `libgettextpo` to be installed on your system. On Debian/Ubuntu:

```sh
apt install libgettextpo-dev
```

## API Overview

This crate exposes the following functions from `libgettextpo`:

### File Operations

| Function | Description |
|----------|-------------|
| `po_file_create` | Create a new PO file in memory |
| `po_file_read` | Read a PO file from disk |
| `po_file_write` | Write a PO file to disk |
| `po_file_free` | Free a PO file from memory |
| `po_file_domains` | Get the list of domains in a file |
| `po_file_domain_header` | Get the header for a specific domain |
| `po_file_check_all` | Check entire file for errors |

### Header Operations

| Function | Description |
|----------|-------------|
| `po_header_field` | Get a header field value |
| `po_header_set_field` | Set a header field value |

### Message Iterator

| Function | Description |
|----------|-------------|
| `po_message_iterator` | Create an iterator over messages |
| `po_message_iterator_free` | Free a message iterator |
| `po_next_message` | Get the next message |
| `po_message_insert` | Insert a message at current position |

### Message Properties

| Function | Description |
|----------|-------------|
| `po_message_create` | Create a new message |
| `po_message_msgctxt` / `po_message_set_msgctxt` | Get/set message context |
| `po_message_msgid` / `po_message_set_msgid` | Get/set message ID |
| `po_message_msgid_plural` / `po_message_set_msgid_plural` | Get/set plural form |
| `po_message_msgstr` / `po_message_set_msgstr` | Get/set translation |
| `po_message_msgstr_plural` / `po_message_set_msgstr_plural` | Get/set plural translation |
| `po_message_comments` / `po_message_set_comments` | Get/set translator comments |
| `po_message_extracted_comments` / `po_message_set_extracted_comments` | Get/set extracted comments |
| `po_message_prev_msgctxt` / `po_message_set_prev_msgctxt` | Get/set previous context |
| `po_message_prev_msgid` / `po_message_set_prev_msgid` | Get/set previous message ID |
| `po_message_prev_msgid_plural` / `po_message_set_prev_msgid_plural` | Get/set previous plural |

### Message Flags

| Function | Description |
|----------|-------------|
| `po_message_is_obsolete` / `po_message_set_obsolete` | Check/set obsolete flag |
| `po_message_is_fuzzy` / `po_message_set_fuzzy` | Check/set fuzzy flag |
| `po_message_is_format` / `po_message_set_format` | Check/set format flags |
| `po_message_is_range` / `po_message_set_range` | Check/set range |
| `po_message_check_all` | Check message for errors |
| `po_message_check_format` | Check message format strings |

### File Position

| Function | Description |
|----------|-------------|
| `po_message_filepos` | Get file position at index |
| `po_message_remove_filepos` | Remove file position |
| `po_message_add_filepos` | Add file position |
| `po_filepos_file` | Get filename from position |
| `po_filepos_start_line` | Get line number from position |

### Format Utilities

| Function | Description |
|----------|-------------|
| `po_format_list` | Get list of format types |
| `po_format_pretty_name` | Get human-readable format name |

## License

This crate is licensed under the **GNU General Public License v3.0 or later** (GPLv3+), as it links against `libgettextpo`.

See the [GNU gettext licensing information](https://www.gnu.org/software/gettext/manual/html_node/Licenses.html) for details.
