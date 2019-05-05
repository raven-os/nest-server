use chrono::{DateTime, Utc};
use libnest::package::PackageFullName;
use rocket_contrib::templates::handlebars::{
    Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext,
};
use serde_json::value::Value;

pub fn timeago(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> HelperResult {
    let maybe_param = h
        .param(0)
        .and_then(|param| serde_json::from_value::<DateTime<Utc>>(param.value().clone()).ok());

    if let Some(date) = maybe_param {
        let f = timeago::Formatter::new();
        out.write(&f.convert_chrono(date, Utc::now()))?;
    }

    Ok(())
}

pub fn repository_name(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> HelperResult {
    let maybe_param = h
        .param(0)
        .and_then(|param| serde_json::from_value::<PackageFullName>(param.value().clone()).ok());

    if let Some(val) = maybe_param {
        out.write(val.repository())?;
    }

    Ok(())
}

pub fn category_name(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> HelperResult {
    let maybe_param = h
        .param(0)
        .and_then(|param| serde_json::from_value::<PackageFullName>(param.value().clone()).ok());

    if let Some(val) = maybe_param {
        out.write(val.category())?;
    }

    Ok(())
}

pub fn package_name(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> HelperResult {
    let maybe_param = h
        .param(0)
        .and_then(|param| serde_json::from_value::<PackageFullName>(param.value().clone()).ok());

    if let Some(val) = maybe_param {
        out.write(val.name())?;
    }

    Ok(())
}

pub fn capitalize(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> HelperResult {
    let maybe_param = h
        .param(0)
        .and_then(|param| serde_json::from_value::<String>(param.value().clone()).ok());

    if let Some(s) = maybe_param {
        let mut c = s.chars();

        if let Some(f) = c.next() {
            out.write(&(f.to_uppercase().collect::<String>() + c.as_str()))?;
        }
    }

    Ok(())
}

pub fn plural(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> HelperResult {
    let singular = h.param(0);
    let plural = h.param(1);
    let quantity = h.param(2);

    if let (Some(singular), Some(plural), Some(quantity)) = (singular, plural, quantity) {
        if let Value::Number(quantity) = quantity.value() {
            if let Some(quantity) = quantity.as_u64() {
                if quantity > 1 {
                    out.write(&plural.value().render())?;
                } else {
                    out.write(&singular.value().render())?;
                }
            }
        }
    }
    Ok(())
}

pub fn concat(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> HelperResult {
    for param in h.params() {
        out.write(&param.value().render())?;
    }

    Ok(())
}
