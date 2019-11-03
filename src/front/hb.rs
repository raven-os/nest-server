use chrono::{DateTime, Utc};
use libnest::package::PackageRequirement;
use rocket_contrib::templates::handlebars::{
    Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext,
};
use serde_json::value::Value;

pub fn timeago(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
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
    out: &mut dyn Output,
) -> HelperResult {
    let maybe_param = h
        .param(0)
        .and_then(|param| serde_json::from_value::<PackageRequirement>(param.value().clone()).ok());

    if let Some(val) = maybe_param
        .as_ref()
        .and_then(|req| req.repository().as_ref())
    {
        out.write(val)?;
    }

    Ok(())
}

pub fn category_name(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let maybe_param = h
        .param(0)
        .and_then(|param| serde_json::from_value::<PackageRequirement>(param.value().clone()).ok());

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
    out: &mut dyn Output,
) -> HelperResult {
    let maybe_param = h
        .param(0)
        .and_then(|param| serde_json::from_value::<PackageRequirement>(param.value().clone()).ok());

    if let Some(val) = maybe_param {
        out.write(val.name())?;
    }

    Ok(())
}

pub fn version_req(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let maybe_param = h
        .param(0)
        .and_then(|param| serde_json::from_value::<PackageRequirement>(param.value().clone()).ok());

    if let Some(val) = maybe_param {
        out.write(&val.version_requirement().to_string())?;
    }

    Ok(())
}

pub fn capitalize(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
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
    out: &mut dyn Output,
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
    out: &mut dyn Output,
) -> HelperResult {
    for param in h.params() {
        out.write(&param.value().render())?;
    }

    Ok(())
}

pub fn eq(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let a = h.param(0);
    let b = h.param(1);

    if let (Some(a), Some(b)) = (a, b) {
        if a.value() == b.value() {
            out.write("1")?;
        }
    }

    Ok(())
}

pub fn neq(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let a = h.param(0);
    let b = h.param(1);

    if let (Some(a), Some(b)) = (a, b) {
        if a.value() != b.value() {
            out.write("1")?;
        }
    }

    Ok(())
}
