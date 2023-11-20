#[macro_export]
macro_rules! model_vec {
	($container:ident, $enum:ident { $($variant:ident -> $variant_str:literal),* }) => {
		#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, rocket::FromFormField)]
		pub enum $enum {
			$(
				#[serde(rename = $variant_str)]
				$variant,
			)*
		}

		impl ToString for $enum {
			fn to_string(&self) -> String {
				match self {
					$(
						$enum::$variant => $variant_str.to_string(),
					)*
				}
			}
		}

		impl std::str::FromStr for $enum {
			type Err = String;

			fn from_str(s: &str) -> Result<Self, Self::Err> {
				match s {
					$(
						$variant_str => Ok($enum::$variant),
					)*
					_ => Err(format!("Invalid {}: {}", stringify!($enum), s)),
				}
			}
		}

		#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sea_orm::FromJsonQueryResult)]
		pub struct $container(pub Vec<$enum>);

		impl $container {
			pub fn into_inner(self) -> Vec<$enum> {
				self.0
			}
		}

		impl ToString for $container {
			fn to_string(&self) -> String {
				self.0
					.iter()
					.map(|x| x.to_string())
					.collect::<Vec<_>>()
					.join(",")
			}
		}

		impl rocket::form::FromFormField<'_> for $container {
			fn from_value(field: rocket::form::ValueField<'_>) -> rocket::form::Result<'_, Self> {
				let values = field.value.split(",").collect::<Vec<_>>();
				let mut enums = Vec::with_capacity(values.len());
				for value in values {
					let parsed = value.parse::<$enum>();
					let Ok(parsed) = parsed else {
						return Err(rocket::form::Error::validation(format!(
							"Invalid {}: {}",
							stringify!($container),
							value
						)).into());
					};

					enums.push(parsed);
				}
				Ok($container(enums))
			}
		}
	};
	($container:ident, String) => {
		#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sea_orm::FromJsonQueryResult)]
		pub struct $container(pub Vec<String>);

		impl $container {
			pub fn into_inner(self) -> Vec<String> {
				self.0
			}
		}
	};
}
