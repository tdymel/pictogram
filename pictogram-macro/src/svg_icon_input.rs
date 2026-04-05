use syn::{Path, parse::Parse};

pub struct SvgIconInput {
    pub(crate) name: String,
    pub(crate) variant: String,
    pub(crate) source: String,
}

impl Parse for SvgIconInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse::<Path>()?;
        let num_segments = path.segments.len();

        let source = path
            .segments
            .get(num_segments - 3)
            .unwrap()
            .ident
            .to_string();
        let name = path
            .segments
            .get(num_segments - 2)
            .unwrap()
            .ident
            .to_string();
        let variant = path
            .segments
            .get(num_segments - 1)
            .unwrap()
            .ident
            .to_string();

        Ok(Self {
            name,
            variant,
            source,
        })
    }
}
