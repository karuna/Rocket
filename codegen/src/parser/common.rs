use std::str::FromStr;

use syntax::ast::*;
use syntax::ext::base::ExtCtxt;
use rocket::http::MediaType;
use super::keyvalue::KVSpanned;

pub fn parse_opt<O, T, F>(ecx: &ExtCtxt, kv: &KVSpanned<T>, f: F) -> Option<KVSpanned<O>>
    where F: Fn(&ExtCtxt, &KVSpanned<T>) -> O
{
    Some(kv.map_ref(|_| f(ecx, kv)))
}

pub fn parse_format(ecx: &ExtCtxt, kv: &KVSpanned<LitKind>) -> MediaType {
    if let LitKind::Str(ref s, _) = *kv.value() {
        if let Ok(ct) = MediaType::from_str(&s.as_str()) {
            if !ct.is_known() {
                let msg = format!("'{}' is not a known media type", s);
                ecx.span_warn(kv.value.span, &msg);
            }

            return ct;
        } else {
            ecx.span_err(kv.value.span, "malformed media type");
        }
    }

    ecx.struct_span_err(kv.span, r#"`format` must be a "media/type""#)
        .help(r#"format, if specified, must be a key-value pair where
              the key is `format` and the value is a string representing the
              media type accepted. e.g: format = "application/json""#)
        .emit();

    MediaType::Any
}
