# rust-gettextpo

`libgettextpo` bindings for Rust.

Below functions are available:

```rust
pub fn po_file_create() -> po_file_t;
pub fn po_file_read_v3(filename: *const c_char, handler: po_xerror_handler_t) -> po_file_t;
pub fn po_file_write_v2(file: po_file_t, filename: *const c_char, handler: po_xerror_handler_t) -> po_file_t;
pub fn po_file_free(file: po_file_t);
pub fn po_file_domains(file: po_file_t) -> *const *const c_char;
pub fn po_file_domain_header(file: po_file_t, domain: *const c_char) -> *const c_char;
pub fn po_header_field(header: *const c_char, field: *const c_char) -> *mut c_char;
pub fn po_header_set_field(header: *const c_char, field: *const c_char, value: *const c_char) -> *mut c_char;
pub fn po_message_iterator(file: po_file_t, domain: *const c_char) -> po_message_iterator_t;
pub fn po_message_iterator_free(iterator: po_message_iterator_t);
pub fn po_next_message(iterator: po_message_iterator_t) -> po_message_t;
pub fn po_message_insert(iterator: po_message_iterator_t, message: po_message_t);
pub fn po_message_create() -> po_message_t;
pub fn po_message_msgctxt(message: po_message_t) -> *const c_char;
pub fn po_message_set_msgctxt(message: po_message_t, msgctxt: *const c_char);
pub fn po_message_msgid(message: po_message_t) -> *const c_char;
pub fn po_message_set_msgid(message: po_message_t, msgid: *const c_char);
pub fn po_message_msgid_plural(message: po_message_t) -> *const c_char;
pub fn po_message_set_msgid_plural(message: po_message_t, msgid_plural: *const c_char);
pub fn po_message_msgstr(message: po_message_t) -> *const c_char;
pub fn po_message_set_msgstr(message: po_message_t, msgstr: *const c_char);
pub fn po_message_msgstr_plural(message: po_message_t, index: c_int) -> *const c_char;
pub fn po_message_set_msgstr_plural(message: po_message_t, index: c_int, msgstr: *const c_char);
pub fn po_message_comments(message: po_message_t) -> *const c_char;
pub fn po_message_set_comments(message: po_message_t, comments: *const c_char);
pub fn po_message_extracted_comments(message: po_message_t) -> *const c_char;
pub fn po_message_set_extracted_comments(message: po_message_t, comments: *const c_char);
pub fn po_message_filepos(message: po_message_t, i: c_int) -> po_filepos_t;
pub fn po_message_remove_filepos(message: po_message_t, i: c_int);
pub fn po_message_add_filepos(message: po_message_t, file: *const c_char, start_line: usize);
pub fn po_message_prev_msgctxt(message: po_message_t) -> *const c_char;
pub fn po_message_set_prev_msgctxt(message: po_message_t, prev_msgctxt: *const c_char);
pub fn po_message_prev_msgid(message: po_message_t) -> *const c_char;
pub fn po_message_set_prev_msgid(message: po_message_t, prev_msgid: *const c_char);
pub fn po_message_prev_msgid_plural(message: po_message_t) -> *const c_char;
pub fn po_message_set_prev_msgid_plural(message: po_message_t, prev_msgid_plural: *const c_char);
pub fn po_message_is_obsolete(message: po_message_t) -> c_int;
pub fn po_message_set_obsolete(message: po_message_t, obsolete: c_int);
pub fn po_message_is_fuzzy(message: po_message_t) -> c_int;
pub fn po_message_set_fuzzy(message: po_message_t, fuzzy: c_int);
pub fn po_message_is_format(message: po_message_t, format_type: *const c_char) -> c_int;
pub fn po_message_set_format(message: po_message_t, format_type: *const c_char, value: c_int);
pub fn po_message_is_range(message: po_message_t, minp: *mut c_int, maxp: *mut c_int) -> c_int;
pub fn po_message_set_range(message: po_message_t, min: c_int, max: c_int);
pub fn po_filepos_file(filepos: po_filepos_t) -> *const c_char;
pub fn po_filepos_start_line(filepos: po_filepos_t) -> usize;
pub fn po_format_list() -> *const *const c_char;
pub fn po_format_pretty_name(format_type: *const c_char) -> *const c_char;
pub fn po_file_check_all(file: po_file_t, handler: po_xerror_handler_t);
pub fn po_message_check_all(message: po_message_t, iterator: po_message_iterator_t, handler: po_xerror_handler_t);
pub fn po_message_check_format_v2(message: po_message_t, handler: po_xerror_handler_t);
```

# License

Because this crate is linking to `libgettextpo`, it is licensed under the GPLv3 or later.

https://www.gnu.org/software/gettext/manual/html_node/Licenses.html

From `gettext-po.h` header:

```
/* Public API for GNU gettext PO files - contained in libgettextpo.
   Copyright (C) 2003-2008, 2010, 2012-2016, 2019-2023 Free Software Foundation, Inc.
   Written by Bruno Haible <bruno@clisp.org>, 2003.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */
```
