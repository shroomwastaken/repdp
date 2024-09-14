use std::fmt::Display;

/// A shortcut macro for implementing display for dump structs
#[macro_export]
macro_rules! impl_display {
	($target:ident) => {
        use crate::get_fields;
        use crate::print::prettify_field;
        impl Display for $target {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                for (field, value) in get_fields!(self) {
                    write!(f, "{:<20} {}\n", prettify_field(field), value)?;
                }
                Ok(())
            }
        }
    };
}

/// Returns a HashMap consisting of string field-value pairs from a given struct
#[macro_export]
macro_rules! get_fields {
	($obj:ident) => {{
		use std::collections::HashMap;
		let debug = format!("{:#?}", $obj);
		let debug = debug.split('\n').collect::<Vec<&str>>();
		// map over every field-value pair except first and last lines
		let iter = debug[1..debug.len() - 1].iter()
			.map(|line| {
				let line = line.trim();
				// split a single line into a field and its value
				let (field, val) = line.split_at(line.find(" ").unwrap());
				// cut unnecessary characters and convert both to String
				let field = String::from(field);
				let val = String::from(&val[1..val.len() - 1]).replace('\"', "");
				(field, val)
			});
		let map: HashMap<String, String> = HashMap::from_iter(iter);
		map
	}};
}

/// Coverts a struct field name from snake case to camel case
pub fn prettify_field(field: String) -> String {
    let mut pretty_field = field;
    while let Some(index) = pretty_field.find('_') {
        let upper = pretty_field.chars().nth(index + 1).unwrap().to_ascii_uppercase();
        pretty_field.replace_range(index..index + 2, &format!(" {}", upper));
    }
    let mut chars = pretty_field.chars();
    chars.next().unwrap().to_uppercase().collect::<String>() + chars.as_str()
}